#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Episode {
    Episode: String,
}

#[component]
pub fn TVAndorSeaPage() -> impl IntoView {
    let (episodes, set_episodes) = signal(Vec::new());

    spawn_local(async move {
        match fetch_episodes().await {
            Ok(data) => {
                log::info!("Fetched episodes data: {:?}", data); // Debugging log
                set_episodes.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });

    view! {
        <div class="seaMainDiv">
            <h1 class="seaH1">1923</h1>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 1</h3>
                <div class="seaBtnGrp">
                    {move || episodes.get().iter().map(|episode| view! {
                        <button class="seaBtn">{episode.Episode.clone()}</button>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

async fn fetch_episodes() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/andor").await?;
    let episodes: Vec<Episode> = response.json().await?;
    Ok(episodes)
}