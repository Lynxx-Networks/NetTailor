use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::context::AppState;
use yew::platform::spawn_local;
use web_sys::console;
use crate::requests::setting_reqs::{call_mfa_settings, call_generate_mfa_secret, call_verify_temp_mfa};
use std::borrow::Borrow;
use wasm_bindgen::JsValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}