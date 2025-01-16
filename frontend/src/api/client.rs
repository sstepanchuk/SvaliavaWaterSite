use shared::{routes::ApiEndpoint, error::ApiError};
use gloo::net::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use crate::config::API_URL;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    auth_token: Option<String>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            base_url: API_URL.to_string(),
            auth_token: None,
        }
    }

    pub fn with_auth(token: String) -> Self {
        Self {
            base_url: API_URL.to_string(),
            auth_token: Some(token),
        }
    }

    pub async fn request<E: ApiEndpoint>(
        &self,
        params: E::Request,
    ) -> Result<E::Response, ApiError>
    where
        E::Request: Serialize,
        E::Response: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, E::PATH);
        let mut request = Request::new(&url).method(E::METHOD);

        if let Some(token) = &self.auth_token {
            request = request.header("Authorization", &format!("Bearer {}", token));
        }

        let response = request
            .json(&params)?
            .send()
            .await?;

        if response.ok() {
            Ok(response.json().await?)
        } else {
            Err(ApiError::RequestFailed(response.status().to_string()))
        }
    }
}