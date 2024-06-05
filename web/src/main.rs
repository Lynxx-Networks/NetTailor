// Custom Mods
mod components;
mod requests;

use components::routes::Route;
use components::login::Login;
use components::login::ChangeServer;
use components::login::LogOut;
use components::saved::Saved;
use components::search::Search;
use components::settings::Settings;
use components::user_stats::UserStats;
use components::create_config::CreateConfig;
use components::edit_config::EditConfig;
use components::home::Home;

// Yew Imports
use yew_router::prelude::*;
use yew::prelude::*;


#[function_component(NotFound)]
pub fn not_found() -> Html {
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

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
        Route::CreateConfig => html! { <CreateConfig /> },
        Route::EditConfig => html! { <EditConfig /> },
        Route::ChangeServer => html! { <ChangeServer /> },
        Route::Saved => html! { <Saved /> },
        Route::Settings => html! { <Settings /> },
        Route::Search => html! { <Search /> },
        Route::UserStats => html! { <UserStats /> },
        Route::LogOut => html! { <LogOut /> },

    }
}


#[function_component(Main)]
fn main_component() -> Html {
    // console::log_1(&format!("Initial User Context: {:?}", (*user_context).clone()).into());
    // console::log_1(&format!("Initial Auth Context: {:?}", (*user_auth_context).clone()).into());

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}



fn main() {
    yew::Renderer::<Main>::new().render();
}