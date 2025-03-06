#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVFantasyPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <img src="http://10.0.4.41:9090/houseofthedragon.webp" alt="House of the Dragon" />
            <img src="http://10.0.4.41:9090/thelordoftheringsringsofpower.webp" alt="The Rings of Power" />
            <img src="http://10.0.4.41:9090/wheeloftime.webp" alt="The Wheel Of Time" />
        </div>
    }
}