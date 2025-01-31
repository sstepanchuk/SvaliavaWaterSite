#![allow(non_snake_case)]

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
                <Script src="https://cdn.tailwindcss.com"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone()/>
                <Stylesheet href=format!("/pkg/{}.css", options.output_name) />

                <Script src="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/js/all.min.js" defer="" async_="" />
                <MetaTags/>
            </head>
            <body class="h-full flex flex-col">
                <App/>
            </body>
        </html>
    }
}
