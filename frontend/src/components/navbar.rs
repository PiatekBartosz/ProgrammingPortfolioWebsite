use yew::{function_component, html, Html};

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav class="bg-gray shadow-md">
        <div class="container mx-auto px-4 py-4 flex justify-between items-center">
          <div class="text-2xl font-bold text-gray-800"> {"MyWebsite"} </div>
          <ul class="flex space-x-6">
            <li>
              <a href="#" class="text-gray-800 hover:text-blue-500 transition duration-300">{"Home"}</a>
            </li>
            <li>
              <a href="#" class="text-gray-800 hover:text-blue-500 transition duration-300">{"About"}</a>
            </li>
            <li>
              <a href="#" class="text-gray-800 hover:text-blue-500 transition duration-300">{"Others"}</a>
            </li>
          </ul>
        </div>
      </nav>
    }
}
