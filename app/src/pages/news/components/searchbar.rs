use leptos::prelude::*;

#[component]
pub fn Comp() -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg p-6 mb-8">
            <h3 class="text-xl font-bold mb-4">"Пошук новин"</h3>
            <div class="flex">
                <input type="text" placeholder="Введіть текст..." 
                    class="flex-grow px-4 py-2 border border-gray-300 rounded-l focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"/>
                <button class="bg-blue-600 text-white px-4 py-2 rounded-r hover:bg-blue-700 transition duration-300">
                    <i class="fas fa-search"></i>
                </button>
            </div>
        </div>
    }
}