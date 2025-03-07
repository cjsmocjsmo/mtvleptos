use leptos::prelude::*;

#[component]
pub fn SearchPage() -> impl IntoView {

    view! {
        <div class="searchDiv">
            <input class="search-input" type="text" placeholder="Search..." />
            <button class="searchButton">Search</button>
        </div>
    }
}