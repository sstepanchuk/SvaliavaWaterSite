use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use shared::models::CreateUserRequest;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<CreateUserRequest>,
}

#[function_component]
pub fn RegisterForm(props: &Props) -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);

    let onsubmit = {
        let onsubmit = props.onsubmit.clone();
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = CreateUserRequest {
                username: (*username).clone(),
                email: (*email).clone(),
                password: (*password).clone(),
            };
            onsubmit.emit(request);
        })
    };

    html! {
        <form {onsubmit}>
            <div class="mb-3">
                <label for="username" class="form-label">{"Username"}</label>
                <input
                    type="text"
                    class="form-control"
                    id="username"
                    required=true
                />
            </div>
            <div class="mb-3">
                <label for="email" class="form-label">{"Email"}</label>
                <input
                    type="email"
                    class="form-control"
                    id="email"
                    required=true
                />
            </div>
            <div class="mb-3">
                <label for="password" class="form-label">{"Password"}</label>
                <input
                    type="password"
                    class="form-control"
                    id="password"
                    required=true
                />
            </div>
            <button type="submit" class="btn btn-primary w-100">{"Register"}</button>
        </form>
    }
}