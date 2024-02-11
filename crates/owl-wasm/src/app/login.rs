use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoutes;
use crate::components::list_errors::ListErrors;
use crate::components::owl_logo::OwlLogo;
use crate::hooks::use_user_context;
use crate::services::{auth::*, get_access};
use crate::types::{LoginInfo, LoginInfoWrapper};

/// Login page
#[function_component(Login)]
pub fn login_page() -> Html {
    let navigator = use_navigator().unwrap();
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    let btn_state = use_state(|| false);

    if get_access().is_some() {
        navigator.push(&AppRoutes::Home);
    }

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
        let btn_state = btn_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            btn_state.set(true);
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
                    { if *btn_state {
                        html! {
                            <button disabled=true type="button" class="text-gray-800 focus:ring-4 focus:outline-none font-medium text-lg w-full sm:w-auto px-10 py-1.5 text-center bg-[#16B1BB] hover:bg-[#118A92] hover:text-gray-200 focus:ring-[#16B1BB]">
                                <svg aria-hidden="true" role="status" class="inline w-4 h-4 me-3 text-white animate-spin" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="#E5E7EB"/>
                                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentColor"/>
                                </svg>
                                {"Loading..."}
                            </button>
                        }
                    } else {
                        html! {
                            <button type="submit" disabled=false class="text-gray-800 focus:ring-4 focus:outline-none font-medium text-lg w-full sm:w-auto px-10 py-1.5 text-center bg-[#16B1BB] hover:bg-[#118A92] hover:text-gray-200 focus:ring-[#16B1BB]"> { "Login" } </button>
                        }
                    }}
                </div>
            </form>
        </div>
    }
}
