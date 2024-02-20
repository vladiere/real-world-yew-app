use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="w-full h-full dark:bg-slate-900 dark:text-slate-50 flex flex-col items-center justify-center gap-10">
            <h1 class="text-8xl">{ "404" }</h1>
            <h1 class="text-4xl">{ "The page you are looking for is not found." }</h1>
        </div>
    }
}
