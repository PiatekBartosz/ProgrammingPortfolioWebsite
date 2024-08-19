use crate::app::Route;
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
    html! {
      <nav class="bg-gray-700 shadow-md">
      <div class="container mx-auto px-4 py-4 flex justify-between items-center">
        <div class="text-2xl font-bold text-white transition-all duration-300 hover:text-yellow-500">
            <Link<Route> to={Route::Home} >
              {"My Porfolio Website"}
            </Link<Route>>
         </div>
        <ul class="flex space-x-6">
          <li class="text-white bold bg-transparent border-none cursor-pointer transition-all duration-300 hover:text-yellow-500">
            <Link<Route> to={Route::Home} >
              {"Home"}
            </Link<Route>>
          </li>
          <li class="text-white bold bg-transparent border-none cursor-pointer transition-all duration-300 hover:text-yellow-500">
            <Link<Route> to={Route::Projects} >
              {"Projects"}
            </Link<Route>>
          </li>
          <li class="text-white bold bg-transparent border-none cursor-pointer transition-all duration-300 hover:text-yellow-500">
            <Link<Route> to={Route::Dsa} >
              {"Data structres and algorithms"}
            </Link<Route>>
          </li>
        </ul>
      </div>
    </nav>
    }
}
