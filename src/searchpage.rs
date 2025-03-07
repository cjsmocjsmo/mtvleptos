use leptos::prelude::*;

#[component]
pub fn SearchPage() -> impl IntoView {

    view! {
        <h1>"SearchPage"</h1>
        <input type="text" placeholder="Search..." />
        <input type="submit" value="Search" />
    }
}