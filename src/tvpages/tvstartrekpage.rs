#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarTrekPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <a href="/tvvoyagerseapage">
                <img src="http://10.0.4.41:9090/voyager.webp" alt="voyager" />
            </a>
            <a href="/tvsttvseapage"
                <img src="http://10.0.4.41:9090/sttv.webp" alt="STTV" />
            </a>
            <a href="/tventerpriseseapage"
                <img src="http://10.0.4.41:9090/enterprise.webp" alt="enterprise" />
            </a>
            <a href="/tvnexgenseapage"
                <img src="http://10.0.4.41:9090/nextgen.webp" alt="next generation" />
            </a>
            <a href="tvdiscoveryseapage">
                <img src="http://10.0.4.41:9090/discovery.webp" alt="discovery" />
            </a>
            <a href="/tvpicardseapage"
                <img src="http://10.0.4.41:9090/picard.webp" alt="picard" />
            <a href="/tvlowerdecksseapage"
                <img src="http://10.0.4.41:9090/lowerdecks.webp" alt="lower decks" />
            </a>
            <a href="/tvprodigyseapage">
                <img src="http://10.0.4.41:9090/prodigy.webp" alt="prodigy" />
            </a>
            <a href="tvstrangenewworldsseapage">
                <img src="http://10.0.4.41:9090/strangenewworlds.webp" alt="strange new worlds" />
            </a>
        </div>
    }
}