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

    let is_loading = use_state(|| false);

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

    let is_load = is_loading.clone();

    use_effect_with(user_login.clone(), move |user_login| {
        if let Some(user_info) = &user_login.data {
            is_load.set(false);
            user_ctx.login(user_info.admin.clone());
        }

        if let Some(_) = &user_login.error {
            is_load.set(false);
        }
        || ()
    });

    let onsubmit = {
        let user_login = user_login.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            is_loading.set(true);
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
                            <button type="submit" disabled={(*is_loading).clone()} class="mt-4 my-4 w-full bg-black text-white p-2 rounded-md hover:bg-gray-800 focus:outline-none focus:bg-black focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900 transition-colors duration-300 dark:bg-blue-700 dark:text-white flex items-center justify-center">
                                if !(*is_loading).clone() {
                                    { "Signin" }
                                } else {
                                        <svg aria-hidden="true" class="w-6 h-6 text-gray-200 animate-spin dark:text-gray-600 fill-white" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                                            <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                                            <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                                        </svg>
                                }
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
