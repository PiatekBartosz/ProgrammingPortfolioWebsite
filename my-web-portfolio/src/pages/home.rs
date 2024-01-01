use crate::components::footer::Footer;
use crate::components::top_nav::TopNav;

use leptos::*;

pub fn Home() -> impl IntoView {
    view! {
        <body>
        <h1>"Welcome to Leptos!"</h1>
        <TopNav>
        </TopNav>
        </body>
        <Footer>
        </Footer>
    }
}