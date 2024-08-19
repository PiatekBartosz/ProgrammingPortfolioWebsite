use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use crate::pages::dsa::Dsa;
use crate::pages::home::Home;
use crate::pages::projects::Projects;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/dsa")]
    Dsa,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Projects => html! {
            html! { <Projects /> }
        },
        Route::Dsa => html! {
            html! { <Dsa /> }
        },
        Route::NotFound => {
            html! { <h1 class="max-w-2xl mb-4 text-4xl font-extrabold leading-none tracking-tight text-white">{ "404 - Page not found" }</h1> }
        }
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
                <Switch<Route> render={switch} />
            <Footer />
        </BrowserRouter>
    }
}
