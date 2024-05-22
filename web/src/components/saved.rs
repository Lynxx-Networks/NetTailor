use yew::{function_component, Html, html};
use yew::prelude::*;
use super::app_drawer::App_drawer;
use yewdux::prelude::*;
use crate::components::context::{AppState, UIState};
use super::search_nav::Search_nav;
use crate::requests::net_requests::{SavedConfig, remove_saved_config, get_saved_configs};
use crate::components::empties::empty_message;
use wasm_bindgen::closure::Closure;
use web_sys::{console, window};
use wasm_bindgen_futures::spawn_local;
use yew_router::history::{BrowserHistory, History};
use wasm_bindgen::JsCast;
use crate::requests::login_requests::use_check_authentication;
use crate::components::state_messages::UIStateMsg;
use wasm_bindgen::JsValue;
// use base64::engine::Config;
use std::collections::HashSet;
use crate::components::gen_funcs::format_date;

fn extract_unique_values(configs: &Vec<SavedConfig>) -> (HashSet<String>, HashSet<String>, HashSet<String>) {
    let mut client_names = HashSet::new();
    let mut device_types = HashSet::new();
    let mut locations = HashSet::new();

    for config in configs {
        client_names.insert(config.client_name.clone());
        device_types.insert(config.device_type.clone());
        locations.insert(config.location.clone());
    }

    (client_names, device_types, locations)
}


fn format_date_only(date_time_str: &str) -> String {
    let datetime = chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S");
    datetime.map(|dt| dt.date().to_string()).unwrap_or_else(|_| String::from("Invalid date"))
}

fn filter_configs(configs: &Vec<SavedConfig>, client_name: &Option<String>, device_type: &Option<String>, location: &Option<String>) -> Vec<SavedConfig> {
    configs.iter().filter(|config| {
        let client_name_matches = client_name.as_ref().map_or(true, |client| &config.client_name == client);
        let device_type_matches = device_type.as_ref().map_or(true, |device| &config.device_type == device);
        let location_matches = location.as_ref().map_or(true, |loc| &config.location == loc);
        client_name_matches && device_type_matches && location_matches
    }).cloned().collect()
}




