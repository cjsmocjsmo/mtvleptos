#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVComedyPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <a href="/tvfubarseapage">
                <img src="http://10.0.4.41:9090/fubar.webp" alt="Comedy" />
            </a>
        </div>
    }
}