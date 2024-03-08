use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    app::AdminRoutes,
    components::{
        alerts::{DangerAlert, SuccessAlert},
        home::admins::delete_admin_btn::DeleteButton,
        props_error::PropsError,
    },
    services::accounts::{get_one_admin, update_one_admin},
    types::{UpdateOneAdminInfo, UpdateOneAdminInfoWrapper},
};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub admin_id: i64,
}

#[function_component(ViewAdmin)]
pub fn view_admin(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let id = props.admin_id;
    let update_admin = use_state(UpdateOneAdminInfo::default);
    let is_loading = use_state(|| false);
    let message = use_state(|| String::default());

    let one_admin = use_async_with_options(
        async move { get_one_admin(id).await },
        UseAsyncOptions::enable_auto(),
    );

    let on_update = {
        let update_admin = update_admin.clone();
        use_async(async move {
            let request = UpdateOneAdminInfoWrapper {
                admin: (*update_admin).clone(),
            };
            update_one_admin(request).await
        })
    };

    let msg = message.clone();
    let is_load = is_loading.clone();

    use_effect_with(on_update.clone(), move |res_update| {
        if let Some(update_data) = &res_update.data {
            is_load.set(false);
            msg.set(update_data.message.clone());
        }

        if let Some(_) = &res_update.error {
            is_load.set(false);
        }
        || ()
    });

    let onsubmit = {
        let on_update = on_update.clone();
        let is_loading = is_loading.clone();
        let update_admin = update_admin.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            is_loading.set(true);
            let mut info = (*update_admin).clone();
            info.admin_id = id;
            update_admin.set(info);
            on_update.run();
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

    let oninput_email = {
        let update_admin = update_admin.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_admin).clone();
            info.email_address = input.value();
            update_admin.set(info);
        })
    };

    let delete_callback = Callback::from(move |_state| navigator.push(&AdminRoutes::AdminsList));

    if let Some(admin) = &one_admin.data {
        html! {
            <div class="flex flex-col gap-5">
                <SuccessAlert alert_msg={(*message).clone()} />
                <DangerAlert error={on_update.error.clone()} />
                <h1 class="text-2xl font-bold">{ "Admin information details" }</h1>
                <div class="grid items-center">
                    <img src="https://img.freepik.com/free-vector/illustration-businessman_53876-5856.jpg?t=st=1709475853~exp=1709479453~hmac=c828619b151f43e33085076888676a6c564fe5b88e4a20baba5b94f2be13766d&w=740" alt="admin" class="rounded-lg h-52" />
                </div>
                <div class="grid gap-5 md:grid-cols-2">
                    <div>
                        <label for="first_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "First name" }</label>
                        <input type="text" id="first_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.firstname.clone()} />
                    </div>
                    <div>
                        <label for="middle_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Middle name" }</label>
                        <input type="text" id="middle_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.middlename.clone()} />
                    </div>
                    <div>
                        <label for="last_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Last name" }</label>
                        <input type="text" id="last_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.lastname.clone()} />
                    </div>
                    <div>
                        <label for="recent_address" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Recent address" }</label>
                        <input type="text" id="recent_address" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.recent_address.clone()} />
                    </div>
                    <div>
                        <label for="gender" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Gender" }</label>
                        <input type="text" id="gender" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.gender.clone()} />
                    </div>
                    <div>
                        <label for="civil_status" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Civil status" }</label>
                        <input type="text" id="civil_status" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.civil_status.clone()} />
                    </div>
                    <div>
                        <label for="occupation" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Occupation" }</label>
                        <input type="text" id="occupation" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.occupation.clone()} />
                    </div>
                    <div>
                        <label for="date_enrolled" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Date enrolled" }</label>
                        <input type="text" id="date_enrolled" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" disabled=true value={admin.admin.date_enrolled.clone()} />
                    </div>
                </div>
                <h1 class="text-2xl font-bold">{ "Change username or email address" }</h1>
                <form {onsubmit}>
                    <div class="grid gap-5 md:grid-cols-2">
                        <div>
                            <label for="username" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Username: " }</label>
                            <input type="text" id="username" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" required=true value={update_admin.username.clone()} oninput={oninput_username} />
                        </div>
                        <div>
                            <label for="date_enrolled" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Email address: " }</label>
                            <input type="text" id="date_enrolled" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter admin first name" required=true value={update_admin.email_address.clone()} oninput={oninput_email}/>
                        </div>
                        <div class="flex items-center justify-between">
                            <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{ "Save changes" }</button>
                            <span class="text-2xl font-bold">{ "OR" }</span>
                            <DeleteButton admin_id={id.clone()} callback={delete_callback.clone()} />
                        </div>
                    </div>
                </form>
            </div>
        }
    } else {
        html! {
            <PropsError errors={one_admin.error.clone()} />
        }
    }
}
