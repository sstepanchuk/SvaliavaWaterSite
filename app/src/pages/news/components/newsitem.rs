use leptos::prelude::*;

#[component]
pub fn Comp() -> impl IntoView {
  view! {
    <article class="bg-white rounded-lg shadow-lg overflow-hidden mb-8 transform transition duration-300 hover:shadow-xl hover:-translate-y-1">
      <img src="/images/mock_main/news/1.png" alt="Новина 1" class="w-full h-64 object-cover"/>
      <div class="p-6">
        <div class="flex items-center text-gray-600 text-sm mb-2">
            <i class="far fa-calendar-alt mr-2"></i>
            "10 січня 2025"
        </div>
        <h2 class="text-2xl font-bold mb-4">"Планові ремонтні роботи на водогоні"</h2>
        <p class="text-gray-600 mb-4">
            "У зв'язку з плановими ремонтними роботами на магістральному водогоні можливе тимчасове припинення водопостачання
              в районі вулиць Центральна та Паркова. Роботи триватимуть з 9:00 до 17:00. Просимо вибачення за тимчасові незручності."
        </p>
        <button class="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700 transition duration-300">
            "Читати далі"
        </button>
      </div>
    </article>
  }
}
