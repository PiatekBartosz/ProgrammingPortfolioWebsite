use crate::components::footer::Footer;
use crate::components::topnav::TopNav;

use leptos::*;
use leptos_meta::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        // <Html lang="en" class="h-full"/>
        // <Title text="My Web Portfolio"/>
        // <Meta name="description" content="My Web Portfolio"/>
        <body>
            <TopNav/>
            <h1 class="text-white"> "About page" </h1>
            <Footer/>
        </body>
    }
}