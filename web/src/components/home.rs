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


#[function_component(Home)]
pub fn home() -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let effect_dispatch = dispatch.clone();
    let history = BrowserHistory::new();

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
    let loading = use_state(|| true);
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

    console::log_1(&format!("loading ep value: {:?}", *loading).into());

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
                            empty_message(
                                "Sometimes I feel like",
                                "Somebody's watchin me"
                            )
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