#![allow(non_snake_case)]

mod app;
pub mod shared;
pub mod pages;
pub mod components;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Script, Style, Stylesheet};

pub use app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Script id="tailwind" src="https://cdn.tailwindcss.com" defer="" async_=""/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone()/>
                <Stylesheet id="leptos" href=format!("/pkg/{}.css", options.output_name) />
                <Script id="font-awesome" src="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/js/all.min.js" defer="" async_="" />
                
                <Style id="loader-css">
                    "body.loading {
                        overflow: hidden;
                        height: 100%;
                    }

                    #loader {
                        position: fixed;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        background: white;
                        display: -webkit-box;
                        display: -ms-flexbox;
                        display: flex;
                        -webkit-box-pack: center;
                            -ms-flex-pack: center;
                                justify-content: center;
                        -webkit-box-align: center;
                            -ms-flex-align: center;
                                align-items: center;
                        z-index: 9999;
                        opacity: 1;
                        visibility: visible;
                        -webkit-transition: opacity 0.5s ease-in-out, visibility 0.5s ease-in-out;
                        -o-transition: opacity 0.5s ease-in-out, visibility 0.5s ease-in-out;
                        transition: opacity 0.5s ease-in-out, visibility 0.5s ease-in-out;
                    }

                    #loader.hidden {
                        opacity: 0;
                        visibility: hidden;
                    }

                    .circle{
                        position: relative;
                        width: 150px;
                        height: 150px;
                        background: #f5f5f5;
                        border: 2px solid #e0e0e0;
                        border-radius: 50%;
                        overflow: hidden;
                    }
                    .wave{
                        position: relative;
                        width: 100%;
                        height: 100%;
                        background: #2196F3;
                        border-radius: 50%;
                        -webkit-box-shadow: inset 0 0 30px rgba(0,0,0,0.2);
                                box-shadow: inset 0 0 30px rgba(0,0,0,0.2);
                    }
                    .wave:before,.wave:after{
                        content: '';
                        position: absolute;
                        width: 200%;
                        height: 200%;
                        top: 0;
                        left: 50%;
                        -webkit-transform: translate(-50%,-75%);
                            -ms-transform: translate(-50%,-75%);
                                transform: translate(-50%,-75%);
                    }
                    .wave:before{
                        border-radius: 45%;
                        background: rgba(255,255,255,.8);
                        -webkit-animation: animate 5s linear infinite;
                                animation: animate 5s linear infinite;
                    }
                    .wave:after{
                        border-radius: 40%;
                        background: rgba(255,255,255,.3);
                        -webkit-animation: animate 10s linear infinite;
                                animation: animate 10s linear infinite;
                    }
                    @-webkit-keyframes animate{
                        0%{
                            -webkit-transform: translate(-50%,-75%) rotate(0deg);
                                    transform: translate(-50%,-75%) rotate(0deg);
                        }
                        100%{
                            -webkit-transform: translate(-50%,-75%) rotate(360deg);
                                    transform: translate(-50%,-75%) rotate(360deg);
                        }
                    }
                    @keyframes animate{
                        0%{
                            -webkit-transform: translate(-50%,-75%) rotate(0deg);
                                    transform: translate(-50%,-75%) rotate(0deg);
                        }
                        100%{
                            -webkit-transform: translate(-50%,-75%) rotate(360deg);
                                    transform: translate(-50%,-75%) rotate(360deg);
                        }
                    }"
                </Style>
                <Script id="close-loader">
                    "window.addEventListener('load', function() {
                        document.body.classList.remove('loading');
                        document.getElementById('loader').classList.add('hidden');
                        setTimeout(() => {
                            document.getElementById('loader').style.display = 'none';
                        }, 500);
                    })"
                </Script>
                <MetaTags/>
            </head>
            <body class="h-full flex flex-col loading">
                <App/>
            </body>
        </html>
    }
}