#[function_component(Saved)]
pub fn saved() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let effect_dispatch = dispatch.clone();

    console::log_1(&format!("About to run check auth").into());
    // check_auth(effect_dispatch);

    let loading = use_state(|| true);

    let session_dispatch = effect_dispatch.clone();
    let session_state = state.clone();

    let selected_client_name = use_state(|| None::<String>);
    let selected_device_type = use_state(|| None::<String>);
    let selected_location = use_state(|| None::<String>);
    
    let client_names: UseStateHandle<HashSet<String>> = use_state(HashSet::new);
    let device_types: UseStateHandle<HashSet<String>> = use_state(HashSet::new);
    let locations: UseStateHandle<HashSet<String>> = use_state(HashSet::new);


    use_effect_with((), move |_| {
        // Check if the page reload action has already occurred to prevent redundant execution
        if session_state.reload_occured.unwrap_or(false) {
            // Logic for the case where reload has already been processed
        } else {
            // Normal effect logic for handling page reload
            let window = web_sys::window().expect("no global `window` exists");
            let performance = window.performance().expect("should have performance");
            let navigation_type = performance.navigation().type_();
            
            if navigation_type == 1 { // 1 stands for reload
                let session_storage = window.session_storage().unwrap().unwrap();
                session_storage.set_item("isAuthenticated", "false").unwrap();
                console::log_1(&"Page was reloaded, user not authenticated, clearing session storage".into());
            }
    
            // Always check authentication status
            let current_route = window.location().href().unwrap_or_default();
            use_check_authentication(session_dispatch.clone(), &current_route);
    
            // Mark that the page reload handling has occurred
            session_dispatch.reduce_mut(|state| {
                state.reload_occured = Some(true);
                state.clone() // Return the modified state
            });
        }
    
        || ()
    });

    let saved_configs: UseStateHandle<Vec<SavedConfig>> = use_state(Vec::new);
    {
        let saved_configs = saved_configs.clone();
        let loading = loading.clone();
        let client_names = client_names.clone();
        let device_types = device_types.clone();
        let locations = locations.clone();
        let api_key = state.auth_details.as_ref().map(|ud| ud.api_key.clone());
        let server_name = state.auth_details.as_ref().map(|ud| ud.server_name.clone());
        let user_id = state.user_details.as_ref().map(|ud| ud.UserID.clone());
    

        use_effect_with((), move |_| {
            let saved_configs = saved_configs.clone();
            let loading = loading.clone();
            let client_names = client_names.clone();
            let device_types = device_types.clone();
            let locations = locations.clone();
            let api_key = api_key.clone();
            let server_name = server_name.clone();
            let user_id = user_id.clone();


            spawn_local(async move {
                match get_saved_configs(&server_name.unwrap(), user_id.unwrap(), &api_key.unwrap()).await {
                    Ok(configs) => {
                        let (clients, devices, locs) = extract_unique_values(&configs);
                        client_names.set(clients);
                        device_types.set(devices);
                        locations.set(locs);

                        saved_configs.set(configs);
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&JsValue::from_str(&format!("Failed to fetch configs: {}", e)));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }
    let (_post_state, _post_dispatch) = use_store::<AppState>();
    let (audio_state, audio_dispatch) = use_store::<UIState>();
    let error_message = audio_state.error_message.clone();
    let info_message = audio_state.info_message.clone();
    web_sys::console::log_1(&"testlog".into());

    {
        let ui_dispatch = audio_dispatch.clone();
        use_effect(move || {
            let window = window().unwrap();
            let document = window.document().unwrap();

            let closure = Closure::wrap(Box::new(move |_event: Event| {
                ui_dispatch.apply(UIStateMsg::ClearErrorMessage);
                ui_dispatch.apply(UIStateMsg::ClearInfoMessage);
            }) as Box<dyn Fn(_)>);

            document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();

            // Return cleanup function
            move || {
                document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
                closure.forget(); // Prevents the closure from being dropped
            }
        });
    }

    let dropdown1_open = use_state(|| false);
    let dropdown2_open = use_state(|| false);
    let dropdown3_open = use_state(|| false);

    let on_client_name_select = {
        let selected_client_name = selected_client_name.clone();
        Callback::from(move |client_name: String| {
            selected_client_name.set(Some(client_name));
        })
    };
    
    let on_device_type_select = {
        let selected_device_type = selected_device_type.clone();
        Callback::from(move |device_type: String| {
            selected_device_type.set(Some(device_type));
        })
    };
    
    let on_location_select = {
        let selected_location = selected_location.clone();
        Callback::from(move |location: String| {
            selected_location.set(Some(location));
        })
    };

    let toggle_dropdown1 = {
        let dropdown1_open = dropdown1_open.clone();
        Callback::from(move |_| dropdown1_open.set(!*dropdown1_open))
    };

    let toggle_dropdown2 = {
        let dropdown2_open = dropdown2_open.clone();
        Callback::from(move |_| dropdown2_open.set(!*dropdown2_open))
    };

    let toggle_dropdown3 = {
        let dropdown3_open = dropdown3_open.clone();
        Callback::from(move |_| dropdown3_open.set(!*dropdown3_open))
    };



    let filtered_configs = filter_configs(&*saved_configs, &*selected_client_name, &*selected_device_type, &*selected_location);
    
    let on_remove_saved_config = {
        let api_key = state.auth_details.as_ref().map(|ud| ud.api_key.clone());
        let server_name = state.auth_details.as_ref().map(|ud| ud.server_name.clone());
        let user_id = state.user_details.as_ref().map(|ud| ud.UserID.clone()).unwrap_or(0);
    
        Callback::from(move |config_id: i32| {
            let api_key = api_key.clone();
            let server_name = server_name.clone();
            spawn_local(async move {
                match remove_saved_config(&server_name.unwrap(), user_id, config_id, &api_key.unwrap()).await {
                    Ok(_) => {
                        web_sys::console::log_1(&JsValue::from_str("Configuration removed successfully"));
                    }
                    Err(e) => {
                        web_sys::console::log_1(&JsValue::from_str(&format!("Failed to remove configuration: {}", e)));
                    }
                }
            });
        })
    };

    fn update_current_editing_config(dispatch: Dispatch<AppState>, config_id: i32) {
        dispatch.reduce_mut(|state| {
            state.current_editing_config = Some(config_id);
            state.clone()
        });
    }
    
    let on_edit_config = {
        let history = BrowserHistory::new();
        let history_clone = history.clone();
        let dispatch = dispatch.clone();
    
        Callback::from(move |config_id: i32| {
            update_current_editing_config(dispatch.clone(), config_id);
            history_clone.push("/edit_config");
        })
    };
    

    html! {
        <>
        <div class="main-container">
            <Search_nav />
            {
                if *loading { // If loading is true, display the loading animation
                    html! {
                        <div class="loading-animation">
                            <div class="frame1"></div>
                            <div class="frame2"></div>
                            <div class="frame3"></div>
                            <div class="frame4"></div>
                            <div class="frame5"></div>
                            <div class="frame6"></div>
                        </div>
                    }
                } else {
                            html! {
                                <>
                                <h1 class="text-2xl item_container-text font-bold text-center mb-6">{"Saved Configurations"}</h1>
                                <div class="config-container">    
                                    <div class="filter-bar w-1/10 p-4">
                                        <h2 class="text-lg font-bold mb-4 item_container-text">{"Filter:"}</h2>
                                        <div class="flex space-x-4 mb-5">
                                            <div class="relative inline-block text-left">
                                                <button id="dropdown1Button" type="button" onclick={toggle_dropdown1.clone()} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                                    {"Client Name"}
                                                    <svg class="w-2.5 h-2.5 ms-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                                    </svg>
                                                </button>
                                                <div id="dropdown1" class={if *dropdown1_open { "z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" } else { "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" }}>
                                                    <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdown1Button">
                                                        { for client_names.iter().map(|client| html! {
                                                            <li>
                                                                <button onclick={
                                                                    let on_client_name_select = on_client_name_select.clone();
                                                                    let client = client.clone();
                                                                    Callback::from(move |e: MouseEvent| {
                                                                        e.prevent_default();
                                                                        on_client_name_select.emit(client.clone());
                                                                    })
                                                                } class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">
                                                                    { client }
                                                                </button>
                                                            </li>
                                                        }) }
                                                    </ul>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="flex space-x-4 mb-5">
                                            <div class="relative inline-block text-left">
                                                <button id="dropdown2Button" type="button" onclick={toggle_dropdown2.clone()} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                                    {"Device Type"}
                                                    <svg class="w-2.5 h-2.5 ms-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                                    </svg>
                                                </button>
                                                <div id="dropdown2" class={if *dropdown2_open { "z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" } else { "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" }}>
                                                    <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdown2Button">
                                                        { for device_types.iter().map(|device| html! {
                                                            <li>
                                                                <button onclick={
                                                                    let on_device_type_select = on_device_type_select.clone();
                                                                    let device = device.clone();
                                                                    Callback::from(move |e: MouseEvent| {
                                                                        e.prevent_default();
                                                                        on_device_type_select.emit(device.clone());
                                                                    })
                                                                } class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">
                                                                    { device }
                                                                </button>
                                                            </li>
                                                        }) }
                                                    </ul>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="flex space-x-4 mb-5">
                                            <div class="relative inline-block text-left">
                                                <button id="dropdown3Button" type="button" onclick={toggle_dropdown3.clone()} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                                    {"Location"}
                                                    <svg class="w-2.5 h-2.5 ms-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                                    </svg>
                                                </button>
                                                <div id="dropdown3" class={if *dropdown3_open { "z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" } else { "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" }}>
                                                    <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdown3Button">
                                                        { for locations.iter().map(|location| html! {
                                                            <li>
                                                                <button onclick={
                                                                    let on_location_select = on_location_select.clone();
                                                                    let location = location.clone();
                                                                    Callback::from(move |e: MouseEvent| {
                                                                        e.prevent_default();
                                                                        on_location_select.emit(location.clone());
                                                                    })
                                                                } class="block w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">
                                                                    { location }
                                                                </button>
                                                            </li>
                                                        }) }
                                                    </ul>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                <div class="item-container w-9/10 mx-auto p-6 shadow-md rounded">
                                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                        {
                                            for filtered_configs.iter().map(|config| {
                                                let config_id = config.config_id;
                                                html! {
                                                    <div class="config-card">
                                                        <div class="config-item">
                                                            <span class="config-label">{"Hostname: "}</span>
                                                            <span class="config-value">{&config.device_hostname}</span>
                                                        </div>
                                                        <div class="config-item">
                                                            <span class="config-label">{"Client Name: "}</span>
                                                            <span class="config-value">{&config.client_name}</span>
                                                        </div>
                                                        <div class="config-item">
                                                            <span class="config-label">{"Location: "}</span>
                                                            <span class="config-value">{&config.location}</span>
                                                        </div>
                                                        <div class="config-item">
                                                            <span class="config-label">{"Device Type: "}</span>
                                                            <span class="config-value">{&config.device_type}</span>
                                                        </div>
                                                        <div class="config-item">
                                                            <span class="config-label">{"Saved On: "}</span>
                                                            <span class="config-value">{format_date(&config.saved_at)}</span>
                                                        </div>
                                                        <div class="config-item">
                                                        <button onclick={on_edit_config.reform(move |_| config_id.clone())} class="bg-blue-500 mr-5 hover:bg-blue-700 text-white font-bold py-1 px-2 mt-3 rounded">{"Edit Config"}</button>
                                                            <button onclick={on_remove_saved_config.reform(move |_| config_id.clone())} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 mt-3 rounded">{"Remove Saved Config"}</button>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                        }
                                    </div>
                                </div>
                                </div>
                                </>
                            }
                        }
                    }
        // Conditional rendering for the error banner
        if let Some(error) = error_message {
            <div class="error-snackbar">{ error }</div>
        }
        if let Some(info) = info_message {
            <div class="info-snackbar">{ info }</div>
        }
        </div>
        <App_drawer />
        </>
    }
}