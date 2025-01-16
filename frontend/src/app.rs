use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Route, switch};
use crate::components::nav::NavBar;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="app">
                <NavBar />
                <main class="container mt-4">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}