use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/home")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/queue")]
    Queue,
    #[at("/saved")]
    Saved,
    #[at("/settings")]
    Settings,
    #[at("/history")]
    PodHistory,
    #[at("/downloads")]
    Downloads,
    #[at("/search")]
    Search,
    #[at("/user_stats")]
    UserStats,
    #[at("/sign_out")]
    LogOut,
    #[at("/pod_layout")]
    PodLayout,
    #[at("/search_new")]
    SearchNew,
    #[at("/podcasts")]
    Podcasts,
    #[at("/episode_layout")]
    EpisodeLayout,
    #[at("/episode")]
    Episode,
}
