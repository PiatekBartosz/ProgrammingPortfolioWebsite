use crate::components::footer::Footer;
use crate::components::top_nav::TopNav;

use leptos::*;

pub fn Home() -> impl IntoView {
    view! {
        <body class="dark bg-gray-600">
            <TopNav>
            </TopNav>
                <h1 class="text-white"> "test" </h1>
            <Footer>
            </Footer>
        </body>
    }
}