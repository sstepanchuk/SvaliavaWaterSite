use crate::models::{LoginRequest, ApiResponse, AuthResponse};
use api_endpoint_core::{ApiEndpoint, AuthorizedApiEndpoint};
use api_endpoint_derive::api_endpoint;
use serde::{Deserialize, Serialize};

#[api_endpoint(
    "/auth/login",
    POST,
    LoginRequest,
    ApiResponse<AuthResponse>
)]
#[derive(Serialize, Deserialize)]
pub struct LoginRoute;

#[api_endpoint(
    "/auth/refresh",
    POST,
    (),
    ApiResponse<AuthResponse>,
    authorized
)]
#[derive(Serialize, Deserialize)]
pub struct RefreshTokenRoute;