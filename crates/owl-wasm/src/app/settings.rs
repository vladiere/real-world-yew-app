use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    app::AppRoutes,
    hooks::use_user_context,
    services::{auth::*, get_refresh},
    types::{AdminUpdateInfo, AdminUpdateInfoWrapper, LogoutInfo, LogoutInfoWrapper},
};

#[function_component(SettingsPage)]
pub fn settings() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let update_admin = use_state(AdminUpdateInfo::default);
    let confirm_password = use_state(|| String::default());

    if !user_ctx.is_authenticated() {
        navigator.push(&AppRoutes::Login);
    }

    let admin_update = {
        let update_admin = update_admin.clone();
        let password = confirm_password.clone();
        use_async(async move {
            let mut request = AdminUpdateInfoWrapper {
                admin: (*update_admin).clone(),
            };
            if !(*password).is_empty() {
                request.admin.password = (*password).clone();
            }
            save(request).await
        })
    };

    let oninput_email = {
        let update_admin = update_admin.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_admin).clone();
            info.email_address = input.value();
            update_admin.set(info);
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

    let oninput_username = {
        let update_admin = update_admin.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_admin).clone();
            info.username = input.value();
            update_admin.set(info);
        })
    };

    let on_logout = { Callback::from(move |_| user_ctx.logout()) };

    html! {
        <div class="flex flex-col w-full h-full">
            <div class="flex flex-col mb-5">
                <h1 class="text-2xl font-bold">{ "Settings" }</h1>
            </div>
            <form>
                <div class="mb-6">
                    <label for="username" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Admin username" }</label>
                    <input type="text" id="username" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="e.g. admin123" required=true value={update_admin.username.clone()} oninput={oninput_username} />
                </div>
                <div class="mb-6">
                    <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Email address" }</label>
                    <input type="email" id="email" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="e.g. john.doe@company.com" required=true value={update_admin.email_address.clone()} oninput={oninput_email}/>
                </div>
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
                <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-10 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{ "Save changes" }</button>
            </form>
            <div class="flex flex-col gap-3 mt-3">
                <span class="text-8md font-bold">{ "OR" }</span>
                <button type="button" class="self-start px-10 py-2.5 text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700" onclick={on_logout}>{ "Logout" }</button>
            </div>
        </div>
    }
}
