use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home::HomePage,
    auth::{LoginPage, RegisterPage},
    not_found::NotFoundPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Register => html! { <RegisterPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}