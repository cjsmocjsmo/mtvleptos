use leptos::prelude::*;

#[component]
pub fn SearchPage() -> impl IntoView {
    view! {
        <div class="searchDiv">
            <div class="searchInnerDiv">
                <input class="search-input" type="text" placeholder="Search..." />
                <button class="searchButton">Search</button>
            </div>
            <span></span>
            <span></span>
        </div>
    }
}