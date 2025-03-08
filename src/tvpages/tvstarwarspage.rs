#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarWarsPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <a href="/tvskeletoncrewseapage">
                <img src="http://10.0.4.41:9090/skeletoncrew.webp" alt="skeleton crew" />
            </a>
            <a href="/tvahsokaseapage">
                <img src="http://10.0.4.41:9090/ahsoka.webp" alt="ahsoka" />
            </a>
            <a href="/tvandorsepage">
                <img src="http://10.0.4.41:9090/andor.webp" alt="andor" />
            </a>
            <a href="/tvbadbatchseapage">
                <img src="http://10.0.4.41:9090/badbatch.webp" alt="bad batch" />
            </a>
            <a href="/tvbobafettseapage">
                <img src="http://10.0.4.41:9090/bobafett.webp" alt="boba fett" />
            </a>
            <a href="/tvmandilorianseapage">
                <img src="http://10.0.4.41:9090/mandalorian.webp" alt="mandalorian" />
            </a>
            <a href="/tvtalesofthejediseapage">
                <img src="http://10.0.4.41:9090/talesofthejedi.webp" alt="tales of the jedi" />
            </a>
            <a href="/tvobiwansepage">
                <img src="http://10.0.4.41:9090/obiwan.webp" alt="obiwan" />
            </a>
            <a href="/tvvisionsseapage">
                <img src="http://10.0.4.41:9090/visions.webp" alt="visions" />
            </a>
        </div>
    }
}