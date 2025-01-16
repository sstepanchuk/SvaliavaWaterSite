use yew::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use shared::models::{User, LoginRequest, ApiResponse, AuthResponse};
use crate::api::{ApiClient, AuthApi};

const AUTH_TOKEN_KEY: &str = "auth_token";

#[hook]
pub fn use_auth() -> UseAuthHandle {
    let token = use_state(|| {
        LocalStorage::get(AUTH_TOKEN_KEY).ok()
    });
    let user = use_state(|| None);

    let login = {
        let token = token.clone();
        let user = user.clone();
        
        Callback::from(move |request: LoginRequest| {
            let token = token.clone();
            let user = user.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let client = ApiClient::new();
                if let Ok(response) = AuthApi::login(&client, request).await {
                    if let Some(auth) = response.data {
                        LocalStorage::set(AUTH_TOKEN_KEY, &auth.token).ok();
                        token.set(Some(auth.token));
                        // Тут можна додати запит на отримання даних користувача
                    }
                }
            });
        })
    };

    let logout = {
        let token = token.clone();
        let user = user.clone();
        
        Callback::from(move |_| {
            LocalStorage::delete(AUTH_TOKEN_KEY);
            token.set(None);
            user.set(None);
        })
    };

    UseAuthHandle {
        token: (*token).clone(),
        user: (*user).clone(),
        login,
        logout,
    }
}

pub struct UseAuthHandle {
    pub token: Option<String>,
    pub user: Option<User>,
    pub login: Callback<LoginRequest>,
    pub logout: Callback<()>,
}