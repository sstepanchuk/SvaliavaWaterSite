use leptos::prelude::*;

#[component]
pub fn Comp() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <nav class="inline-flex rounded-md shadow">
                <a href="#" class="px-3 py-2 rounded-l-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50">"Попередня"</a>
                <a href="#" class="px-3 py-2 border-t border-b border-gray-300 bg-blue-600 text-white">"1"</a>
                <a href="#" class="px-3 py-2 border border-gray-300 bg-white text-gray-700 hover:bg-gray-50">"2"</a>
                <a href="#" class="px-3 py-2 border border-gray-300 bg-white text-gray-700 hover:bg-gray-50">"3"</a>
                <a href="#" class="px-3 py-2 rounded-r-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50">"Наступна"</a>
            </nav>
        </div>
    }
}