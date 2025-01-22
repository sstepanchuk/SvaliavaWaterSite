use leptos::prelude::*;
use crate::components::hero;
use leptos_meta::Title;

#[component]
pub fn Comp() -> impl IntoView {
    view! {
      <>
        <Title text="КП \"Водопостачання та водовідведення\""/>

        <hero::Comp background_url="/images/main.jpeg".to_string()>
            <h1 class="text-3xl md:text-4xl font-bold text-center">"Контакти"</h1>
            <p class="text-center text-lg mt-4">"Зв'яжіться з нами будь-яким зручним способом"</p>
        </hero::Comp>
          
        <main class="container mx-auto px-4 py-12">
          <div class="grid md:grid-cols-3 gap-8 mb-12">
              <div class="bg-white rounded-lg shadow-lg p-8 text-center transform transition duration-300 hover:-translate-y-1 hover:shadow-xl">
                  <div class="text-blue-600 text-4xl mb-4">
                      <i class="fas fa-phone-alt"></i>
                  </div>
                  <h3 class="text-xl font-bold mb-4">"Телефони"</h3>
                  <div class="space-y-2 text-gray-600">
                      <p>"Диспетчерська служба:"<br/>"(000) 123-45-67"</p>
                      <p>"Приймальня:"<br/>"(000) 123-45-68"</p>
                  </div>
              </div>
              <div class="bg-white rounded-lg shadow-lg p-8 text-center transform transition duration-300 hover:-translate-y-1 hover:shadow-xl">
                  <div class="text-blue-600 text-4xl mb-4">
                      <i class="fas fa-envelope"></i>
                  </div>
                  <h3 class="text-xl font-bold mb-4">"Email"</h3>
                  <div class="space-y-2 text-gray-600">
                      <p>"Загальні питання:"<br/>"info@water.com"</p>
                      <p>"Технічна підтримка:"<br/>"support@water.com"</p>
                  </div>
              </div>
              <div class="bg-white rounded-lg shadow-lg p-8 text-center transform transition duration-300 hover:-translate-y-1 hover:shadow-xl">
                  <div class="text-blue-600 text-4xl mb-4">
                      <i class="fas fa-clock"></i>
                  </div>
                  <h3 class="text-xl font-bold mb-4">"Графік роботи"</h3>
                  <div class="space-y-2 text-gray-600">
                      <p>"Пн-Пт: 8:00 - 17:00"</p>
                      <p>"Сб: 9:00 - 13:00"</p>
                      <p>"Нд: Вихідний"</p>
                      <p>"Диспетчер: цілодобово"</p>
                  </div>
              </div>
          </div>

          <div class="grid lg:grid-cols-2 gap-8">
              <div class="bg-white rounded-lg shadow-lg overflow-hidden">
                  <h3 class="text-xl font-bold p-6 bg-gray-50 border-b">"Наше місцезнаходження"</h3>
                  <div class="aspect-w-16 aspect-h-9">
                      <iframe
                          src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2540.0667265302024!2d30.519999999999996!3d50.450000000000006!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x0%3A0x0!2zNDDCsDI3JzAyLjQiTiAzMMKwMzEnMTIuMCJF!5e0!3m2!1suk!2sua!4v1234567890"
                          class="w-full h-[400px]"
                          style="border:0;"
                          allowfullscreen=""
                          loading="lazy"
                      ></iframe>
                  </div>
                  <div class="p-6">
                      <h4 class="font-bold text-lg mb-2 flex items-center">
                          <i class="fas fa-map-marker-alt text-blue-600 mr-2"></i>
                          "Адреса"
                      </h4>
                      <p class="text-gray-600">"вул. Водна, 1, м. Місто, 12345"</p>
                  </div>
              </div>
              <div class="bg-white rounded-lg shadow-lg">
                  <h3 class="text-xl font-bold p-6 bg-gray-50 border-b">"Форма зворотного зв'язку"</h3>
                  <form class="p-6 space-y-4">
                      <div>
                          <label for="name" class="block text-sm font-medium text-gray-700 mb-1">"Ваше ім'я"</label>
                          <input
                              type="text"
                              id="name"
                              class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                              required
                          />
                      </div>
                      <div>
                          <label for="email" class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                          <input
                              type="email"
                              id="email"
                              class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                              required
                          />
                      </div>
                      <div>
                          <label for="phone" class="block text-sm font-medium text-gray-700 mb-1">"Телефон"</label>
                          <input
                              type="tel"
                              id="phone"
                              class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                          />
                      </div>
                      <div>
                          <label for="subject" class="block text-sm font-medium text-gray-700 mb-1">"Тема"</label>
                          <input
                              type="text"
                              id="subject"
                              class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                              required
                          />
                      </div>
                      <div>
                          <label for="message" class="block text-sm font-medium text-gray-700 mb-1">"Повідомлення"</label>
                          <textarea
                              id="message"
                              rows="5"
                              class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                              required
                          ></textarea>
                      </div>
                      <button
                          type="submit"
                          class="w-full bg-blue-600 text-white px-6 py-3 rounded-md hover:bg-blue-700 transform transition duration-300 hover:-translate-y-0.5 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                      >
                          "Надіслати повідомлення"
                      </button>
                  </form>
              </div>
          </div>

          <div class="mt-12">
              <div class="bg-white rounded-lg shadow-lg p-6">
                  <div class="grid md:grid-cols-2 gap-8">
                      <div>
                          <h4 class="text-lg font-bold mb-4 flex items-center text-blue-600">
                              <i class="fas fa-exclamation-triangle mr-2"></i>
                              "Аварійна служба"
                          </h4>
                          <p class="text-gray-600 mb-2">"У разі виникнення аварійних ситуацій звертайтеся за номером: (000) 123-45-69"</p>
                          <p class="text-gray-600">"Аварійна служба працює цілодобово без вихідних"</p>
                      </div>
                      <div>
                          <h4 class="text-lg font-bold mb-4 flex items-center text-blue-600">
                              <i class="fas fa-info-circle mr-2"></i>
                              "Показники лічильника"
                          </h4>
                          <p class="text-gray-600 mb-2">"Передати показники лічильника можна:"</p>
                          <ul class="list-disc list-inside text-gray-600 space-y-1">
                              <li>"Через особистий кабінет на сайті"</li>
                              <li>"За телефоном: (000) 123-45-70"</li>
                              <li>"Через мобільний додаток"</li>
                          </ul>
                      </div>
                  </div>
              </div>
          </div>
        </main>
      </>
    }
}
