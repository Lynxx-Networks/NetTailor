use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use yewdux::use_store;
use crate::components::gen_funcs::get_base_url;
use web_sys::window;
use super::app_drawer::App_drawer;
use super::search_nav::Search_nav;
use crate::components::context::{AppState, UIState};
use crate::components::misc_func::generate_config;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use crate::components::settings::AccordionItem;
use crate::components::settings::AccordionItemPosition;
use wasm_bindgen_futures::spawn_local;
use crate::requests::net_requests::{call_edit_config, DeviceInfo, get_config_raw, send_config_to_server, add_config_db, DeviceConfig};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    name: String,
}


async fn get_clients_list() -> Result<Vec<Client>, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;
    let session_storage = window.session_storage()?.ok_or_else(|| JsValue::from_str("Failed to get session storage"))?;

    let clients_json = session_storage.get_item("clients")?.ok_or_else(|| JsValue::from_str("No clients found in session storage"))?;

    serde_json::from_str(&clients_json).map_err(|e| JsValue::from_str(&e.to_string()))
}


#[function_component(EditConfig)]
pub fn edit_config() -> Html {
    let (state, _dispatch) = use_store::<AppState>();


    let (_post_state, _post_dispatch) = use_store::<AppState>();
    let (audio_state, _audio_dispatch) = use_store::<UIState>();
    let error_message = audio_state.error_message.clone();
    let info_message = audio_state.info_message.clone();
    let api_key = state.auth_details.as_ref().map(|ud| ud.api_key.clone());
    let server_name = state.auth_details.as_ref().map(|ud| ud.server_name.clone());
    let _user_id = state.user_details.as_ref().map(|ud| ud.UserID.clone());
    let shared_link = use_state(|| String::new());
    let config_content = use_state(|| "".to_string());
    let config_id = state.current_editing_config;

    let config_api_key = api_key.clone();
    let config_server_name = server_name.clone();
    let get_config_content = config_content.clone();
    let get_config_id = config_id.clone();
    use_effect_with((), move |_| {
        let config_content = get_config_content.clone();
        let config_api_key = config_api_key.clone();
        let config_server_name = config_server_name.clone();
        let config_id = get_config_id.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let Some(config_id) = config_id {
                match get_config_raw(&config_server_name.unwrap(), config_id, &config_api_key.unwrap()).await {
                    Ok(content) => {
                        config_content.set(content);
                    },
                    Err(err) => {
                        eprintln!("Failed to fetch configuration: {:?}", err);
                    }
                }
            }
        });

        || () // Cleanup logic if necessary
    });

    let config_content_oninput = {
        let config_content = config_content.clone();
        Callback::from(move |e: InputEvent| {
            config_content.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
        })
    };

    let link_display = if !shared_link.is_empty() {
        html! {
            <div class="link-display">
                <p class="text-lg font-bold mb-2">{"Command to install config on switch:"}</p>
                <div class="border border-gray-300 p-2 rounded">
                    <p class="font-mono text-sm">{format!("copy {} running-config", *shared_link)}</p>
                </div>
            </div>
        }
    } else {
        html! {}
    };

    let generate_config_id = config_id.clone();
    let generate_config_click = {
        let config_content = config_content.clone();
        let api_key = api_key.clone();
        let server_name = server_name.clone();
        let config_id = config_id.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let config_content = config_content.clone();
            let api_key = api_key.clone();
            let server_name = server_name.clone();
            let config_id = config_id.unwrap();
            let shared_link = shared_link.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match call_edit_config(&server_name.unwrap(), config_id, (*config_content).clone(), &api_key.unwrap()).await {
                    Ok(config_response) => {
                        web_sys::console::log_1(&JsValue::from_str("Configuration edited successfully"));
                        shared_link.set(config_response.shared_link.clone());
                    },
                    Err(err) => {
                        web_sys::console::log_1(&JsValue::from_str(&format!("Failed to edit configuration: {}", err)));
                    }
                }
            });
        })
    };

    
    html! {
        <>
        <div class="main-container">
            <Search_nav />
            {
                html! {
                    // <div class="flex flex-col space-y-4 w-full max-w-xs p-8 border border-gray-300 rounded-lg shadow-lg bg-gray-600">
                        <>
                        <h1 class="text-2xl item_container-text font-bold text-center mb-6">{"Edit Configuration"}</h1>
                        <div class="relative container item_container-text">
                            <div>
                                <textarea class="config-output peer h-full min-h-[500px] w-full resize-vertical rounded-[7px] border border-blue-gray-200 border-t-transparent bg-transparent px-3 py-2.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border placeholder-shown:border-blue-gray-200 placeholder-shown:border-t-blue-gray-200 focus:border-2 focus:border-gray-900 focus:border-t-transparent focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50" value={(*config_content).clone()} oninput={config_content_oninput} />
                                <label
                                    class="before:content[' '] after:content[' '] pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-[11px] font-normal leading-tight text-blue-gray-400 transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-blue-gray-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-blue-gray-200 after:transition-all peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[3.75] peer-placeholder-shown:text-blue-gray-500 peer-placeholder-shown:before:border-transparent peer-placeholder-shown:after:border-transparent peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-gray-900 peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:border-gray-900 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:border-gray-900 peer-disabled:text-transparent peer-disabled:before:border-transparent peer-disabled:after:border-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                                    {"Editing Configuration"}
                                </label>
                            </div>
                        </div>
                        <div>
                            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 mt-3 rounded" onclick={generate_config_click}>{"Save Config"}</button>
                        </div>

                        {link_display}
                        </>
                        // </div>
                    // </div>
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

