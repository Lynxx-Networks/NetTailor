
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::history::{BrowserHistory, History};
use web_sys::{window, HtmlInputElement, MouseEvent};
use yewdux::prelude::*;
use crate::components::context::{AppState, UIState};
use yew::Callback;
use std::any::Any;
use crate::components::gen_funcs::format_time;

pub fn empty_message(header: &str, paragraph: &str) -> Html {
    html! {
        <div class="empty-episodes-container">
            <img src="static/assets/favicon.png" alt="Logo" class="logo"/>
            <h1 class="page-paragraphs">{ header }</h1>
            <p class="page-paragraphs">{ paragraph }</p>
        </div>
    }
}