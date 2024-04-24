use yew::prelude::*;
use yewdux::use_store;
use super::app_drawer::App_drawer;
use super::search_nav::Search_nav;
use yew_router::history::BrowserHistory;
use crate::components::context::{AppState, UIState};
use crate::components::misc_func::generate_config;

#[function_component(CreateConfig)]
pub fn create_config() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let effect_dispatch = dispatch.clone();
    let history = BrowserHistory::new();


    let (post_state, _post_dispatch) = use_store::<AppState>();
    let (audio_state, audio_dispatch) = use_store::<UIState>();
    let error_message = audio_state.error_message.clone();
    let info_message = audio_state.info_message.clone();
    let loading = use_state(|| true);
 
    let os_version = use_state(|| String::from(""));
    let hostname = use_state(|| String::from(""));
    let tacacs_server = use_state(|| String::from(""));
    let ise_server = use_state(|| String::from(""));
    let timezone = use_state(|| String::from(""));
    let boot_bin = use_state(|| String::from(""));
    let switch_number = use_state(|| String::from(""));
    let model = use_state(|| String::from(""));
    let vtp_domain = use_state(|| String::from(""));
    let auvik_collector = use_state(|| String::from(""));
    let crypto_auth_block = use_state(|| String::from(""));
    let vlan_range = use_state(|| String::from(""));
    let encrypted_user_pass = use_state(|| String::from(""));
    let vlan_definitions = use_state(|| String::from(""));
    let class_map_definitions = use_state(|| String::from(""));
    let policy_map_definitions = use_state(|| String::from(""));


    let configuration = use_state(|| String::new());

    // Adjust the generate_config_click to accept a MouseEvent parameter
    let generate_config_click = {
        let configuration = configuration.clone();
        let os_version = os_version.clone();
        let hostname = hostname.clone();
        let tacacs_server = tacacs_server.clone();
        let ise_server = ise_server.clone();
        let timezone = timezone.clone();
        let boot_bin = boot_bin.clone();
        let switch_number = switch_number.clone();
        let model = model.clone();
        let vtp_domain = vtp_domain.clone();
        let auvik_collector = auvik_collector.clone();
        let crypto_auth_block = crypto_auth_block.clone();
        let vlan_range = vlan_range.clone();
        let encrypted_user_pass = encrypted_user_pass.clone();
        let vlan_definitions = vlan_definitions.clone();
        let class_map_definitions = class_map_definitions.clone();
        let policy_map_definitions = policy_map_definitions.clone();

        // Define the closure correctly to take a MouseEvent argument
        Callback::from(move |_: MouseEvent| { // Add _: MouseEvent to the closure parameters
            let config = generate_config(
                &os_version,
                &hostname,
                &tacacs_server,
                &ise_server,
                &timezone,
                &boot_bin,
                &switch_number,
                &model,
                &vtp_domain,
                &auvik_collector,
                &crypto_auth_block,
                &vlan_range,
                &encrypted_user_pass,
                &vlan_definitions,
                &class_map_definitions,
                &policy_map_definitions
            );
            configuration.set(config.clone());
            web_sys::console::log_1(&format!("Config: {}", config).into());
            web_sys::console::log_1(&format!("Config: {}", *configuration).into());
        })
    };

    html! {
        <>
        <div class="main-container">
            <Search_nav />
            {
                html! {
                    // <div class="flex flex-col space-y-4 w-full max-w-xs p-8 border border-gray-300 rounded-lg shadow-lg bg-gray-600">

                        <div class="container mx-auto text-center p-10">
                            <div class="flex justify-center items-center">
                                <img class="object-scale-down h-20 w-66" src="static/assets/favicon.png" alt="NetTailor Logo" />
                            </div>
                            <h1 class="item_container-text text-4xl font-bold mb-5">{"Generate New Configuration"}</h1>
                            <div>
                            <textarea class="config-output" value={(*configuration).clone()} readonly=true />
                            <button onclick={generate_config_click}>{"Generate Config"}</button>
                            </div>
                        </div>
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