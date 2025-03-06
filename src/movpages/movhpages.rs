#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Infos {
    Name: String,
    HttpThumbPath: String,
}

#[component]
pub fn HarrisonFordPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_harrisonford().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {move || infos.get().iter().map(|info| view! {
                <img src={info.HttpThumbPath.clone()} alt={info.Name.clone()} />
            }).collect_view()}
        </div>
    }
}
async fn fetch_harrisonford() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/harrisonford").await?;
    let harrisonford: Vec<Infos> = response.json().await?;
    Ok(harrisonford)
}

#[component]
pub fn HarryPotterPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_harrypotter().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {move || infos.get().iter().map(|info| view! {
                <img src={info.HttpThumbPath.clone()} alt={info.Name.clone()} />
            }).collect_view()}
        </div>
    }
}
async fn fetch_harrypotter() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/harrypotter").await?;
    let harrypotter: Vec<Infos> = response.json().await?;
    Ok(harrypotter)
}

#[component]
pub fn HellBoyPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_hellboy().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-row">
            {move || infos.get().iter().map(|info| view! {
                <img src={info.HttpThumbPath.clone()} alt={info.Name.clone()} />
            }).collect_view()}
        </div>
    }
}
async fn fetch_hellboy() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/hellboy").await?;
    let hellboy: Vec<Infos> = response.json().await?;
    Ok(hellboy)
}