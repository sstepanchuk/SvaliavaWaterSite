use yew::prelude::*;
use crate::components::auth::RegisterForm;
use shared::models::CreateUserRequest;

#[function_component]
pub fn RegisterPage() -> Html {
    let onsubmit = Callback::from(|request: CreateUserRequest| {
        // Реалізація реєстрації
    });

    html! {
        <div class="container">
            <div class="row justify-content-center">
                <div class="col-md-6">
                    <div class="card mt-5">
                        <div class="card-body">
                            <h2 class="card-title text-center mb-4">{"Register"}</h2>
                            <RegisterForm {onsubmit} />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}