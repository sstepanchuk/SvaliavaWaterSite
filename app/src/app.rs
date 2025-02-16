use std::time::Duration;

use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_image::provide_image_context;
use leptos_router::components::*;
use leptos_router::path;
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_image_context();

    let (is_routing, set_is_routing) = signal(false);

    view! {
        <>
            <div class=move || {
                if is_routing.get() {
                    "routing-progress show"
                } else {
                    "routing-progress show"
                }
            }>
                <RoutingProgress is_routing max_time=Duration::from_millis(250) />
            </div>
            <div id="loader">
                <div class="circle">
                    <div class="wave"></div>
                </div>
            </div>

            <Router set_is_routing>
                <Routes fallback=|| "Page not found.".into_view()>       
                    <ParentRoute path=path!("/") view=client::Layout>
                        <Route path=path!("main") view=client::main::Comp />
                        <Route path=path!("news") view=client::news::Comp />
                        <Route path=path!("contacts") view=client::contacts::Comp />
                    </ParentRoute>

                    <ParentRoute path=path!("/admin") view=client::Layout>
                        <Route path=path!("login") view=client::main::Comp />
                        
                        <Route path=path!("") view=|| view! { <Redirect path="login"/> } />
                    </ParentRoute>

                    <Route path=path!("/") view=|| view! { <Redirect path="/main"/> } />
                </Routes>
            </Router>
        </>
    }
}
