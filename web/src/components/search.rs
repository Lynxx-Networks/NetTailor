use yew::{function_component, Html, html};
use yew::prelude::*;
use super::app_drawer::App_drawer;
use crate::requests::pod_req;
use yewdux::prelude::*;
use yew_router::history::BrowserHistory;
use crate::components::context::{AppState, UIState};
use crate::components::gen_funcs::{sanitize_html_with_blank_target, truncate_description, format_datetime, parse_date, DateFormat};
use crate::requests::pod_req::RecentEps;
use super::search_nav::Search_nav;
use crate::components::empties::empty_message;
// use crate::components::gen_funcs::check_auth;
use wasm_bindgen::closure::Closure;
use web_sys::{console, window};
use wasm_bindgen::JsCast;
use crate::requests::login_requests::use_check_authentication;
use crate::components::state_messages::UIStateMsg;


#[derive(Properties, Clone, PartialEq)]
pub struct SearchProps {
    pub on_search: Callback<String>,
}

#[function_component(Search)]
pub fn search(_props: &SearchProps) -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let effect_dispatch = dispatch.clone();
    let history = BrowserHistory::new();

    let input_ref = use_node_ref();
    let input_ref_clone1 = input_ref.clone();
    let input_ref_clone2 = input_ref.clone();
    let container_ref = use_node_ref();
    let container_ref_clone1 = container_ref.clone();
    let form_ref = NodeRef::default();
    let form_ref_clone1 = form_ref.clone();

    console::log_1(&format!("About to run check auth").into());
    // check_auth(effect_dispatch);

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
    
    let (post_state, _post_dispatch) = use_store::<AppState>();
    let (audio_state, audio_dispatch) = use_store::<UIState>();
    let error_message = audio_state.error_message.clone();
    let info_message = audio_state.info_message.clone();
    let loading = use_state(|| false);
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

    console::log_1(&format!("loading ep value: {:?}", *loading).into());

    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
    });


    html! {
        <>
        <div class="search-page-container flex flex-col">
            <div class="search-container" ref={container_ref.clone()}>
                <form class="search-page-input" onsubmit={on_submit} ref={form_ref.clone()}>
                    <div class="flex space-x-4">
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
                </form>
            </div>
        </div>
            <App_drawer />
            if let Some(error) = error_message {
                <div class="error-snackbar">{ error }</div>
            }
            if let Some(info) = info_message {
                <div class="info-snackbar">{ info }</div>
            }

        </>
    }

}