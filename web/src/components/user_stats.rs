use yew::{function_component, Html, html};
use yew::prelude::*;
use super::app_drawer::App_drawer;
use super::search_nav::Search_nav;
use crate::requests::stat_reqs;
use web_sys::console;
use yewdux::prelude::*;
use crate::components::context::{AppState, UIState, UserStatsStore};
use crate::components::gen_funcs::format_date;


#[function_component(UserStats)]
pub fn user_stats() -> Html {
    let (_state, _dispatch) = use_store::<AppState>();
    let (stat_state, stat_dispatch) = use_store::<UserStatsStore>();
    let user_stats = stat_state.stats.as_ref();

    // use_effect_with(
    //     (),
    //     move |_| {
    //         let effect_dispatch_clone = effect_dispatch.clone();

    //         wasm_bindgen_futures::spawn_local(async move {
    //             let window = web_sys::window().expect("no global `window` exists");
    //             let location = window.location();
    //             let current_route = location.href().expect("should be able to get href");
    //             use_check_authentication(effect_dispatch_clone, &current_route);
    //         });

    //         || ()
    //     }
    // );

    // let error = use_state(|| None);
    let (post_state, _post_dispatch) = use_store::<AppState>();
    let (_audio_state, _audio_dispatch) = use_store::<UIState>();

    // Fetch episodes on component mount
    {
        // let episodes = episodes.clone();
        // let error = error.clone();
        let api_key = post_state.auth_details.as_ref().map(|ud| ud.api_key.clone());
        let user_id = post_state.user_details.as_ref().map(|ud| ud.UserID.clone());
        let server_name = post_state.auth_details.as_ref().map(|ud| ud.server_name.clone());
        console::log_1(&"Test log on home".to_string().into());
        if let Some(api_key) = &api_key {
            console::log_1(&format!("API Key: {:?}", api_key).into());
        }
        if let Some(user_id) = user_id {
            console::log_1(&format!("User ID: {}", user_id).into());
        }
        if let Some(server_name) = &server_name {
            console::log_1(&format!("Server Name: {}", server_name).into());
        }

        console::log_1(&format!("apikey: {:?}", &api_key).into());
        console::log_1(&format!("userid: {:?}", &user_id).into());
        console::log_1(&format!("servername: {:?}", &server_name).into());

        let server_name_effect = server_name.clone();
        let user_id_effect = user_id.clone();
        let api_key_effect = api_key.clone();

        console::log_1(&format!("server_name: {:?}", &server_name_effect).into());
        console::log_1(&format!("user_id: {:?}", &user_id_effect).into());
        console::log_1(&format!("api_key: {:?}", &api_key_effect).into());

        use_effect_with(
            (api_key.clone(), user_id.clone(), server_name.clone()),
            move |_| {
                // your async call here, using stat_dispatch to update stat_state
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(fetched_stats) = stat_reqs::call_get_stats(server_name_effect.unwrap().clone(), api_key.flatten().clone(), &user_id.unwrap()).await {
                        stat_dispatch.reduce_mut(move |state| {
                            state.stats = Some(fetched_stats);
                        });
                    }
                    // handle error case
                });
                || ()
            },
        );
    }

    html! {
        <>
        <div class="main-container">
            <Search_nav />
            <h1 class="text-2xl item_container-text font-bold text-center mb-6">{"User Statistics"}</h1>
            <div class="item-container mx-auto p-6 shadow-md rounded">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">

                        {
                            if let Some(stats) = user_stats {
                                let formatted_date = format_date(&stats.UserCreated);
                                html! {
                                    <>
                                        <div class="stats-card">
                                            <p class="stats-label">{"User Created"}</p>
                                            <p class="stats-value">{&formatted_date}</p>
                                        </div>

                                        <div class="stats-card">
                                            <p class="stats-label">{"Configs You've Created"}</p>
                                            <p class="stats-value">{ &stats.ConfigsCreated }</p>
                                        </div>

                                        <div class="stats-card">
                                            <p class="stats-label">{"Configs Total"}</p>
                                            <p class="stats-value">{ &stats.TotalConfigsCreated }</p>
                                        </div>

                                        <div class="large-card col-span-1 md:col-span-3">
                                            <img src="static/assets/favicon.png" alt="NetTailor Logo" class="large-card-image"/>
                                            <p class="large-card-paragraph item_container-text">{"Thanks for using NetTailor! This app was born from a need to automate easy creation of network configurations and keeping those configs backed up with zero effort along with that. Feel free to reach out for questions and open an issue if you have ideas for new features. Pull Requests on this software are welcome and encouraged."}</p>
                                            <div class="large-card-content flex flex-col space-y-2">
                                                <a href="https://nettailor.org" target="_blank" class="large-card-button focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 focus:outline-none">{"NetTailor Documentation"}</a>
                                                <a href="https://github.com/Lynxx-Networks/NetTailor" target="_blank" class="large-card-button focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 focus:outline-none">{"NetTailor Github Repo"}</a>

                                                // Additional content...
                                            </div>
                                        </div>
                                        // Other stats...
                                    </>
                                }
                            } else {
                                html! { <p class="item_container-text">{"Loading user stats..."}</p> } // or handle the `None` case appropriately
                            }
                        }
                    // </div>
                </div>
            </div>
        </div>
        <App_drawer />
        </>
    }
}
