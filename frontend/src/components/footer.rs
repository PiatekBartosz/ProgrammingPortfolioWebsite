use yew::{function_component, html, Html};

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer class="flex shadow-md items-center justify-center w-full h-16 fixed bottom-0 bg-gray-800">
            <p class="font-light text-gray-400">
                {"This website is written in Rust 🦀"}
            </p>
        </footer>
    }
}
