use std::{sync::Arc, time::SystemTime};

use anyhow::{Error, Result};
use reqwest::Method;

use crate::{
    models::{ApiRequest, Credentials, Endpoint, ResponseWrapper},
    utilities::{get_signature, get_timestamp},
};

const BASE_URL: &str = "https://api.bybit.com";

pub(crate) struct Api {
    credentials: Option<Arc<Credentials>>,
    recv_window: String,
}

impl Api {
    pub(crate) fn new(credentials: Option<Arc<Credentials>>) -> Self {
        dotenv::dotenv().ok();
        let recv_window = std::env::var("BYBIT_RECV_WINDOW").unwrap_or("5000".to_string());
        Self {
            credentials,
            recv_window,
        }
    }

    pub(crate) async fn request<Req>(&self, request: Req) -> Result<Req::ApiResponse>
    where
        Req: ApiRequest,
        Req::ApiResponse: Default,
    {
        let endpoint: Endpoint = Req::endpoint();

        let client = reqwest::Client::new();

        let params_map = request.to_query_map()?;
        let params_str = request.to_query_string()?;

        let mut request_builder = match endpoint.method {
            Method::POST => client
                .post(format!("{}{}", BASE_URL, endpoint.path))
                .json(&params_map),
            Method::GET => client.get(format!("{}{}?{}", BASE_URL, endpoint.path, params_str)),
            _ => unreachable!("Unsupported HTTP method"),
        };

        if endpoint.auth
            && let Some(credentials) = &self.credentials
        {
            let timestamp = String::from(format!("{}", get_timestamp(SystemTime::now())));
            let signature = self.get_signed_query_string(
                credentials,
                &timestamp,
                match endpoint.method {
                    Method::POST => serde_json::to_string(&params_map)?,
                    Method::GET => params_str,
                    _ => unreachable!("Unsupported HTTP method"),
                }
                .as_str(),
            );
            request_builder = request_builder
                .header("X-BAPI-API-KEY", credentials.api_key())
                .header("X-BAPI-TIMESTAMP", timestamp)
                .header("X-BAPI-SIGN", signature)
                .header("X-BAPI-RECV-WINDOW", self.recv_window.clone());
        }

        let response = request_builder.send().await?;
        let response_text = response.text().await?;
        let response: ResponseWrapper<Req::ApiResponse> = serde_json::from_str(&response_text)?;

        match response.ret_code {
            0 => {
                if let Some(result) = response.result {
                    Ok(result)
                } else {
                    Ok(Req::ApiResponse::default())
                }
            }
            _ => Err(Error::msg(format!(
                "[{}][{}]{}",
                response.time.unwrap_or(0),
                response.ret_code,
                response.ret_msg
            ))),
        }
    }

    fn get_signed_query_string(
        &self,
        credentials: &Credentials,
        timestamp: &str,
        params: &str,
    ) -> String {
        get_signature(
            credentials.api_secret(),
            vec![
                timestamp,
                credentials.api_key(),
                self.recv_window.as_str(),
                params,
            ],
        )
    }
}
