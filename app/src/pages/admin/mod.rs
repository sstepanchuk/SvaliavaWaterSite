use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <>
            <Outlet/>
        </>
    }
}