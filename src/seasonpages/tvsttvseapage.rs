#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Episode {
    Episode: String,
}

#[component]
pub fn TVSTTVSeaPage() -> impl IntoView {
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

    let (episodes3, set_episodes3) = signal(Vec::new());
    spawn_local(async move {
        match fetch_episodes_s3().await {
            Ok(data) => {
                log::info!("Fetched episodes data: {:?}", data); // Debugging log
                set_episodes3.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });
    
    view! {
        <div class="seaMainDiv">
            <h1 class="seaH1">Star Trek</h1>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 1</h3>
                <div class="seaBtnGrp">
                    {move || episodes.get().iter().map(|episode| view! {
                        <button class="seaBtn">{episode.Episode.clone()}</button>
                    }).collect_view()}
                </div>
            </div>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 2</h3>
                <div class="seaBtnGrp">
                    {move || episodes2.get().iter().map(|episode| view! {
                        <button class="seaBtn">{episode.Episode.clone()}</button>
                    }).collect_view()}
                </div>
            </div>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 3</h3>
                <div class="seaBtnGrp">
                    {move || episodes3.get().iter().map(|episode| view! {
                        <button class="seaBtn">{episode.Episode.clone()}</button>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

async fn fetch_episodes_s1() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/sttv1").await?;
    let episodes: Vec<Episode> = response.json().await?;
    Ok(episodes)
}
async fn fetch_episodes_s2() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/sttv2").await?;
    let episodes2: Vec<Episode> = response.json().await?;
    Ok(episodes2)
}
async fn fetch_episodes_s3() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/sttv3").await?;
    let episodes3: Vec<Episode> = response.json().await?;
    Ok(episodes3)
}
