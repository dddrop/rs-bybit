use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures_util::{SinkExt, StreamExt, lock::Mutex};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::{sync::mpsc, time::interval};
use tokio_tungstenite::{connect_async, tungstenite};

use crate::models::{
    AuthOperation, Command, CommandType, Credentials, Message, SubscribeArgs, TopicType,
    UnsubscribeArgs,
};

const PRIVATE_URL: &str = "wss://stream.bybit.com/v5/private";

pub(crate) struct WebSocket {
    credentials: Option<Arc<Credentials>>,
    pub rx: Arc<Mutex<mpsc::Receiver<Message>>>,
    pub tx: Arc<Mutex<mpsc::Sender<Command>>>,
}

impl WebSocket {
    pub async fn new(credentials: Option<Arc<Credentials>>) -> Self {
        match connect_async(PRIVATE_URL).await {
            Ok((stream, _response)) => {
                #[cfg(feature = "logging")]
                {
                    tracing::info!("[Bybit][WS] Connected successfully: {:?}", _response);
                }

                let (mut sink, mut stream) = stream.split();

                let (send_tx, mut send_rx) = mpsc::channel::<Command>(100);
                let (recv_tx, recv_rx) = mpsc::channel::<Message>(100);

                tokio::spawn(async move {
                    while let Some(msg) = stream.next().await {
                        match msg {
                            Ok(msg) => match msg.clone() {
                                tungstenite::Message::Text(text) => {
                                    match serde_json::from_str::<Message>(&text.to_string()) {
                                        Ok(msg) => {
                                            if let Err(_e) = recv_tx.send(msg).await {
                                                #[cfg(feature = "logging")]
                                                {
                                                    tracing::error!(
                                                        "[Bybit][WS] Failed to send to recv channel: {:?}",
                                                        _e
                                                    );
                                                }
                                                break;
                                            }
                                        }
                                        Err(_e) => {
                                            #[cfg(feature = "logging")]
                                            {
                                                tracing::error!(
                                                    "[Bybit][WS] Failed to deserialize WsMessage: {:?} (raw: {:?})",
                                                    _e,
                                                    msg.to_string(),
                                                );
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            },
                            Err(_e) => {
                                #[cfg(feature = "logging")]
                                {
                                    tracing::error!("[Bybit][WS] Read failed: {:?}", _e);
                                }
                                break;
                            }
                        }
                    }
                });

                tokio::spawn(async move {
                    while let Some(msg) = send_rx.recv().await {
                        let encoded_msg = serde_json::to_string(&msg).unwrap();
                        if let Err(_e) = sink
                            .send(tungstenite::Message::Text(encoded_msg.into()))
                            .await
                        {
                            #[cfg(feature = "logging")]
                            {
                                tracing::error!("[Bybit][WS] Send failed: {:?}", _e);
                            }
                            break;
                        }
                    }
                });

                Self {
                    credentials,
                    rx: Arc::new(Mutex::new(recv_rx)),
                    tx: Arc::new(Mutex::new(send_tx)),
                }
            }
            Err(_e) => {
                #[cfg(feature = "logging")]
                {
                    tracing::error!("[Bybit][WS] Connection failed: {:?}", _e);
                }
                panic!();
            }
        }
    }

    pub async fn send(&self, msg: Command) {
        if let Err(_e) = self.tx.lock().await.send(msg).await {
            #[cfg(feature = "logging")]
            {
                tracing::error!("[Bybit][WS] Send failed: {:?}", _e);
            }
        }
    }

    pub async fn auth(&self) {
        if let Some(credentials) = &self.credentials {
            let expires = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                + 10000;
            let message = format!("GET/realtime{}", expires);

            let mut mac = Hmac::<Sha256>::new_from_slice(credentials.api_secret().as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(message.as_bytes());
            let signature = hex::encode(mac.finalize().into_bytes());

            let auth_ops = AuthOperation {
                key: credentials.api_key().to_string(),
                timestamp: expires,
                signature,
            };

            self.send(Command {
                conn_id: None,
                command_type: CommandType::Auth(auth_ops.into()),
            })
            .await
        }
    }

    // pub async fn ping(&self) {
    //     self.send(Command {
    //         conn_id: None,
    //         command_type: CommandType::Ping,
    //     })
    //     .await
    // }

    pub async fn subscribe(&self, topics: Vec<TopicType>) {
        self.send(Command {
            conn_id: None,
            command_type: CommandType::Subscribe(SubscribeArgs { topics }.into()),
        })
        .await
    }

    pub async fn unsubscribe(&self, topics: Vec<TopicType>) {
        self.send(Command {
            conn_id: None,
            command_type: CommandType::Unsubscribe(UnsubscribeArgs { topics }.into()),
        })
        .await
    }

    // pub async fn close(self) {
    //     drop(self.tx);
    //     // Wait a short time to allow pending messages to be processed
    //     tokio::time::sleep(Duration::from_millis(100)).await;
    // }

    pub fn keep_alive(&self) {
        let ws_ping = self.tx.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(20));
            loop {
                interval.tick().await;
                if let Err(_e) = ws_ping
                    .lock()
                    .await
                    .send(Command {
                        conn_id: None,
                        command_type: CommandType::Ping,
                    })
                    .await
                {
                    #[cfg(feature = "logging")]
                    {
                        tracing::error!("[ERR-PING][WS] Ping failed: {:?}", _e);
                    }
                    break;
                }
            }
        });
    }
}
