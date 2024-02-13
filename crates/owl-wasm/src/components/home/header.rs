use web_sys::{console, js_sys::JsString};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::AppRoutes, services::get_access};

#[function_component(Header)]
pub fn header_view() -> Html {
    let navigator = use_navigator().unwrap();

    if !get_access().is_some() {
        navigator.push(&AppRoutes::Login);
    }

    html! {
        <div class="w-full flex flex-row items-center justify-center py-3 border-b-2 border-slate-400">
            <div class="flex-auto items-center justify-end w-[60%] flex flex-row gap-2">
                <img src="public/images/owl-logo.png" class="w-[10%] h-[5%]" alt="OWL logo" />
                <h1 class="text-2xl font-medium uppercase">{ "owl web server" }</h1>
            </div>
            <div class="flex-auto justify-center items-end w-[40%] flex flex-row gap-2">
                <img src="public/icons/admin.svg" class="text-white w-[5%] h-[5%]" alt="ADMIN" />
                <span class="font-medium text-8md capitalize">{ "hi, jude cadavez" }</span>
            </div>
        </div>
    }
}
