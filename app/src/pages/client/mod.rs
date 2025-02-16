pub mod main;
pub mod news;
pub mod contacts;

use leptos::prelude::*;
use leptos_router::components::Outlet;
use crate::components::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <>
            <navbar::Comp/>
            <Outlet/>
            <footer::Comp/>
        </>
    }
}