use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoutes;
use crate::components::list_errors::ListErrors;
use crate::components::owl_logo::OwlLogo;
use crate::hooks::use_user_context;
use crate::services::auth::*;
use crate::types::{LoginInfo, LoginInfoWrapper};

/// Login page
#[function_component(Login)]
pub fn login_page() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    let user_login = {
        let login_info = login_info.clone();
        use_async(async move {
            let request = LoginInfoWrapper {
                user: (*login_info).clone(),
            };
            login(request).await
        })
    };

    use_effect_with(user_login.clone(), move |user_login| {
        if let Some(user_info) = &user_login.data {
            user_ctx.login(user_info.admin.clone());
        }
        || ()
    });

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            user_login.run();
        })
    };
    let oninput_username = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.username = input.value();
            login_info.set(info);
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div class="h-full w-full flex flex-col items-center justify-center">
            <form class="w-[25%] flex flex-col gap-2" {onsubmit}>
                <div class="flex flex-col items-center justify-center">
                    <OwlLogo img_class={"w-[60%] rounded-[50%]"} />
                    <span class="font-bold text-[32px] text-[#16B1BB] mb-2"> { "Login to OWL ADMIN" } </span>
                    <ListErrors error={user_login.error.clone()} />
                </div>
                <div class="flex flex-col gap-2">
                    <div class="mb-6">
                        <label for="username" class="block mb-2 text-sm font-medium text-[#16B1BB]"> { "Username" } </label>
                        <input type="text" id="username" class="bg-transparent border-0 border-b-2 outline-none border text-sm focus:border-[#16B1BB] block w-full p-2.5 border-[#16B1BB] placeholder-[#16B1BB] text-[#16B1BB] focus:ring-[#16B1BB] focus:border-[#16B1BB]" placeholder="e.g admin123" required=true value={login_info.username.clone()} oninput={oninput_username} />
                    </div>
                    <div>
                        <label for="password" class="block mb-2 text-sm font-medium text-[#16B1BB]"> { "Password" } </label>
                        <input type="password" id="password" class="bg-transparent border-0 border-b-2 outline-none border border-[#16B1BB] text-[#16B1BB] text-sm focus:border-[#16B1BB] block w-full p-2.5 border-[#16B1BB] placeholder-[#16B1BB] dark:focus:ring-[#16B1BB] focus:border-[#16B1BB]" placeholder="•••••••••" required=true value={login_info.password.clone()} oninput={oninput_password} />
                    </div>
                </div>
                <div class="flex flex-col items-end gap-5">
                    <p class="text-6md font-medium text-[#16B1BB] hover:underline hover:italic">
                        <Link<AppRoutes> to={AppRoutes::ForgotPassword}>
                            { "Forgot Password?" }
                        </Link<AppRoutes>>
                    </p>
                    <button type="submit" disabled=false class="text-gray-800 focus:ring-4 focus:outline-none font-medium text-lg w-full sm:w-auto px-10 py-1.5 text-center bg-[#16B1BB] hover:bg-[#118A92] hover:text-gray-200 focus:ring-[#16B1BB]"> { "Login" } </button>
                </div>
            </form>
        </div>
    }
}
