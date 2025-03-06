#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarTrekPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <img src="http://10.0.4.41:9090/voyager.webp" alt="voyager" />
            <img src="http://10.0.4.41:9090/sttv.webp" alt="STTV" />
            <img src="http://10.0.4.41:9090/enterprise.webp" alt="enterprise" />
            <img src="http://10.0.4.41:9090/nextgen.webp" alt="next generation" />
            <img src="http://10.0.4.41:9090/discovery.webp" alt="discovery" />
            <img src="http://10.0.4.41:9090/picard.webp" alt="picard" />
            <img src="http://10.0.4.41:9090/lowerdecks.webp" alt="lower decks" />
            <img src="http://10.0.4.41:9090/prodigy.webp" alt="prodigy" />
            <img src="http://10.0.4.41:9090/strangenewworlds.webp" alt="strange new worlds" />
        </div>
    }
}