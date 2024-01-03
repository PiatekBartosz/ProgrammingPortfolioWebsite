use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-white rouded-lg shadow m-4 dark:bg-gray-800">
            <div class="w-full mx-auto max-w-screen-xl p-4 md:flex md:items-center md:justify-between">
                <span class="ext-sm text-gray-500 sm:text-center dark:text-gray-400">
                    "Made with Rust, Leptos & Tailwind CSS 🦀 © 2024."
                </span>
            </div>
        </footer>
    }
}