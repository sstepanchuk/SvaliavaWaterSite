use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos::logging;

#[component]
pub fn Comp() -> impl IntoView {
    // Signal to track if the mobile menu is open
    let (is_menu_open, set_is_menu_open) = signal(false);

    // Function to toggle the menu
    let toggle_menu = move |_| {
      set_is_menu_open.set(!is_menu_open.get());
    };

    let location = use_location();
    Effect::new(move |_| {
        let location = location.pathname.get();

        logging::log!("Value: {}", location);
        set_is_menu_open.set(false);
    });

    view! {
        <nav class="bg-blue-600 text-white">
            <div class="container mx-auto px-4">
                <div class="flex justify-between items-center py-4">
                    // Logo
                    <A href="/main">
                        <img src="/api/placeholder/200/50" alt="Логотип" class="h-12" />
                    </A>

                    // Мобільна кнопка меню
                    <button 
                        class="lg:hidden flex flex-col space-y-1.5" 
                        on:click=toggle_menu
                    >
                        <div 
                            class=move || {
                              format!(
                                "w-6 h-0.5 bg-white transition-transform duration-300 ease-in-out {}", 
                                if is_menu_open.get() { "rotate-45 translate-y-2" } else { "" }
                              )
                            }
                        ></div>
                        <div 
                          class=move || {
                            format!(
                              "w-6 h-0.5 bg-white transition-opacity duration-300 ease-in-out {}", 
                              if is_menu_open.get() { "opacity-0" } else { "" }
                            )
                          }
                        ></div>
                        <div 
                          class=move || {
                            format!(
                              "w-6 h-0.5 bg-white transition-transform duration-300 ease-in-out {}", 
                              if is_menu_open.get() { "-rotate-45 -translate-y-2" } else { "" }
                            )
                          }
                        ></div>
                    </button>

                    // Desktop menu
                    <div class="hidden lg:flex space-x-8 desktop">
                        <A href="/main">"Головна"</A> // selected
                        <A href="/news">"Новини"</A>
                        <A href="/contacts">"Контакти"</A>
                    </div>
                </div>

                // Mobile menu
                <div 
                  class=move || {
                    format!(
                      "lg:hidden overflow-hidden transition-all duration-300 ease-in-out {}", 
                      if is_menu_open.get() { "max-h-48 opacity-100" } else { "max-h-0 opacity-0" }
                    )
                  }
                >
                    <div class="flex flex-col space-y-4 py-4 mobile">
                        <A href="/main">"Головна"</A>
                        <A href="/news">"Новини"</A>
                        <A href="/contacts">"Контакти"</A>
                    </div>
                </div>
            </div>
        </nav>
    }
}
