use crate::components::footer::Footer;
use crate::components::top_nav::TopNav;

use leptos::*;

pub fn Home() -> impl IntoView {
    view! {
        <body class="dark">
            <TopNav>
            </TopNav>



            <Footer>
            </Footer>
        </body>
    }
}