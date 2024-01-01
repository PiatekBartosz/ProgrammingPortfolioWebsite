use leptos::*;

#[component]
pub fn TopNav() -> impl IntoView {
    view! {
        <div class="topnav">
            <a href="/">"Home"</a>
            <a href="/">"About"</a>
            <a href="/">"Projects"</a>
            <a href="/">"Data Structures & Algorithms"</a>
        </div>
    }
}