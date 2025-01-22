use leptos::prelude::*;

#[component]
pub fn Comp(background_url: String, children: Children) -> impl IntoView {
    view! {
        <div class="relative bg-gray-900 text-white py-24">
            <div class="absolute inset-0">
                <img src={background_url} alt="Background" class="w-full h-full object-cover opacity-50" />
            </div>
            <div class="relative container mx-auto px-4 text-center">
                {children()}
            </div>
        </div>
    }
}