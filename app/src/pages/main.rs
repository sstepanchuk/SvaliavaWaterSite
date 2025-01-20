use leptos::prelude::*;

#[component]
pub fn Comp() -> impl IntoView {
    view! {
        <div>
            <div class="relative bg-gray-900 text-white py-24">
                <div class="absolute inset-0">
                    <img src="/images/main.jpeg" alt="Background" class="w-full h-full object-cover opacity-50" />
                </div>
                <div class="relative container mx-auto px-4 text-center">
                    <h1 class="text-4xl md:text-5xl font-bold mb-4">"КП \"Водопостачання та водовідведення\""</h1>
                    <p class="text-xl md:text-2xl">"Забезпечуємо якісне водопостачання та водовідведення для комфорту наших клієнтів"</p>
                </div>
            </div>

            <main class="flex-grow">
                <section class="py-16 bg-white">
                    <div class="container mx-auto px-4">
                        <h2 class="text-3xl font-bold text-center mb-12">"Про нашу компанію"</h2>
                        <div class="grid md:grid-cols-2 gap-8 items-center">
                            <div class="space-y-4">
                                <p class="text-gray-600 text-lg">
                                    "КП \"Водопостачання та водовідведення\" - це сучасне комунальне підприємство, 
                                    яке забезпечує населення та підприємства якісними послугами водопостачання та водовідведення. 
                                    Наша місія - надання надійних та екологічно безпечних послуг для покращення якості життя громади."
                                </p>
                            </div>
                            <div>
                                <img src="/images/mock_main/image.png" alt="Про компанію" class="rounded-lg shadow-lg w-full" />
                            </div>
                        </div>
                    </div>
                </section>

                <section class="py-16 bg-gray-50">
                    <div class="container mx-auto px-4">
                        <h2 class="text-3xl font-bold text-center mb-12">"Наші послуги"</h2>
                        <div class="grid md:grid-cols-3 gap-8">
                            <div class="bg-white rounded-lg shadow-lg p-6 transition duration-300 hover:-translate-y-1">
                                <div class="text-blue-600 text-4xl mb-4 text-center">
                                    <i class="fas fa-tint"></i>
                                </div>
                                <h3 class="text-xl font-bold text-center mb-4">"Водопостачання"</h3>
                                <p class="text-gray-600 text-center">"Забезпечуємо безперебійне постачання якісної питної води"</p>
                            </div>

                            <div class="bg-white rounded-lg shadow-lg p-6 transition duration-300 hover:-translate-y-1">
                                <div class="text-blue-600 text-4xl mb-4 text-center">
                                    <i class="fas fa-water"></i>
                                </div>
                                <h3 class="text-xl font-bold text-center mb-4">"Водовідведення"</h3>
                                <p class="text-gray-600 text-center">"Надаємо послуги з відведення та очищення стічних вод"</p>
                            </div>

                            <div class="bg-white rounded-lg shadow-lg p-6 transition duration-300 hover:-translate-y-1">
                                <div class="text-blue-600 text-4xl mb-4 text-center">
                                    <i class="fas fa-wrench"></i>
                                </div>
                                <h3 class="text-xl font-bold text-center mb-4">"Обслуговування"</h3>
                                <p class="text-gray-600 text-center">"Проводимо ремонт та обслуговування мереж водопостачання"</p>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="py-16 bg-white">
                    <div class="container mx-auto px-4">
                        <h2 class="text-3xl font-bold text-center mb-12">"Останні новини"</h2>
                        <div class="grid md:grid-cols-3 gap-8">
                            <div class="bg-white rounded-lg shadow-lg overflow-hidden transition duration-300 hover:-translate-y-1">
                                <img src="/images/mock_main/news/1.png" alt="Новина 1" class="w-full h-48 object-cover" />
                                <div class="p-6">
                                    <h3 class="text-xl font-bold mb-2">"Планові ремонтні роботи"</h3>
                                    <p class="text-gray-600 mb-4">"У зв'язку з плановими ремонтними роботами можливе тимчасове припинення водопостачання..."</p>
                                    <a href="news.html" class="inline-block bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition">"Читати далі"</a>
                                </div>
                            </div>

                            <div class="bg-white rounded-lg shadow-lg overflow-hidden transition duration-300 hover:-translate-y-1">
                                <img src="/images/mock_main/news/2.png" alt="Новина 2" class="w-full h-48 object-cover" />
                                <div class="p-6">
                                    <h3 class="text-xl font-bold mb-2">"Модернізація обладнання"</h3>
                                    <p class="text-gray-600 mb-4">"Встановлено нове сучасне обладнання для покращення якості води..."</p>
                                    <a href="news.html" class="inline-block bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition">"Читати далі"</a>
                                </div>
                            </div>

                            <div class="bg-white rounded-lg shadow-lg overflow-hidden transition duration-300 hover:-translate-y-1">
                                <img src="/images/mock_main/news/3.png" alt="Новина 3" class="w-full h-48 object-cover" />
                                <div class="p-6">
                                    <h3 class="text-xl font-bold mb-2">"Акція для споживачів"</h3>
                                    <p class="text-gray-600 mb-4">"Знижки на встановлення лічильників води для пенсіонерів..."</p>
                                    <a href="news.html" class="inline-block bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition">"Читати далі"</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </main>
        </div>
    }
}
