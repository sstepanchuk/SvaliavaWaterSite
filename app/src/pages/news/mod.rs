pub mod components;

use crate::components::*;
use components::*;
use leptos::prelude::*;
use leptos_meta::Title;

pub fn Comp() -> impl IntoView {
  view! {
    <>
      <Title text="КП \"Водопостачання та водовідведення\""/>

      <hero::Comp background_url="/images/main.jpeg".to_string()>
        <h1 class="text-3xl md:text-4xl font-bold text-center">"Новини та оголошення"</h1>
      </hero::Comp>

      <main class="container mx-auto px-4 py-12">
        <searchbar::Comp />
        <div>
          <newsitem::Comp />
          <newsitem::Comp />
          <newsitem::Comp />
        </div>
        <pagination::Comp />
      </main>
    </>
  }
}