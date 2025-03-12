#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Episode {
    TvId: String,
    Episode: String,
}

#[component]
pub fn TVForAllManKindSeaPage() -> impl IntoView {
    let (episodes, set_episodes) = signal(Vec::new());

    spawn_local(async move {
        match fetch_episodes_s1().await {
            Ok(mut data) => {
                data.sort_by(|a, b| a.Episode.cmp(&b.Episode));
                log::info!("Fetched and sorted episodes data: {:?}", data); // Debugging log
                set_episodes.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });
    
    let (episodes2, set_episodes2) = signal(Vec::new());
    spawn_local(async move {
        match fetch_episodes_s2().await {
            Ok(mut data) => {
                data.sort_by(|a, b| a.Episode.cmp(&b.Episode));
                log::info!("Fetched and sorted episodes data: {:?}", data); // Debugging log
                set_episodes2.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });

    let (episodes3, set_episodes3) = signal(Vec::new());
    spawn_local(async move {
        match fetch_episodes_s3().await {
            Ok(mut data) => {
                data.sort_by(|a, b| a.Episode.cmp(&b.Episode));
                log::info!("Fetched and sorted episodes data: {:?}", data); // Debugging log
                set_episodes3.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });

    let (episodes4, set_episodes4) = signal(Vec::new());
    spawn_local(async move {
        match fetch_episodes_s4().await {
            Ok(mut data) => {
                data.sort_by(|a, b| a.Episode.cmp(&b.Episode));
                log::info!("Fetched and sorted episodes data: {:?}", data); // Debugging log
                set_episodes4.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });
    
    view! {
        <div class="seaMainDiv">
            <h1 class="seaH1">For All Man Kind</h1>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 1</h3>
                <div class="seaBtnGrp">
                {
                    let episodes_list = move || episodes.get().clone(); move || episodes_list().iter().map(|episode| {
                        let tv_id = episode.TvId.clone();
                        let episode_clone = episode.Episode.clone();
                        view! {
                            <button class="seaBtn" on:click=move |_| {
                                let tv_id = tv_id.clone();
                                spawn_local(async move {
                                    if let Err(err) = send_get_request(&tv_id).await {
                                        log::error!("Error sending GET request: {:?}", err);
                                    }
                                });
                            }>{episode_clone}</button>
                        }
                    }).collect_view()
                }
                </div>
            </div>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 2</h3>
                <div class="seaBtnGrp">
                {
                    let episodes_list2 = move || episodes2.get().clone(); move || episodes_list2().iter().map(|episode| {
                        let tv_id = episode.TvId.clone();
                        let episode_clone = episode.Episode.clone();
                        view! {
                            <button class="seaBtn" on:click=move |_| {
                                let tv_id = tv_id.clone();
                                spawn_local(async move {
                                    if let Err(err) = send_get_request(&tv_id).await {
                                        log::error!("Error sending GET request: {:?}", err);
                                    }
                                });
                            }>{episode_clone}</button>
                        }
                    }).collect_view()
                }
                </div>
            </div>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 3</h3>
                <div class="seaBtnGrp">
                {
                    let episodes_list3 = move || episodes3.get().clone(); move || episodes_list3().iter().map(|episode| {
                        let tv_id = episode.TvId.clone();
                        let episode_clone = episode.Episode.clone();
                        view! {
                            <button class="seaBtn" on:click=move |_| {
                                let tv_id = tv_id.clone();
                                spawn_local(async move {
                                    if let Err(err) = send_get_request(&tv_id).await {
                                        log::error!("Error sending GET request: {:?}", err);
                                    }
                                });
                            }>{episode_clone}</button>
                        }
                    }).collect_view()
                }
                </div>
            </div>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 4</h3>
                <div class="seaBtnGrp">
                {
                    let episodes_list4 = move || episodes4.get().clone(); move || episodes_list4().iter().map(|episode| {
                        let tv_id = episode.TvId.clone();
                        let episode_clone = episode.Episode.clone();
                        view! {
                            <button class="seaBtn" on:click=move |_| {
                                let tv_id = tv_id.clone();
                                spawn_local(async move {
                                    if let Err(err) = send_get_request(&tv_id).await {
                                        log::error!("Error sending GET request: {:?}", err);
                                    }
                                });
                            }>{episode_clone}</button>
                        }
                    }).collect_view()
                }
                </div>
            </div>
        </div>
    }
}

async fn fetch_episodes_s1() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/forallmankind1").await?;
    let episodes: Vec<Episode> = response.json().await?;
    Ok(episodes)
}
async fn fetch_episodes_s2() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/forallmankind2").await?;
    let episodes2: Vec<Episode> = response.json().await?;
    Ok(episodes2)
}
async fn fetch_episodes_s3() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/forallmankind3").await?;
    let episodes3: Vec<Episode> = response.json().await?;
    Ok(episodes3)
}
async fn fetch_episodes_s4() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/forallmankind4").await?;
    let episodes4: Vec<Episode> = response.json().await?;
    Ok(episodes4)
}

async fn send_get_request(tv_id: &str) -> Result<(), Error> {
    let url = format!("http://10.0.4.41:7777/set_player_tv_id/{}", tv_id);
    reqwest::get(&url).await?;
    Ok(())
}
