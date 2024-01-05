use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::projects::Projects;
use crate::pages::dsa::DSA;
use crate::pages::hire::Hire;

use crate::components::footer::Footer;
use crate::components::topnav::TopNav;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en"/>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="My Web Portfolio"/>

        // content for this welcome page
        <Router>
            <Routes>
                <Route path="" view=Home/>
                <Route path="/about" view=About/>
                <Route path="/projects" view=Projects/>
                <Route path="/dsa" view=DSA/>
                <Route path="/hire" view=Hire/>
                <Route path="/*any" view=NotFound/>
            </Routes>
        </Router>
    }
}

// / Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let (count, set_count) = create_signal(0);
//     let on_click = move |_| set_count.update(|count| *count += 1);

//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <button on:click=on_click>"Click Me: " {count}</button>
//     }
// }

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Html lang="en" class="h-full"/>
        <head>
            <Title text="My Web Portfolio"/>
        </head>

        <body class="bg-gray-900 h-5/6">
            <TopNav/>
            
            <div class="bg-gray-900 flex flex-col justify-center mx-auto w-full h-full">
            
                <div class="text-center py-10 px-4 sm:px-6 lg:px-8">
                    <h1 class="text-3xl font-extrabold text-white sm:text-4xl">
                        "404 - Not Found"
                    </h1>
                    <p class="text-white text-m text-center">
                        "The page you are looking for does not exist."
                    </p>
                    <div class="w-1/4 mt-5 mx-auto flex flex-col justify-center gap-2 sm:flex-row sm:gap-3">
                        <a href="/" class="mx-auto flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-700 hover:bg-blue-800 md:py-4 md:text-lg md:px-10">
                            "Go Home"
                        </a>
                    </div>
                </div>

                <Footer/>
            </div>
        </body>
    }
}