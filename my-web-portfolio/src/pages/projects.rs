use crate::components::footer::Footer;
use crate::components::topnav::TopNav;

use leptos::*;
use leptos_meta::*;

pub fn Projects() -> impl IntoView {

    view! {
        <Html lang="en" class="h-full"/>
        <head>
            <Title text="My Web Portfolio"/>
        </head>

        <body class="bg-gray-900 h-5/6">
            <TopNav/>
            
            <div class="bg-gray-900 flex flex-col justify-center mx-auto w-full h-full">
            
                <div class="text-center py-10 px-4 sm:px-6 lg:px-8">
                    <h1 class="text-3xl font-extrabold text-white sm:text-4xl">
                        "My Projects"
                    </h1>
                    <p class="text-white text-m text-center">
                        "TODO"
                    </p>
                </div>

                <Footer/>
            </div>
        </body>
    }
}