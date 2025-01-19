use leptos::{prelude::*, task::spawn_local};

#[server]
pub async fn test() -> Result<String, ServerFnError> {
    Ok("NOOO".to_string()) 
}

/// Renders the home page of your application.
#[component]
pub fn Comp() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <button on:click=move |_| {
            spawn_local(async {
                test().await;
            });
        }>
            "TEST"
        </button>
    }
}
