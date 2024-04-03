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
    #[at("/saved")]
    Saved,
    #[at("/settings")]
    Settings,
    #[at("/search")]
    Search,
    #[at("/user_stats")]
    UserStats,
    #[at("/create_config")]
    CreateConfig,
    #[at("/change_server")]
    ChangeServer,
    #[at("/sign_out")]
    LogOut,
}
