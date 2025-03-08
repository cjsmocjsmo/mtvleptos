#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVActionPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <a href="/tvcontinentalseapage">
                <img src="http://10.0.4.41:9090/continental.webp" alt="Continental" />
            </a>
            <a href="/tvshogunseapage">
                <img src="http://10.0.4.41:9090/shogun.webp" alt="Showgun" />
            </a>
        </div>
    }
}