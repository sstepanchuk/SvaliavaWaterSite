use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::hooks::use_auth;

#[function_component]
pub fn NavBar() -> Html {
    let auth = use_auth();
    
    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>
                    {"Rust App"}
                </Link<Route>>
                
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
                    <span class="navbar-toggler-icon"></span>
                </button>
                
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav ms-auto">
                        if auth.token.is_some() {
                            <li class="nav-item">
                                <button 
                                    class="nav-link btn btn-link" 
                                    onclick={auth.logout.reform(|_| ())}
                                >
                                    {"Logout"}
                                </button>
                            </li>
                        } else {
                            <li class="nav-item">
                                <Link<Route> classes={classes!("nav-link")} to={Route::Login}>
                                    {"Login"}
                                </Link<Route>>
                            </li>
                        }
                    </ul>
                </div>
            </div>
        </nav>
    }
}