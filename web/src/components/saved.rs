use yew::{function_component, Html, html};
use yew::prelude::*;
use super::app_drawer::App_drawer;
use yewdux::prelude::*;
use crate::components::context::{AppState, UIState};
use super::search_nav::Search_nav;
use crate::requests::net_requests::get_saved_configs_dummy;
use crate::components::empties::empty_message;
use wasm_bindgen::closure::Closure;
use web_sys::{console, window};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use crate::requests::login_requests::use_check_authentication;
use crate::components::state_messages::UIStateMsg;
use wasm_bindgen::JsValue;

fn format_date_only(date_time_str: &str) -> String {
    let datetime = chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S");
    datetime.map(|dt| dt.date().to_string()).unwrap_or_else(|_| String::from("Invalid date"))
}


#[function_component(Saved)]
pub fn saved() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let effect_dispatch = dispatch.clone();

    console::log_1(&format!("About to run check auth").into());
    // check_auth(effect_dispatch);

    let saved_configs = use_state(|| Vec::new());
    let loading = use_state(|| true);

    let session_dispatch = effect_dispatch.clone();
    let session_state = state.clone();

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
    
    {
        let saved_configs = saved_configs.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            let saved_configs = saved_configs.clone();
            let loading = loading.clone();
            spawn_local(async move {
                match get_saved_configs_dummy("https://dummyapi.com", 123, &Some("dummyapikey".to_string())).await {
                    Ok(configs) => {
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
                                <div class="flex space-x-4 mb-10 config-page-input">
                                    <div class="relative">
                                        <button id="dropdown1Button" type="button" onclick={toggle_dropdown1.clone()} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                            {"Client Name"}
                                            <svg class="w-2.5 h-2.5 ms-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                            </svg>
                                        </button>
                                        <div id="dropdown1" class={if *dropdown1_open { "z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" } else { "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" }}>
                                            <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdown1Button">
                                                <li>
                                                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Option 1"}</a>
                                                </li>
                                                <li>
                                                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Option 2"}</a>
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="relative">
                                        <button id="dropdown2Button" type="button" onclick={toggle_dropdown2.clone()} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                            {"Device Type"}
                                            <svg class="w-2.5 h-2.5 ms-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                            </svg>
                                        </button>
                                        <div id="dropdown2" class={if *dropdown2_open { "z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" } else { "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" }}>
                                            <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdown2Button">
                                                <li>
                                                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Option 1"}</a>
                                                </li>
                                                <li>
                                                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Option 2"}</a>
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div class="relative">
                                        <button id="dropdown3Button" type="button" onclick={toggle_dropdown3.clone()} class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                            {"Location"}
                                            <svg class="w-2.5 h-2.5 ms-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                            </svg>
                                        </button>
                                        <div id="dropdown3" class={if *dropdown3_open { "z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" } else { "z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" }}>
                                            <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdown3Button">
                                                <li>
                                                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Option 1"}</a>
                                                </li>
                                                <li>
                                                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Option 2"}</a>
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                </div>
                                <div class="item-container mx-auto p-6 shadow-md rounded">
                                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                        {
                                            for saved_configs.iter().map(|config| {
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
                                                            <span class="config-value">{format_date_only(&config.saved_at)}</span>
                                                        </div>
                                                        <div class="config-item">
                                                            <button class="bg-blue-500 mr-5 hover:bg-blue-700 text-white font-bold py-1 px-2 mt-3 rounded">{"Edit Config"}</button>
                                                            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 mt-3 rounded">{"Remove Saved Config"}</button>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                        }
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