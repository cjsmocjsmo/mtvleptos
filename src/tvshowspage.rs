use leptos::prelude::*;

use crate::NavBar;

#[component]
pub fn TVShowsListPage() -> impl IntoView {
    view! {
        
        <NavBar />
        <main>
            <div class="tvshows-section">
                <div class="tvshows-sectionDiv">
                    <a class="tvshows-sectionDivItem" href="/tvactionpage">"Action"</a>
                    <a class="tvshows-sectionDivItem" href="/tvcomedypage">"Comedy"</a>
                    <a class="tvshows-sectionDivItem" href="/tvfantasypage">"Fantasy"</a>
                    <a class="tvshows-sectionDivItem" href="/tvmcupage">"MCU"</a>
                    <a class="tvshows-sectionDivItem" href="/tvscience">"Science"</a>
                    <a class="tvshows-sectionDivItem" href="/tvscifi">"SciFi"</a>
                    <a class="tvshows-sectionDivItem" href="/tvstartrek">"Star Trek"</a>
                    <a class="tvshows-sectionDivItem" href="/tvstarwars">"Star Wars"</a>
                    <a class="tvshows-sectionDivItem" href="/tvwesterns">"Westerns"</a>
                </div>
                <span></span>
            </div>
        </main>
    }
}
