use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::history::{BrowserHistory, History};
use web_sys::{window, HtmlInputElement, MouseEvent};
use yewdux::prelude::*;
use crate::components::context::{AppState, UIState};
use yew::Callback;
use crate::requests::pod_req::{call_download_episode, call_queue_episode, call_save_episode, DownloadEpisodeRequest, Episode, EpisodeDownload, HistoryEpisode, QueuePodcastRequest, QueuedEpisode, SavePodcastRequest, SavedEpisode, call_remove_downloaded_episode, call_remove_queued_episode, call_remove_saved_episode};
use crate::requests::search_pods::SearchEpisode;
use std::any::Any;
use crate::components::gen_funcs::format_time;


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