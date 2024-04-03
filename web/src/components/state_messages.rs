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
use std::rc::Rc;

pub enum UIStateMsg {
    ClearErrorMessage,
    ClearInfoMessage,
}

impl Reducer<UIState> for UIStateMsg {
    fn apply(self, mut state: Rc<UIState>) -> Rc<UIState> {
        let state = Rc::make_mut(&mut state);

        match self {
            UIStateMsg::ClearErrorMessage => {
                state.error_message = None;
            },
            UIStateMsg::ClearInfoMessage => {
                state.info_message = None;
            },
        }

        (*state).clone().into()
    }
}
