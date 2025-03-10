#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVMCUPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <a href="/tvsecretinvasionseapage">
                <img src="http://10.0.4.41:9090/secret_invasion.webp" alt="Secret Invation" />
            </a>
            <a href="/tviamgrootseapage">
                <img src="http://10.0.4.41:9090/iamgroot.webp" alt="I Am Groot" />
            </a>
            <a href="/tvlokiseapage">
                <img src="http://10.0.4.41:9090/loki.webp" alt="Loki" />
            </a>
            <a href="/tvmoonknightseapage">
                <img src="http://10.0.4.41:9090/moonknight.webp" alt="MoonKnight" />
            </a>
            <a href="/tvshehulkseapage">
                <img src="http://10.0.4.41:9090/shehulk.webp" alt="SheHulk" />
            </a>
            <a href="/tvhawkeyeseapage">
                <img src="http://10.0.4.41:9090/Hawkeye.webp" alt="Hawkeye" />
            </a>
            <a href="/tvfalconwintersoldierseapage">
                <img src="http://10.0.4.41:9090/falconwintersoldier.webp" alt="Falcon And The Winter Soldier" />
            </a>
            <a href="/tvwandavisionseapage">
                <img src="http://10.0.4.41:9090/wandavision.webp" alt="Wandavision" />
            </a>
        </div>
    }
}