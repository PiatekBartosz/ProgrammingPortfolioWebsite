use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mx-auto bg-gray-800 mt-auto max-w-[85rem] px-6 sm:px-8 lg:px- rounded-lg">
            <div class="container px-4 py-4 mx-auto items-center">
                <p class="text-center text-sm text-gray-400 sm:ml-4 sm:pl-4 sm:border-l-2 sm:border-gray-800 sm:py-2 sm:mt-0 mt-4">
                    "Made with Rust 🦀, Leptos & Tailwindcss ©2024 by Bartosz Piątek"
                </p>
            </div>
        </footer>
    }
}