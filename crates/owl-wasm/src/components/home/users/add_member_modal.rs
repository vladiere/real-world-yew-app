use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    components::alerts::DangerAlert,
    services::accounts::add_new_member,
    types::{MemberInfo, MemberInfoWrapper},
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub client_id: i64,
    pub callback: Callback<(bool, String)>,
}

#[function_component(AddMemberModal)]
pub fn add_member_modal(props: &Props) -> Html {
    let member = use_state(MemberInfo::default);
    let is_loading = use_state(|| false);
    let id = props.client_id;

    let add_member = {
        let member = member.clone();
        use_async(async move {
            let req = MemberInfoWrapper {
                member: (*member).clone(),
            };
            add_new_member(req).await
        })
    };

    {
        let is_load = is_loading.clone();
        let callback = props.callback.clone();
        let member = member.clone();
        use_effect_with(add_member.clone(), move |res_member| {
            if let Some(res_data) = &res_member.data {
                is_load.set(false);
                callback.emit((false, res_data.message.clone()));
                member.set(Default::default());
            }
        });
    }

    let onsubmit = {
        let add_member = add_member.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            add_member.run();

            //Set loading for button and close the modal.
            is_loading.set(true);
        })
    };

    let close_modal = {
        let cb = props.callback.clone();
        Callback::from(move |_| {
            cb.emit((false, String::default()));
        })
    };

    let oninput_firstname = {
        let member = member.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*member).clone();
            info.firstname = input.value();
            member.set(info);
        })
    };

    let oninput_middlename = {
        let member = member.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*member).clone();
            info.middlename = input.value();
            member.set(info);
        })
    };

    let oninput_lastname = {
        let member = member.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*member).clone();
            info.lastname = input.value();
            member.set(info);
        })
    };

    let oninput_age = {
        let member = member.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*member).clone();
            info.age = input.value().parse().unwrap_or_default();
            member.set(info);
        })
    };

    let onchange_gender = {
        let member = member.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*member).clone();
            info.gender = select.value();
            info.user_id = id;
            member.set(info);
        })
    };

    html! {
        <div class="overflow-y-auto overflow-x-hidden fixed z-50 flex justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full">
            <DangerAlert error={add_member.error.clone()} />
            <div class="h-screen w-screen absolute bg-black opacity-60">
            </div>
            <div class="relative p-4 w-full max-w-md max-h-full absolute">
                <div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
                    <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
                            { "Add member" }
                        </h3>
                        <button type="button" disabled={(*is_loading).clone()} class="end-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white" data-modal-hide="authentication-modal" onclick={close_modal.clone()}>
                            if (*is_loading).clone() {
                                <svg aria-hidden="true" class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                                </svg>
                            } else {
                                <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                                </svg>
                                <span class="sr-only">{ "Close modal" }</span>
                            }
                        </button>
                    </div>
                    <div class="p-4 md:p-5">
                        <form class="space-y-4" {onsubmit} >
                            <div class="grid gap-5 grid-cols-2">
                                <div>
                                    <label for="first_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Firstname" }</label>
                                    <input type="text" name="first_name" id="first_name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white" placeholder="Enter member's firstname" required=true oninput={oninput_firstname} value={member.firstname.clone()} />
                                </div>
                                <div>
                                    <label for="middle_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Middlename" }</label>
                                    <input type="text" name="middle_name" id="middle_name" placeholder="Enter member's middlename" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white" required=true oninput={oninput_middlename} value={member.middlename.clone()} />
                                </div>
                                <div>
                                    <label for="last_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Lastname" }</label>
                                    <input type="text" name="last_name" id="last_name" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white" placeholder="Enter member's lastname" required=true oninput={oninput_lastname} value={member.lastname.clone()} />
                                </div>
                                <div>
                                    <label for="age" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Age" }</label>
                                    <input type="number" min="0" name="age" id="age" placeholder="Enter member's age" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white" required=true oninput={oninput_age} value={member.age.clone().to_string()} />
                                </div>
                                <div>
                                    <label for="gender" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an gender option" }</label>
                                    <select id="gender" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={onchange_gender} value={member.gender.clone()} >
                                        <option selected=true>{ "Gender" }</option>
                                        <option value="Male">{ "Male" }</option>
                                        <option value="Female">{ "Female" }</option>
                                        <option value="Other">{ "Others" }</option>
                                    </select>
                                </div>
                            </div>
                            {
                                if (*is_loading).clone() {
                                    html! {
                                        <div role="status" class="flex w-full justify-end mr-5">
                                            <svg aria-hidden="true" class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                                                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                                                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                                            </svg>
                                            <span class="sr-only">{ "Loading..." }</span>
                                        </div>
                                    }
                                } else {
                                    html! {
                                        <div class="flex gap-5 flex-shrink-0 flex-wrap items-center justify-end rounded-b-md border-t-2 border-neutral-100 p-4 dark:border-white/10">
                                            <button type="button" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700" onclick={close_modal.clone()}>{ "Close" }</button>
                                            <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" >{ "Save changes" } </button>
                                        </div>
                                    }
                                }
                            }
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
