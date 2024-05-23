use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{
    components::{
        alerts::{DangerAlert, SuccessAlert},
        home::admins::device::DeleteDeviceButton,
    },
    services::devices::{select_device, update_device},
    types::{DeviceForUpdate, DeviceForUpdateWrapper},
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub device_id: i64,
}

#[function_component(DeviceInfo)]
pub fn device_info(props: &Props) -> Html {
    let device_info = use_state(DeviceForUpdate::default);
    let id = props.device_id;
    let input_state = use_state(|| true);
    let message = use_state(|| String::default());

    let update = {
        let device_info = device_info.clone();
        use_async(async move {
            let req = DeviceForUpdateWrapper {
                device: (*device_info).clone(),
            };
            update_device(req).await
        })
    };

    let one_device = use_async_with_options(
        async move { select_device(id).await },
        UseAsyncOptions::enable_auto(),
    );

    let msg = message.clone();
    use_effect_with(update.clone(), move |updated| {
        if let Some(res) = &updated.data {
            msg.set(res.message.clone());
        }
    });

    let enable_input = {
        let input_state = input_state.clone();
        Callback::from(move |_| {
            input_state.set(!(*input_state));
        })
    };

    let onselect_device_state = {
        let device_info = device_info.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*device_info).clone();
            info.device_state = select.value();
            info.id = id;
            device_info.set(info);
        })
    };

    let on_update = {
        let update = update.clone();
        let input_state = input_state.clone();
        let one_device = one_device.clone();
        Callback::from(move |_| {
            update.run();
            one_device.run();
            input_state.set(!(*input_state));
        })
    };

    if let Some(data) = &one_device.data {
        html! {
            <>
                <SuccessAlert alert_msg={(*message).clone()} />
                <DangerAlert error={update.error.clone()} />
                <div class="overflow-y-auto overflow-x-hidden flex justify-center items-center w-full md:inset-0 h-screen max-h-full">
                    <div class="relative p-4 w-full max-w-2xl max-h-full absolute">
                        // <!-- Modal content -->
                        <div class="relative rounded-lg shadow bg-transparent">
                            // <!-- Modal header -->
                            <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-200">
                                <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
                                    { "Device Info Details" }
                                </h3>
                            </div>
                            // <!-- Modal body -->
                            <div class="p-4 md:p-5 space-y-4 relative">
                                <button class="bg-transparent outline-none absolute top-2 right-5" onclick={enable_input.clone()}>
                                    <Icon class="stroke-green-500" icon_id={IconId::FeatherEdit} width={"24px".to_owned()} height={"24px".to_owned()}/>
                                </button>
                                <div class="grid grid-cols-2 gap-5">
                                    <div class="flex flex-col">
                                        <p class="text-4md text-bold text-gray-400 capitalize">{ "Device Name" }</p>
                                        <input class="text-8md text-medium focus:outline-none bg-transparent uppercase border border-gray-200 dark:border-gray-600 p-2" value={data.device.device_name.clone()} readonly=true />
                                    </div>
                                    <div class="flex flex-col">
                                        <p class="text-4md text-bold text-gray-400 capitalize">{ "Device Building" }</p>
                                        <input class="text-8md text-medium focus:outline-none bg-transparent uppercase border border-gray-200 dark:border-gray-600 p-2" value={data.device.device_tower.clone()} readonly=true />
                                    </div>
                                    <div class="flex flex-col">
                                        <p class="text-4md text-bold text-gray-400 capitalize">{ "Device Unit" }</p>
                                        <input class="text-8md text-medium focus:outline-none bg-transparent uppercase border border-gray-200 dark:border-gray-600 p-2" value={data.device.device_room.clone()} readonly=true />
                                    </div>
                                    <div class="flex flex-col">
                                        <p class="text-4md text-bold text-gray-400 capitalize">{ "Device State" }</p>
                                        {
                                            if *input_state.clone() {
                                                html! {
                                                    <p class="text-sm text-medium text-gray-400 uppercase border border-gray-200 dark:border-gray-600 p-2">{ data.device.device_state.clone() }</p>
                                                }
                                            } else {
                                                html! {
                                                    <select id="civil_status" class="text-8md text-medium focus:outline-none bg-transparent border border-green-400 p-2" onchange={onselect_device_state.clone()} value={device_info.device_state.clone()} >
                                                        <option selected=true>{ "Device state" }</option>
                                                        <option value="standby">{ "Standby" }</option>
                                                        <option value="working">{ "Working" }</option>
                                                        <option value="repair">{ "Repair" }</option>
                                                        <option value="unavailable">{ "Unavailable" }</option>
                                                        <option value="available">{ "Available" }</option>
                                                        <option value="damaged">{ "Damaged" }</option>
                                                    </select>
                                                }
                                            }
                                        }

                                    </div>
                                    <div class="flex flex-col">
                                        <p class="text-md text-bold text-gray-400 capitalize">{ "Added At" }</p>
                                        <p class="text-sm text-medium text-gray-400 uppercase border border-gray-200 dark:border-gray-600 p-2">{ "January 11, 1200" }</p>
                                    </div>
                                    <div class="flex flex-col">
                                        <p class="text-md text-bold text-gray-400 capitalize">{ "Modified At" }</p>
                                        <p class="text-sm text-medium text-gray-400 uppercase border border-gray-200 dark:border-gray-600 p-2">{ "January 11, 1200" }</p>
                                    </div>
                                </div>
                            </div>
                            // <!-- Modal footer -->
                            <div class="flex justify-between items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600">
                                <button disabled={*input_state.clone()} data-modal-hide="default-modal" type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={on_update.clone()}>{ "Save changes" }</button>
                                <DeleteDeviceButton id={props.device_id.clone()} />
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    } else {
        html! {
            <div class="flex h-screen w-screen text-xl text-semibold">
                { "Something's wrong, no data available."}
            </div>
        }
    }
}
