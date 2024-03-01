use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoutes;
use crate::components::list_errors::ListErrors;
use crate::hooks::use_user_context;
use crate::services::auth::*;
use crate::types::{LoginInfo, LoginInfoWrapper};

/// Login page
#[function_component(Login)]
pub fn login_page() -> Html {
    let navigator = use_navigator().unwrap();
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);

    if user_ctx.is_authenticated() {
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
        <div class="flex h-screen text-black">
            <div class="hidden lg:flex items-center justify-center flex-1 bg-white dark:bg-slate-200">
                <div class="max-w-md text-center">
                    <img src="public/images/login-bg.png" alt="Admin at work" />
                    <p class="text-md font-medium text-gray-700 dark:text-slate-500">{ "Don't sleep when you are tired, sleep when you are done." }
                        <span class="font-bold text-4md text-blue-700 dark:text-blue-500">{ " Johnny sins" }</span>
                    </p>
                </div>
            </div>
            <div class="w-full bg-gray-100 lg:w-2/6 flex items-center justify-center dark:bg-gray-900 dark:text-white">
                <div class="max-w-md w-full p-6">
                    <div class="w-full flex items-center justify-center">
                        <img src="public/images/owl-logo.png" class="w-[30%]" alt="owl logo" />
                    </div>
                    <h1 class="text-3xl font-semibold mb-6 text-center">{ "Signin to OWL" }</h1>
                    <ListErrors error={user_login.error.clone()} />
                    <form class="space-y-4" {onsubmit}>
                        <div>
                            <label class="block text-sm font-medium text-gray-500" for="username">{ "Username" }</label>
                            <input id="username" type="text" class="mt-1 p-2 w-full text-gray-900 border rounded-md focus:border-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-300 duration-300" required=true placeholder="e.g. john123" value={login_info.username.clone()} oninput={oninput_username}/>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-500" for="password">{ "Password" }</label>
                            <input id="password" type="password" class="mt-1 p-2 w-full text-gray-900 border rounded-md focus:border-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-300 duration-300" required=true placeholder="************" value={login_info.password.clone()} oninput={oninput_password}/>
                        </div>
                        <div>
                            <button type="submit" class="my-4 w-full bg-black text-white p-2 rounded-md hover:bg-gray-800 focus:outline-none focus:bg-black focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900 transition-colors duration-300 dark:bg-blue-700 dark:text-white">{ "Signin" }</button>
                            <Link<AppRoutes> to={AppRoutes::ForgotPassword} classes="text-6md font-medium text-[#16B1BB] hover:underline hover:italic">
                                { "Forgot Password?" }
                            </Link<AppRoutes>>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
