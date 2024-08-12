use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to My Yew App" }</h1>
            <Home />
        </div>
    }
}
