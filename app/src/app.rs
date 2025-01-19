use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::*;
use leptos_router::path;
use crate::components::*;
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <navbar::Comp/>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/main") view=main::Comp/>
                    <Route path=path!("/news") view=main::Comp/>
                    <Route path=path!("/contacts") view=main::Comp/>
                    <Route path=path!("/") view=|| view! { <Redirect path="/main"/> }/>
                </Routes>
            </main>
            <footer::Comp/>
        </Router>
    }
}