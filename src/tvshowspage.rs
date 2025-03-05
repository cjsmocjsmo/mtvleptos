use leptos::prelude::*;

#[component]
pub fn TVShowsListPage() -> impl IntoView {
    view! {
        <div class="tvshows-section">
            <div class="tvshows-sectionDiv">
                <a class="tvshows-sectionDivItem" href="/tvactionpage">"Action"</a>
                <a class="tvshows-sectionDivItem" href="/tvcomedypage">"Comedy"</a>
                <a class="tvshows-sectionDivItem" href="/tvfantasypage">"Fantasy"</a>
                <a class="tvshows-sectionDivItem" href="/tvmcupage">"MCU"</a>
                <a class="tvshows-sectionDivItem" href="/tvscience">"Science"</a>
                <a class="tvshows-sectionDivItem" href="/tvscifi">"SciFi"</a>
                <a class="tvshows-sectionDivItem" href="/startrek">"Star Trek"</a>
                <a class="tvshows-sectionDivItem" href="/starwars">"Star Wars"</a>
                <a class="tvshows-sectionDivItem" href="/westerns">"Westerns"</a>
            </div>
        </div>
    }
}
