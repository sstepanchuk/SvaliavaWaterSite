mod app;
pub mod pages;
pub mod components;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Script, Stylesheet};

pub use app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Script src="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/js/all.min.js" />
                <Script src="https://cdn.tailwindcss.com" />
                <Stylesheet href="/pkg/svaliava-water-site.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
