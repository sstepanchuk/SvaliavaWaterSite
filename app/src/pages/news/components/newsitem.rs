use leptos::prelude::*;

#[component]
pub fn Comp(image: String) -> impl IntoView {
  view! {
    <article class="bg-white rounded-lg shadow-lg overflow-hidden transform transition duration-300 hover:-translate-y-1 hover:shadow-xl">
      <div class="md:flex">
        <div class="md:w-1/3">
          <img src=image alt="Новина 1" class="w-full h-full object-cover"/>
        </div>
        <div class="p-6 md:w-2/3">
          <div class="flex items-center text-gray-600 text-sm mb-2">
            <i class="far fa-calendar-alt mr-2"></i>
            "10 січня 2025"
          </div>
          <h2 class="text-2xl font-bold mb-4">
            "Планові ремонтні роботи на водогоні"
          </h2>
          <p class="text-gray-600 mb-4">
            "У зв'язку з плановими ремонтними роботами на магістральному водогоні можливе тимчасове припинення водопостачання в районі вулиць Центральна та Паркова. Роботи триватимуть з 9:00 до 17:00."
          </p>
          <button class="inline-flex items-center bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700 transition duration-300">
            <span>"Читати далі"</span>
            <i class="fas fa-arrow-right ml-2"></i>
          </button>
        </div>
      </div>
    </article>
  }
}
