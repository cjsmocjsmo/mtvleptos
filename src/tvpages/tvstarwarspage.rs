#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarWarsPage() -> impl IntoView {
    view! {
        <div class="tv-ass">
            <img src="http://10.0.4.41:9090/skeletoncrew.webp" alt="skeleton crew" />
            <img src="http://10.0.4.41:9090/ahsoka.webp" alt="ahsoka" />
            <img src="http://10.0.4.41:9090/andor.webp" alt="andor" />
            <img src="http://10.0.4.41:9090/badbatch.webp" alt="bad batch" />
            <img src="http://10.0.4.41:9090/bobafett.webp" alt="boba fett" />
            <img src="http://10.0.4.41:9090/mandalorian.webp" alt="mandalorian" />
            <img src="http://10.0.4.41:9090/talesofthejedi.webp" alt="tales of the jedi" />
            <img src="http://10.0.4.41:9090/obiwan.webp" alt="obiwan" />
            <img src="http://10.0.4.41:9090/visions.webp" alt="visions" />
        </div>
    }
}