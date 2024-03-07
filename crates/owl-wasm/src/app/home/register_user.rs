use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    components::alerts::{DangerAlert, SuccessAlert},
    services::accounts::add_user,
    types::{AccountRegisterInfo, AccountRegisterInfoWrapper},
};

#[function_component(RegisterUser)]
pub fn register_user() -> Html {
    let user_info = use_state(AccountRegisterInfo::default);
    let message = use_state(|| String::default());
    let is_loading = use_state(|| false);

    let request_register = {
        let user_info = user_info.clone();
        use_async(async move {
            let req = AccountRegisterInfoWrapper {
                account: (*user_info).clone(),
            };
            add_user(req).await
        })
    };

    let msg = message.clone();
    let is_load = is_loading.clone();
    use_effect_with(request_register.clone(), move |req_register| {
        if let Some(reg) = &req_register.data {
            is_load.set(true);
            msg.set(reg.message.clone());
        }
    });

    let onsubmit = {
        let request_register = request_register.clone();
        let is_loading = is_loading.clone();
        let user_info = user_info.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            request_register.run();

            // Create inputs except for the select.
            is_loading.set(true);
            user_info.set(Default::default());
        })
    };

    let oninput_firstname = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.firstname = input.value();
            user_info.set(info);
        })
    };

    let oninput_middlename = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.middlename = input.value();
            user_info.set(info);
        })
    };

    let oninput_lastname = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.lastname = input.value();
            user_info.set(info);
        })
    };

    let oninput_occupation = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.occupation = input.value();
            user_info.set(info);
        })
    };

    let oninput_recent_address = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.recent_address = input.value();
            user_info.set(info);
        })
    };

    let oninput_tower = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.tower = input.value();
            user_info.set(info);
        })
    };

    let oninput_room = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.room = input.value();
            user_info.set(info);
        })
    };

    let oninput_email = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.email_address = input.value();
            user_info.set(info);
        })
    };

    let onselect_civil_status = {
        let user_info = user_info.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.civil_status = select.value();
            user_info.set(info);
        })
    };

    let onselect_gender = {
        let user_info = user_info.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.gender = select.value();
            user_info.set(info);
        })
    };

    let onselect_package = {
        let user_info = user_info.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.package = select.value();
            user_info.set(info);
        })
    };

    html! {
        <>
            <SuccessAlert alert_msg={(*message).clone()} />
            <DangerAlert error={request_register.error.clone()} />
            <div class="flex flex-col gap-5">
                <h1 class="text-2xl font-bold">{ "User Registration" }</h1>
                <form {onsubmit} >
                    <div class="grid gap-6 mb-6 md:grid-cols-2">
                        <div>
                            <label for="first_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "First name" }</label>
                            <input type="text" id="first_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user first name" required=true value={user_info.firstname.clone()} oninput={oninput_firstname} />
                        </div>
                        <div>
                            <label for="middle_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Middle name" }</label>
                            <input type="text" id="middle_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user middle name" required=true value={user_info.middlename.clone()} oninput={oninput_middlename} />
                        </div>
                        <div>
                            <label for="last_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Last name" }</label>
                            <input type="text" id="last_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user last name" required=true value={user_info.lastname.clone()} oninput={oninput_lastname} />
                        </div>
                        <div>
                            <label for="gender" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an gender option" }</label>
                            <select id="gender" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={onselect_gender} value={user_info.gender.clone()} >
                                <option selected=true>{ "Gender" }</option>
                                <option value="male">{ "Male" }</option>
                                <option value="female">{ "Female" }</option>
                                <option value="other">{ "Others" }</option>
                            </select>
                        </div>
                        <div>
                            <label for="civil_status" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an option" }</label>
                            <select id="civil_status" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={onselect_civil_status} value={user_info.civil_status.clone()} >
                                <option selected=true>{ "Civil Status" }</option>
                                <option value="single">{ "Single" }</option>
                                <option value="married">{ "Married" }</option>
                                <option value="divorced">{ "Divorced" }</option>
                                <option value="widowed">{ "Widowed" }</option>
                                <option value="separate">{ "Separate" }</option>
                            </select>
                        </div>
                        <div>
                            <label for="occupation" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Occupation" }</label>
                            <input type="text" id="occupation" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user occupation" required=true oninput={oninput_occupation} value={user_info.occupation.clone()} />
                        </div>
                        <div>
                            <label for="recent_address" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Recent Address" }</label>
                            <input type="text" id="recent_address" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user recent address" required=true oninput={oninput_recent_address} value={user_info.recent_address.clone()} />
                        </div>
                        <div>
                            <label for="tower" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Tower" }</label>
                            <input type="text" id="tower" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter tower for user" required=true oninput={oninput_tower} value={user_info.tower.clone()} />
                        </div>
                        <div>
                            <label for="room" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Room" }</label>
                            <input type="text" id="room" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter room for user" required=true oninput={oninput_room} value={user_info.room.clone()} />
                        </div>
                        <div>
                            <label for="package" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an package option" }</label>
                            <select id="package" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={onselect_package} value={user_info.package.clone()} >
                                <option selected=true>{ "Choose a package" }</option>
                                <option value="Standard">{ "Standard" }</option>
                                <option value="Vip">{ "VIP" }</option>
                                <option value="Vvip">{ "VVIP" }</option>
                                <option value="Business">{ "Business" }</option>
                                <option value="Family">{ "Family" }</option>
                                <option value="Romance">{ "Romance" }</option>
                                <option value="Extended">{ "Extended Stay" }</option>
                                <option value="Inclusive">{ "All Inclusive" }</option>
                            </select>
                        </div>
                    </div>
                    <div class="mb-6">
                        <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Email address" }</label>
                        <input type="email" id="email" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user email address" required=true oninput={oninput_email} value={user_info.email_address.clone()} />
                    </div>
                    <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">if !(*is_loading).clone() {
                        { "Submit" }
                    } else {
                        <svg aria-hidden="true" class="w-6 h-6 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                            <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                        </svg>
                    }</button>
                </form>
            </div>
        </>
    }
}
