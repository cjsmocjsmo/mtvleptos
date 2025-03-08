#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TV1923SeaPage() -> impl IntoView {
    view! {
        <div class="seaMainDiv">
            <h1 class="seaH1">1923</h1>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 1</h3>
                <div class="seaBtnGrp">
                    <button class="seaBtn">1</button>
                    <button class="seaBtn">2</button>
                    <button class="seaBtn">3</button>
                </div>
            </div>
        </div>
    }
}