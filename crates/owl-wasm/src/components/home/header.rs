use web_sys::{console, js_sys::JsString};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::AppRoutes, hooks::use_user_context};

#[function_component(Header)]
pub fn header_view() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() {
        console::log_1(&JsString::from(format!("{}", user_ctx.access_token)));
        return html! { <Redirect<AppRoutes> to={AppRoutes::Login} /> };
    }

    html! {
        <div class="w-full flex flex-row items-center justify-center py-3">
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
