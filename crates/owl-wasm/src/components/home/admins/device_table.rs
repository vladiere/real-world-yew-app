use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{app::DeviceRoutes, types::DeviceForSelectWrapper};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data: DeviceForSelectWrapper,
    pub callback: Callback<bool>,
}

#[function_component(DevicesTable)]
pub fn devices_table(props: &Props) -> Html {
    let devices = &props.data;

    html! {
        <table class="w-full text-sm text-left rtl:text-right text-gray-900 dark:text-gray-400">
            <thead class="border-b dark:border-0 text-xs text-gray-700 uppercase bg-neutral-400 dark:bg-gray-700 dark:text-gray-400">
                <tr>
                    <th scope="col" class="px-6 py-3">{ "DID#" }</th>
                    <th scope="col" class="px-6 py-3">{ "Device Name" }</th>
                    <th scope="col" class="px-6 py-3">{ "Tower" }</th>
                    <th scope="col" class="px-6 py-3">{ "Room" }</th>
                    <th scope="col" class="px-6 py-3">{ "State" }</th>
                    <th scope="col" class="px-6 py-3">{ "Added at" }</th>
                    <th scope="col" class="px-6 py-3">{ "Modified at" }</th>
                    <th scope="col" class="px-3 py-3">
                        <span class="sr-only">{ "Edit" }</span>
                        <span class="sr-only">{ "Remove" }</span>
                    </th>
                </tr>
            </thead>
            <tbody>
                {
                    for devices.devices.iter().map(|device| {
                        html! {
                            <tr class="bg-white border-b bg-gray-400 dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600">
                                <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                    { &device.id }
                                </th>
                                <td class="px-6 py-4 uppercase">
                                    { &device.device_name }
                                </td>
                                <td class="px-6 py-4 uppercase">
                                    { &device.device_tower }
                                </td>
                                <td class="px-6 py-4 uppercase">
                                    { &device.device_room }
                                </td>
                                <td class="px-6 py-4 capitalize">
                                    { &device.device_state }
                                </td>
                                <td class="px-6 py-4">
                                    { &device.created_at }
                                </td>
                                <td class="px-6 py-4">
                                    { &device.modified_at }
                                </td>
                                <td class="px-6 py-4 text-right flex gap-5">
                                    <Link<DeviceRoutes> to={DeviceRoutes::DevicesInfo { id: device.id.clone() }} classes="font-medium bg-transparent text-blue-600 dark:text-blue-500 hover:underline">
                                        { "Info" }
                                    </Link<DeviceRoutes>>
                                </td>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}
