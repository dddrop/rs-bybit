use rs_bybit::{
    client::BybitWebSocketPublic,
    models::{EventMessage, EventMessageData, Message, TopicType},
};

#[tokio::main]
async fn main() {
    let ws = BybitWebSocketPublic::new().await;

    ws.ws_subscribe(vec![TopicType::Kline(
        "1".to_string(),
        "XRPUSDT".to_string(),
    )])
    .await;

    while let Some(message) = ws.rx.lock().await.recv().await {
        println!("{:?}", message);
        match message {
            Message::Operation(o) => {}
            Message::Event(e) => match e.data {
                EventMessageData::Execution(_) => {}
                EventMessageData::FastExecution(_) => {}
                EventMessageData::Order(_) => {}
                EventMessageData::Position(_) => {}
                EventMessageData::Wallet(_) => {}
                EventMessageData::Kline(s, k) => {
                    println!("{}:{:?}", s, k.first().unwrap())
                }
            },
        }
    }
}
