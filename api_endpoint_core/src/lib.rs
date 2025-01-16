use serde::{Serialize, de::DeserializeOwned};

pub trait ApiEndpoint {
    type Request: Serialize + DeserializeOwned + Send;
    type Response: Serialize + DeserializeOwned + Send;
    type Params: Serialize + DeserializeOwned + Send;
    
    const METHOD: &'static str;
    const PATH: &'static str;
}

pub trait AuthorizedApiEndpoint: ApiEndpoint {}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Request failed: {0}")]
    RequestFailed(String),
}