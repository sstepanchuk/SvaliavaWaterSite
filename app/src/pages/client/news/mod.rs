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

      <searchbar::Comp />

      <main class="container mx-auto grid gap-8 max-w-5xl px-4 py-12">
        <pagination::Comp />
        <newsitem::Comp image="/images/mock_main/news/1.png".to_string() />
        <newsitem::Comp image="/images/mock_main/news/2.png".to_string() />
        <newsitem::Comp image="/images/mock_main/news/3.png".to_string() />
        <pagination::Comp />
      </main>
    </>
  }
}