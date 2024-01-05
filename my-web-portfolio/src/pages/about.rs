use crate::components::footer::Footer;
use crate::components::top_nav::TopNav;

use leptos::*;

pub fn About() -> impl IntoView {
    view! {
        <body class="dark bg-gray-600">
            <TopNav/>
                // <h1 class="text-white"> "About page" </h1>
            <Footer/>
        </body>
    }
}