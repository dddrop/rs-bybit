use rs_bybit::client::BybitWebSocketPublic;
use rs_bybit::models::{Message, TopicType};

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
            Message::Event(_) => {}
        }
    }
}
