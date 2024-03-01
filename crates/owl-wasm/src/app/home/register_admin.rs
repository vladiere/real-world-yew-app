use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::{use_async, use_interval};
use yew_router::prelude::*;

use crate::{
    app::AppRoutes, components::alerts::*, hooks::use_user_context, services::auth::register_admin,
    types::*,
};

#[function_component(RegisterAdmin)]
pub fn registration_admin() -> Html {
    let admin_info = use_state(AdminRegisterInfo::default);
    let confirm_password = use_state(|| String::default());
    let message = use_state(|| String::default());

    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    if &user_ctx.role_user != "Super" {
        navigator.push(&AppRoutes::Home);
    }

    let admin_register = {
        let admin_info = admin_info.clone();
        use_async(async move {
            let request = AdminRegisterInfoWrapper {
                admin: (*admin_info).clone(),
            };
            register_admin(request).await
        })
    };

    let msg = message.clone();

    use_effect_with(admin_register.clone(), move |admin_reg| {
        if let Some(reg_data) = &admin_reg.data {
            msg.set(reg_data.message.clone());
        }
        || ()
    });

    let onsubmit = {
        let admin_register = admin_register.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            admin_register.run();
        })
    };

    let oninput_firstname = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.firstname = input.value();
            admin_info.set(info);
        })
    };

    let oninput_middlename = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.middlename = input.value();
            admin_info.set(info);
        })
    };

    let oninput_lastname = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.lastname = input.value();
            admin_info.set(info);
        })
    };

    let oninput_email = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.email_address = input.value();
            admin_info.set(info);
        })
    };

    let oninput_username = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.username = input.value();
            admin_info.set(info);
        })
    };

    let oninput_password = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.password = input.value();
            info.role_user = "Admin".to_string();
            admin_info.set(info);
        })
    };

    let oninput_confirm_password = {
        let confirm_password = confirm_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            confirm_password.set(input.value());
        })
    };

    html! {
        <div class="flex flex-col gap-5">
            <DangerAlert error={admin_register.error.clone()} />
            <SuccessAlert alert_msg={(*message).clone()} />
            <h1 class="text-2xl font-bold">{ "Admin Registration" }</h1>
            <form {onsubmit}>
                <div class="grid gap-6 mb-6 md:grid-cols-2">
                    <div>
                        <label for="first_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "First name" }</label>
                        <input type="text" id="first_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" required=true value={admin_info.firstname.clone()} oninput={oninput_firstname} />
                    </div>
                    <div>
                        <label for="middle_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Middle name" }</label>
                        <input type="text" id="middle_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin middle name" required=true value={admin_info.middlename.clone()} oninput={oninput_middlename} />
                    </div>
                    <div>
                        <label for="last_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Last name" }</label>
                        <input type="text" id="last_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin last name" required=true value={admin_info.lastname.clone()} oninput={oninput_lastname} />
                    </div>
                    <div>
                        <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Email address" }</label>
                        <input type="email" id="email" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin email address" required=true value={admin_info.email_address.clone()} oninput={oninput_email} />
                    </div>
                </div>
                <div class="mb-6">
                    <label for="username" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Username" }</label>
                    <input type="text" id="username" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin username" required=true value={admin_info.username.clone()} oninput={oninput_username} />
                </div>
                <div class="mb-6">
                    <label for="password" class={
                            if !(*confirm_password).clone().is_empty() && !(*admin_info).password.clone().is_empty() {
                                if *admin_info.password.clone() != *confirm_password.clone() {
                                    "block mb-2 text-sm font-medium text-red-700 dark:text-red-500"
                                } else {
                                    "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                }
                            } else {
                                "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            }
                        }>{ "Password" }</label>
                    <input type="password" id="password" class={
                            if !(*confirm_password).clone().is_empty() && !(*admin_info).password.clone().is_empty() {
                                if *admin_info.password.clone() != *confirm_password.clone() {
                                    "bg-red-50 border border-red-500 outline-none text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 dark:bg-gray-700 focus:border-red-500 block w-full p-2.5 dark:text-red-500 dark:placeholder-red-500 dark:border-red-500"
                                } else {
                                    "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                }
                            } else {
                                "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            }
                        } placeholder="•••••••••" required=true value={admin_info.password.clone()} oninput={oninput_password}/>
                    {
                        if !(*confirm_password).clone().is_empty() && !(*admin_info).password.clone().is_empty() {
                            if *admin_info.password.clone() != *confirm_password.clone() {
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
                        if !(*confirm_password).clone().is_empty() && !(*admin_info).password.clone().is_empty() {
                            if *admin_info.password.clone() != *confirm_password.clone() {
                                "block mb-2 text-sm font-medium text-red-700 dark:text-red-500"
                            } else {
                                "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            }
                        } else {
                            "block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        }
                    }>{ "Confirm password" }</label>
                    <input type="password" id="confirm_password" class={
                            if !(*confirm_password).clone().is_empty() && !(*admin_info).password.clone().is_empty() {
                                if *admin_info.password.clone() != *confirm_password.clone() {
                                    "bg-red-50 border border-red-500 outline-none text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 dark:bg-gray-700 focus:border-red-500 block w-full p-2.5 dark:text-red-500 dark:placeholder-red-500 dark:border-red-500"
                                } else {
                                    "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                }
                            } else {
                                "bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            }
                        } placeholder="•••••••••" required=true value={(*confirm_password).clone()} oninput={oninput_confirm_password}/>
                    {
                        if !(*confirm_password).clone().is_empty() && !(*admin_info).password.clone().is_empty() {
                            if *admin_info.password.clone() != *confirm_password.clone() {
                                html! { <p class="mt-2 text-sm text-red-600 dark:text-red-500">{ "Password does not match" }</p> }
                            } else {
                                html! {}
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <button type="submit" disabled={
                    if !(*confirm_password).clone().is_empty() && !(*admin_info).password.clone().is_empty() {
                        if *admin_info.password.clone() == *confirm_password.clone() {
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                } class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{ "Submit" }</button>
            </form>
        </div>
    }
}
