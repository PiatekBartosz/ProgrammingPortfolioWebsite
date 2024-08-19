use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex items-center justify-center min-h-screen max-w-screen-xl px-4 pt-20 pb-8 mx-auto">
           <div class="text-center">
                <h1 class="max-w-2xl mb-4 text-4xl font-extrabold leading-none tracking-tight text-white">{ "Hi, I'm Bartek!" }</h1>
                <div class="mt-8 text-lg text-white">
                    <p>{ "🇵🇱 Software Engineer from Poland" }</p>
                    <p>{ "💻 Intern in Embedded Software Engineering" }</p>
                    <p>{ "🎓 Automatic Control and Robotics Undergrad Student" }</p>
                    <p>{ "⚙️ Member of Control Engineers Science Club" }</p>
                    <p>
                        <span>{ "💼 " }</span>
                        <a href="https://www.linkedin.com/in/bartoszpiatek" target="_blank" class="text-blue-500 hover:underline">
                            { "LinkedIn" }
                        </a>
                    </p>
                    <p>{ "🚀 Vim motion enjoyer" }</p>
                </div>
            </div>
        </div>
    }
}
