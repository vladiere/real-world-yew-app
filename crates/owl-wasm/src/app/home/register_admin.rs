use web_sys::{HtmlInputElement, HtmlSelectElement};

use yew::prelude::*;
use yew_hooks::use_async;
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

    let is_loading = use_state(|| false);

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
    let is_load = is_loading.clone();

    use_effect_with(admin_register.clone(), move |admin_reg| {
        if let Some(reg_data) = &admin_reg.data {
            is_load.set(false);
            msg.set(reg_data.message.clone());
        }
        if let Some(_) = &admin_reg.error {
            is_load.set(false);
        }
        || ()
    });

    let onsubmit = {
        let admin_register = admin_register.clone();
        let admin_info = admin_info.clone();
        let confirm_password = confirm_password.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            admin_register.run();

            // set loading button.
            is_loading.set(true);
            // Clear all input fields after submitting
            admin_info.set(Default::default());
            confirm_password.set(String::default());
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

    let oninput_recent_address = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.recent_address = input.value();
            admin_info.set(info);
        })
    };

    let oninput_occupation = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.occupation = input.value();
            admin_info.set(info);
        })
    };

    let onselect_gender = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.gender = select.value();
            admin_info.set(info);
        })
    };

    let onselect_civil_status = {
        let admin_info = admin_info.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let mut info = (*admin_info).clone();
            info.civil_status = select.value();
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
        <div class="flex flex-col gap-5 p-8">
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
                    <div>
                        <label for="recent_address" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Recent address" }</label>
                        <input type="text" id="recent_address" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin recent address" required=true  value={admin_info.recent_address.clone()} oninput={oninput_recent_address} />
                    </div>
                    <div>
                        <label for="occupation" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Occupation" }</label>
                        <input type="text" id="occupation" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin occupation" required=true value={admin_info.occupation.clone()} oninput={oninput_occupation}  />
                    </div>
                    <div>
                        <label for="gender" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an option" }</label>
                        <select id="gender" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={onselect_gender} value={admin_info.gender.clone()} >
                            <option selected=true>{ "Gender" }</option>
                            <option value="Male">{ "Male" }</option>
                            <option value="Female">{ "Female" }</option>
                            <option value="Other">{ "Others" }</option>
                        </select>
                    </div>
                    <div>
                        <label for="civil_status" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an option" }</label>
                        <select id="civil_status" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={onselect_civil_status} value={admin_info.civil_status.clone()} >
                            <option selected=true>{ "Civil Status" }</option>
                            <option value="Single">{ "Single" }</option>
                            <option value="Married">{ "Married" }</option>
                            <option value="Divorced">{ "Divorced" }</option>
                            <option value="Widowed">{ "Widowed" }</option>
                            <option value="Separate">{ "Separate" }</option>
                        </select>
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
                                (*is_loading).clone()
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    } class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">if !(*is_loading).clone() {
                    { "Submit" }
                } else {
                    <svg aria-hidden="true" class="w-6 h-6 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                        <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                    </svg>
                }</button>
            </form>
        </div>
    }
}
