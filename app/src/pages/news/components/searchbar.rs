use leptos::prelude::*;

#[component]
pub fn Comp() -> impl IntoView {
    view! {
      <div class="bg-white shadow-md">
        <div class="container mx-auto px-4 py-4">
          <div class="max-w-2xl mx-auto">
            <div class="flex items-center space-x-4">
              <div class="flex-1">
                <div class="relative">
                  <input 
                    type="text" 
                    placeholder="Пошук новин..." 
                    class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-300"
                  />
                  <div class="absolute left-3 top-2.5 text-gray-400">
                    <i class="fas fa-search"></i>
                  </div>
                </div>
              </div>
              <button class="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-all duration-300">
                "Пошук"
              </button>
            </div>
          </div>
        </div>
      </div>
    }
}