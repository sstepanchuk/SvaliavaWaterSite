use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_image::provide_image_context;
use leptos_meta::Script;
use leptos_meta::Style;
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
            <Style>
                "#loader {
                    position: fixed;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: white;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    z-index: 9999;
                    opacity: 1;
                    visibility: visible;
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
                    box-shadow: inset 0 0 30px rgba(0,0,0,0.2);
                }
                .wave:before,.wave:after{
                    content: '';
                    position: absolute;
                    width: 200%;
                    height: 200%;
                    top: 0;
                    left: 50%;
                    transform: translate(-50%,-75%);
                }
                .wave:before{
                    border-radius: 45%;
                    background: rgba(255,255,255,.8);
                    animation: animate 5s linear infinite;
                }
                .wave:after{
                    border-radius: 40%;
                    background: rgba(255,255,255,.3);
                    animation: animate 10s linear infinite;
                }
                @keyframes animate{
                    0%{
                        transform: translate(-50%,-75%) rotate(0deg);
                    }
                    100%{
                        transform: translate(-50%,-75%) rotate(360deg);
                    }
                }"
            </Style>

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

            <Script>
                "window.addEventListener('load', function() {
                    document.getElementById('loader').classList.add('hidden');
                    setTimeout(() => {
                        document.getElementById('loader').style.display = 'none';
                    }, 500);
                })"
            </Script>
        </>
    }
}