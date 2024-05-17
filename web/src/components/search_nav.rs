use std::rc::Rc;
use yew::prelude::*;
use yew_router::history::BrowserHistory;
use yewdux::prelude::*;
use crate::components::context::AppState;


#[allow(non_camel_case_types)]
#[function_component(Search_nav)]
pub fn search_bar() -> Html {
    let history = BrowserHistory::new();
    let dispatch = Dispatch::<AppState>::global();
    let state: Rc<AppState> = dispatch.get();
    let podcast_value = use_state(|| "".to_string());
    let search_index = use_state(|| "podcast_index".to_string()); // Default to "podcast_index"
    let (_app_state, dispatch) = use_store::<AppState>();

    let history_clone = history.clone();



    html! {
    <div class="episodes-container w-full search-background"> // Ensure full width and set background color
    </div>
}
}