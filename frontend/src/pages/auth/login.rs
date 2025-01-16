use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use shared::models::LoginRequest;
use crate::hooks::use_auth;
use crate::components::auth::LoginForm;

#[function_component]
pub fn LoginPage() -> Html {
    let auth = use_auth();

    let onsubmit = {
        let auth = auth.clone();
        Callback::from(move |request: LoginRequest| {
            auth.login.emit(request);
        })
    };

    html! {
        <div class="container">
            <div class="row justify-content-center">
                <div class="col-md-6">
                    <div class="card mt-5">
                        <div class="card-body">
                            <h2 class="card-title text-center mb-4">{"Login"}</h2>
                            <LoginForm {onsubmit} />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}