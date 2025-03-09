#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Episode {
    episode_number: String,
}

#[component]
pub fn TVSiloSeaPage() -> impl IntoView {
    let (episodes, set_episodes) = signal(Vec::new());

    spawn_local(async move {
        match fetch_episodes_s1().await {
            Ok(data) => {
                log::info!("Fetched episodes data: {:?}", data); // Debugging log
                set_episodes.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });
    
    let (episodes2, set_episodes2) = signal(Vec::new());
    spawn_local(async move {
        match fetch_episodes_s2().await {
            Ok(data) => {
                log::info!("Fetched episodes data: {:?}", data); // Debugging log
                set_episodes2.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });
    
    view! {
        <div class="seaMainDiv">
            <h1 class="seaH1">Silo</h1>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 1</h3>
                <div class="seaBtnGrp">
                    {move || episodes.get().iter().map(|episode| view! {
                        <button class="seaBtn">{episode.episode_number.clone()}</button>
                    }).collect_view()}
                </div>
            </div>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 2</h3>
                <div class="seaBtnGrp">
                    {move || episodes2.get().iter().map(|episode| view! {
                        <button class="seaBtn">{episode.episode_number.clone()}</button>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

async fn fetch_episodes_s1() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.41:7777/wheeloftime1").await?;
    let episodes: Vec<Episode> = response.json().await?;
    Ok(episodes)
}
async fn fetch_episodes_s2() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.41:7777/wheeloftime2").await?;
    let episodes2: Vec<Episode> = response.json().await?;
    Ok(episodes2)
}