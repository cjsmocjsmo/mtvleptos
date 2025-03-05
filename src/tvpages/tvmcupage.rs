#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVMCUPage() -> impl IntoView {
    view! {
        <div class="tv-grid">
            // <div class="tv-item">
                <img src="http://10.0.4.41:9090/secret_invasion.webp" alt="Secret Invation" />
                <img src="http://10.0.4.41:9090/iamgroot.webp" alt="I Am Groot" />
                <img src="http://10.0.4.41:9090/loki.webp" alt="Loki" />
                <img src="http://10.0.4.41:9090/moonknight.webp" alt="MoonKnight" />
                <img src="http://10.0.4.41:9090/shehulk.webp" alt="SheHulk" />
                <img src="http://10.0.4.41:9090/Hawkeye.webp" alt="Hawkeye" />
                <img src="http://10.0.4.41:9090/falconwintersoldier.webp" alt="Falcon And The Winter Soldier" />
                <img src="http://10.0.4.41:9090/wandavision.webp" alt="Wandavision" />
            // </div>
        </div>
    }
}