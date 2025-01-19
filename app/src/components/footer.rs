use leptos::prelude::*;
use leptos_router::components::*;

#[component]
pub fn Comp() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white py-12">
            <div class="container mx-auto px-4">
                <div class="grid md:grid-cols-3 gap-8">
                    <div>
                        <h5 class="text-lg font-bold mb-4">"Контакти"</h5>
                        <ul class="space-y-2">
                            <li><i class="fas fa-phone mr-2"></i>"(000) 123-45-67"</li>
                            <li><i class="fas fa-envelope mr-2"></i>"info@water.com"</li>
                            <li><i class="fas fa-map-marker-alt mr-2"></i>"вул. Водна, 1, м. Місто"</li>
                        </ul>
                    </div>
                    <div class="links">
                        <h5 class="text-lg font-bold mb-4">"Швидкі посилання"</h5>
                        <ul class="space-y-2">
                            <li><A href="/main">"Головна"</A></li>
                            <li><A href="/news">"Новини"</A></li>
                            <li><A href="/contacts">"Контакти"</A></li>
                        </ul>
                    </div>
                    <div>
                        <h5 class="text-lg font-bold mb-4">"Графік роботи"</h5>
                        <ul class="space-y-2">
                            <li>"Пн-Пт: 8:00 - 17:00"</li>
                            <li>"Сб: 9:00 - 13:00"</li>
                            <li>"Нд: Вихідний"</li>
                        </ul>
                    </div>
                </div>
                <hr class="my-8 border-gray-700" />
                <div class="text-center">
                    <p>"&copy; 2025 КП 'Водопостачання та водовідведення'. Всі права захищені."</p>
                </div>
            </div>
        </footer>
    }
}
