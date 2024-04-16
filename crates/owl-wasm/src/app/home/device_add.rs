use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    components::alerts::{DangerAlert, SuccessAlert},
    services::devices::create_device,
    types::{DeviceForCreate, DeviceForCreateWrapper},
};

#[function_component(DeviceAdd)]
pub fn device_add() -> Html {
    let device_info = use_state(DeviceForCreate::default);
    let message = use_state(|| String::default());
    let is_loading = use_state(|| false);

    let add_device = {
        let device_info = device_info.clone();
        use_async(async move {
            let req = DeviceForCreateWrapper {
                device: (*device_info).clone(),
            };
            create_device(req).await
        })
    };

    let msg = message.clone();
    let is_load = is_loading.clone();
    use_effect_with(add_device.clone(), move |req_create| {
        if let Some(reg) = &req_create.data {
            is_load.set(false);
            msg.set(reg.message.clone());
        }
    });

    let onsubmit = {
        let req_create = add_device.clone();
        let is_loading = is_loading.clone();
        let device_info = device_info.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            req_create.run();

            // Create inputs except for the select.
            is_loading.set(true);
            device_info.set(Default::default());
        })
    };

    let oninput_name = {
        let device_info = device_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*device_info).clone();
            info.device_name = input.value();
            device_info.set(info);
        })
    };

    let oninput_tower_id = {
        let device_info = device_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*device_info).clone();
            info.device_tower = input.value();
            device_info.set(info);
        })
    };

    let oninput_room_num = {
        let device_info = device_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*device_info).clone();
            info.device_room = input.value();
            device_info.set(info);
        })
    };

    let onselect_device_state = {
        let device_info = device_info.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*device_info).clone();
            info.device_state = select.value();
            device_info.set(info);
        })
    };

    html! {
        <>
            <SuccessAlert alert_msg={(*message).clone()} />
            <DangerAlert error={add_device.error.clone()} />
            <div class="flex flex-col gap-5">
                <h1 class="text-2xl font-bold">{ "Add new device" }</h1>
                <form {onsubmit}>
                    <div class="grid gap-6 mb-6 md:grid-cols-2">
                        <div>
                            <label for="ip_address" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Device name" }</label>
                            <input type="text" id="ip_address" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter name of the device" required=true oninput={oninput_name} value={device_info.device_name.clone()} />
                        </div>
                        <div>
                            <label for="tower" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Tower Building ID" }</label>
                            <input type="text" id="tower" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter tower building id for the device" required=true oninput={oninput_tower_id} value={device_info.device_tower.clone()} />
                        </div>
                        <div>
                            <label for="room" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Room Number" }</label>
                            <input type="text" id="room" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter tower room number for the device" required=true oninput={oninput_room_num} value={device_info.device_room.clone()} />
                        </div>
                        <div>
                            <label for="civil_status" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an option" }</label>
                            <select id="civil_status" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={onselect_device_state} value={device_info.device_state.clone()} >
                                <option selected=true>{ "Device state" }</option>
                                <option value="standby">{ "Standby" }</option>
                                <option value="working">{ "Working" }</option>
                                <option value="repair">{ "Repair" }</option>
                                <option value="unavailable">{ "Unavailable" }</option>
                                <option value="available">{ "Available" }</option>
                                <option value="damaged">{ "Damaged" }</option>
                            </select>
                        </div>
                    </div>
                    <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" disabled={(*is_loading).clone()}>if !(*is_loading).clone() {
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
