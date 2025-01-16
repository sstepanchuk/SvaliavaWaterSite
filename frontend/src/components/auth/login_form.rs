use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use shared::models::LoginRequest;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<LoginRequest>,
}

#[function_component]
pub fn LoginForm(props: &Props) -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);

    let onsubmit = {
        let onsubmit = props.onsubmit.clone();
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let request = LoginRequest {
                username: (*username).clone(),
                password: (*password).clone(),
            };
            onsubmit.emit(request);
        })
    };

    let oninput_username = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            username.set(value);
        })
    };

    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            password.set(value);
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
                    value={(*username).clone()}
                    oninput={oninput_username}
                    required=true
                />
            </div>
            <div class="mb-3">
                <label for="password" class="form-label">{"Password"}</label>
                <input
                    type="password"
                    class="form-control"
                    id="password"
                    value={(*password).clone()}
                    oninput={oninput_password}
                    required=true
                />
            </div>
            <button type="submit" class="btn btn-primary w-100">{"Login"}</button>
        </form>
    }
}