use yew::prelude::*;

#[function_component]
pub fn NotFoundPage() -> Html {
    html! {
        <div class="container text-center mt-5">
            <h1>{"404"}</h1>
            <p class="lead">{"Page not found"}</p>
        </div>
    }
}