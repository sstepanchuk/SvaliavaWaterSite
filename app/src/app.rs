use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_image::provide_image_context;
use leptos_router::components::*;
use leptos_router::path;
use crate::components::*;
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_image_context();

    view! {
        <>
            <div id="loader">
                <div class="circle">
                    <div class="wave"></div>
                </div>
            </div>

            <Router>
                <navbar::Comp/>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/main") view=main::Comp/>
                    <Route path=path!("/news") view=news::Comp/>
                    <Route path=path!("/contacts") view=contacts::Comp/>
                    <Route path=path!("/") view=|| view! { <Redirect path="/main"/> }/>
                </Routes>
                <footer::Comp/>
            </Router>
        </>
    }
}

/*



*/