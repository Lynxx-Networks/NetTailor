use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::use_store;
use crate::components::context::AppState;
use super::routes::Route;


#[function_component(CreateConfig)]
pub fn create_config() -> Html {
        html! {
            <>
                <div class="empty-episodes-container">
                    <img src="static/assets/favicon.png" alt="Logo" class="logo"/>
                    <h1>{ "Page not found" }</h1>
                    <p>{"Sorry for the inconvenience. You could eat a taco to cheer you up :)"}</p>
                </div>
            </>
        }

}