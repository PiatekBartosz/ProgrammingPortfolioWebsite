use crate::components::footer::Footer;
use crate::components::topnav::TopNav;

use leptos::*;

pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <body>
            <TopNav/>
                <h1>"Home page"</h1>
                <h1>"Welcome to Leptos!"</h1>
                <button on:click=on_click class="btn btn-blue">"Click Me: " {count}</button>
            <Footer/>
        </body>
    }
}