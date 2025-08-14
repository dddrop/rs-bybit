use std::sync::Arc;

use anyhow::Result;
use futures_util::lock::Mutex;
use tokio::sync::mpsc;

use crate::{
    clients::{api::Api, ws::WebSocket},
    models::*,
};

// MARK: Private API

pub struct BybitApiPrivate {
    api: Api,
}

impl BybitApiPrivate {
    pub async fn new(credentials: Credentials) -> Self {
        let credentials: Arc<Credentials> = Arc::new(credentials);
        let api = Api::new(Some(credentials.clone()));
        Self { api }
    }
}

impl BybitApiPrivate {
    pub async fn set_leverage(&self, request: SetLeverageRequest) -> Result<SetLeverageResponse> {
        self.api.request(request).await
    }

    pub async fn place_order(&self, request: PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        self.api.request(request).await
    }

    pub async fn cancel_order(&self, request: CancelOrderRequest) -> Result<CancelOrderResponse> {
        self.api.request(request).await
    }

    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<CancelAllOrdersResponse> {
        self.api.request(request).await
    }

    pub async fn batch_cancel_orders(
        &self,
        request: BatchCancelOrdersRequest,
    ) -> Result<BatchCancelOrdersResponse> {
        self.api.request(request).await
    }

    pub async fn amend_order(&self, request: AmendOrderRequest) -> Result<AmendOrderResponse> {
        self.api.request(request).await
    }

    pub async fn get_open_and_closed_orders(
        &self,
        request: OpenAndClosedOrdersRequest,
    ) -> Result<OpenAndClosedOrdersResponse> {
        self.api.request(request).await
    }

    pub async fn get_position_info(
        &self,
        request: PositionInfoRequest,
    ) -> Result<PositionInfoResponse> {
        self.api.request(request).await
    }

    pub async fn get_trading_history(
        &self,
        request: TradingHistoryRequest,
    ) -> Result<TradingHistoryResponse> {
        self.api.request(request).await
    }

    pub async fn set_trading_stop(
        &self,
        request: TradingStopRequest,
    ) -> Result<TradingStopResponse> {
        self.api.request(request).await
    }

    pub async fn get_wallet_balance(
        &self,
        request: WalletBalanceRequest,
    ) -> Result<WalletBalanceResponse> {
        self.api.request(request).await
    }

    pub async fn inter_transfer(
        &self,
        request: InterTransferRequest,
    ) -> Result<InterTransferResponse> {
        self.api.request(request).await
    }
}

// MARK: Private WebSocket

pub struct BybitWebSocketPrivate {
    ws: WebSocket,
    pub rx: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl BybitWebSocketPrivate {
    pub async fn new(credentials: Credentials) -> Self {
        let credentials: Arc<Credentials> = Arc::new(credentials);
        let ws = WebSocket::new(Some(credentials.clone())).await;
        let rx = ws.rx.clone();

        ws.auth().await;
        ws.keep_alive();

        Self { ws, rx }
    }
}

impl BybitWebSocketPrivate {
    pub async fn ws_subscribe(&self, topics: Vec<TopicType>) {
        self.ws.subscribe(topics).await;
    }

    pub async fn ws_unsubscribe(&self, topics: Vec<TopicType>) {
        self.ws.unsubscribe(topics).await;
    }
}

// MARK: Public API

pub struct BybitApiPublic {
    api: Api,
}

impl BybitApiPublic {
    pub async fn new() -> Self {
        let api = Api::new(None);
        Self { api }
    }
}

impl BybitApiPublic {
    pub async fn get_server_time(&self) -> Result<ServerTimeResponse> {
        self.api.request(ServerTimeRequest::default()).await
    }

    pub async fn get_instruments_info(
        &self,
        request: InstrumentsInfoRequest,
    ) -> Result<InstrumentsInfoResponse> {
        self.api.request(request).await
    }

    pub async fn get_kline(&self, request: KlineRequest) -> Result<KlineResponse> {
        self.api.request(request).await
    }
}

// MARK: Public WebSocket

pub struct BybitWebSocketPublic {
    ws: WebSocket,
    pub rx: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl BybitWebSocketPublic {
    pub async fn new() -> Self {
        let ws = WebSocket::new(None).await;
        let rx = ws.rx.clone();
        ws.keep_alive();
        Self { ws, rx }
    }
}

impl BybitWebSocketPublic {
    pub async fn ws_subscribe(&self, topics: Vec<TopicType>) {
        self.ws.subscribe(topics).await;
    }

    pub async fn ws_unsubscribe(&self, topics: Vec<TopicType>) {
        self.ws.unsubscribe(topics).await;
    }
}
