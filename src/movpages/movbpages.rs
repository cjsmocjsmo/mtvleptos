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
pub fn BruceLeePage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_brucelee().await {
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

    // view! {
    //     <div class="mov-grid">
    //         {move || infos.get().iter().map(|info| view! {
    //             <div class="mov-item">
    //                 <img src={info.HttpThumbPath.clone()} alt={info.Name.clone()} />
    //                 <p>{info.Name.clone()}</p>
    //             </div>
    //         }).collect_view()}
    //     </div>
    // }
}
async fn fetch_brucelee() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/brucelee").await?;
    let brucelee: Vec<Infos> = response.json().await?;
    Ok(brucelee)
}

#[component]
pub fn BruceWillisPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_brucewillis().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-grid">
            {move || infos.get().iter().map(|info| view! {
                <div class="mov-item">
                    <img src={info.HttpThumbPath.clone()} alt={info.Name.clone()} />
                    <p>{info.Name.clone()}</p>
                </div>
            }).collect_view()}
        </div>
    }
}
async fn fetch_brucewillis() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/brucewillis").await?;
    let brucewillis: Vec<Infos> = response.json().await?;
    Ok(brucewillis)
}

// #[component]
// pub fn BuzzPage() -> impl IntoView {
//     view! {
//         <h1>"Buzz Page"</h1>
//     }
// }

#[component]
pub fn BuzzPage() -> impl IntoView {
    let (infos, set_infos) = signal(Vec::new());

    spawn_local(async move {
        match fetch_buzz().await {
            Ok(data) => {
                log::info!("Fetched infos data: {:?}", data); // Debugging log
                set_infos.set(data);
            },
            Err(err) => log::error!("Error fetching infos data: {:?}", err),
        }
    });

    view! {
        <div class="mov-grid">
            {move || infos.get().iter().map(|info| view! {
                <div class="mov-item">
                    <img src={info.HttpThumbPath.clone()} alt={info.Name.clone()} />
                    <p>{info.Name.clone()}</p>
                </div>
            }).collect_view()}
        </div>
    }
}
async fn fetch_buzz() -> Result<Vec<Infos>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/buzz").await?;
    let buzz: Vec<Infos> = response.json().await?;
    Ok(buzz)
}