#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVActionPage() -> impl IntoView {
    view! {
        <div class="tv-grid">
            <div class="tv-item">
                <img src="http://10.0.4.41:9090/shogun.webp" alt="Showgun" />
                <ing src="http://10.0.4.41:9090/continental.webp" alt="Continental" />
            </div>
        </div>
    }
}