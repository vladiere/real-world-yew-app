use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    app::AppRoutes,
    components::alerts::*,
    hooks::use_user_context,
    services::{auth::*, get_refresh},
    types::{AdminUpdateInfo, LogoutInfo, LogoutInfoWrapper},
};

#[function_component(SettingsPage)]
pub fn settings() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let update_admin = use_state(AdminUpdateInfo::default);
    let confirm_password = use_state(|| String::default());
    let message = use_state(|| String::default());

    let is_loading = use_state(|| false);

    if !user_ctx.is_authenticated() {
        navigator.push(&AppRoutes::Login);
    }

    let admin_update = {
        let update_admin = update_admin.clone();
        let password = confirm_password.clone();
        use_async(async move {
            let mut request = (*update_admin).clone();
            if !(*password).is_empty() {
                request.password = (*password).clone();
            }
            save(request).await
        })
    };

    let msg = message.clone();
    let is_load = is_loading.clone();
    let admin_pass = update_admin.clone();
    let new_pass = confirm_password.clone();

    use_effect_with(admin_update.clone(), move |res_update| {
        if let Some(result_update) = &res_update.data {
            is_load.set(false);
            admin_pass.set(Default::default());
            new_pass.set(String::default());
            msg.set(result_update.message.clone())
        }

        if let Some(_) = &res_update.error {
            is_load.set(false);
        }
        || ()
    });

    let onsubmit = {
        let is_loading = is_loading.clone();
        let admin_update = admin_update.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            is_loading.set(true);
            admin_update.run();
        })
    };

    let oninput_password = {
        let update_admin = update_admin.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_admin).clone();
            info.password = input.value();
            update_admin.set(info);
        })
    };

    let oninput_confirm_password = {
        let confirm_password = confirm_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            confirm_password.set(input.value());
        })
    };

    let username = (*user_ctx).clone().username;

    let user_logout = {
        let token = if let Some(refresh_token) = get_refresh() {
            refresh_token
        } else {
            "".to_string()
        };

        use_async(async move {
            let logout_info = LogoutInfo {
                refresh_token: token,
                username,
            };
            let admin_info = LogoutInfoWrapper { admin: logout_info };
            logout_admin(admin_info).await
        })
    };

    let ctx_clone = user_ctx.clone();

    use_effect_with(user_logout.clone(), move |user_logout| {
        if let Some(_logout_info) = &user_logout.data {
            ctx_clone.logout();
        }
        || ()
    });

    let on_logout = {
        let user_logout = user_logout.clone();
        Callback::from(move |_| user_logout.run())
    };

    html! {
        <div class="flex flex-col w-full h-full">
            <DangerAlert error={admin_update.error.clone()} />
            <SuccessAlert alert_msg={(*message).clone()} />
            <div class="flex flex-col mb-5">
                <h1 class="text-2xl font-bold">{ "Settings" }</h1>
            </div>
            <form {onsubmit}>
                <div class="mb-6">
                    <label for="password" class={
                            if !(*confirm_password).clone().is_empty() && !(*update_admin).password.clone().is_empty() {
                                if *update_admin.password.clone() != *confirm_password.clone() {
                                    "block mb-2 text-sm font-medium text-red-700 dark:text-red-500"
                                } else {
                                    "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                }
                            } else {
                                "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            }
                        }>{ "Password" }</label>
                    <input type="password" id="password" class={
                            if !(*confirm_password).clone().is_empty() && !(*update_admin).password.clone().is_empty() {
                                if *update_admin.password.clone() != *confirm_password.clone() {
                                    "bg-red-50 border border-red-500 outline-none text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 dark:bg-gray-700 focus:border-red-500 block w-full p-2.5 dark:text-red-500 dark:placeholder-red-500 dark:border-red-500"
                                } else {
                                    "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                }
                            } else {
                                "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            }
                        } placeholder="•••••••••" required=true value={update_admin.password.clone()} oninput={oninput_password}/>
                    {
                        if !(*confirm_password).clone().is_empty() && !(*update_admin).password.clone().is_empty() {
                            if *update_admin.password.clone() != *confirm_password.clone() {
                                html! { <p class="mt-2 text-sm text-red-600 dark:text-red-500">{ "Password does not match" }</p> }
                            } else {
                                html! {}
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div class="mb-6">
                    <label for="confirm_password" class={
                        if !(*confirm_password).clone().is_empty() && !(*update_admin).password.clone().is_empty() {
                            if *update_admin.password.clone() != *confirm_password.clone() {
                                "block mb-2 text-sm font-medium text-red-700 dark:text-red-500"
                            } else {
                                "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            }
                        } else {
                            "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        }
                    }>{ "Confirm password" }</label>
                    <input type="password" id="confirm_password" class={
                            if !(*confirm_password).clone().is_empty() && !(*update_admin).password.clone().is_empty() {
                                if *update_admin.password.clone() != *confirm_password.clone() {
                                    "bg-red-50 border border-red-500 outline-none text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 dark:bg-gray-700 focus:border-red-500 block w-full p-2.5 dark:text-red-500 dark:placeholder-red-500 dark:border-red-500"
                                } else {
                                    "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                }
                            } else {
                                "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            }
                        } placeholder="•••••••••" required=true value={(*confirm_password).clone()} oninput={oninput_confirm_password}/>
                    {
                        if !(*confirm_password).clone().is_empty() && !(*update_admin).password.clone().is_empty() {
                            if *update_admin.password.clone() != *confirm_password.clone() {
                                html! { <p class="mt-2 text-sm text-red-600 dark:text-red-500">{ "Password does not match" }</p> }
                            } else {
                                html! {}
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                {
                    if !(*is_loading).clone() {
                        html! {
                            <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-10 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{ "Save changes" }</button>
                        }
                    } else {
                        html! {}
                    }
                }
            </form>
            {
                if !(*is_loading).clone() {
                    html! {
                        <div class="flex flex-col gap-3 mt-3">
                            <span class="text-8md font-bold">{ "OR" }</span>
                            <button type="button" class="self-start px-10 py-2.5 text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700" onclick={on_logout}>{ "Logout" }</button>
                        </div>
                    }
                } else {
                    html! {
                        <div role="status" class="flex justify-start">
                            <svg aria-hidden="true" class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                            </svg>
                            <span class="sr-only">{ "Loading..." }</span>
                        </div>
                    }
                }
            }
        </div>
    }
}
