use yew::prelude::*;

use crate::types::MonitorForSelectWrapper;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data: MonitorForSelectWrapper,
}

#[function_component(MonitoringTable)]
pub fn monitoring_table(props: &Props) -> Html {
    let data = &props.data;

    html! {
        <table class="w-full text-sm text-left rtl:text-center text-gray-900 dark:text-gray-400">
            <thead class="border-b dark:border-0 text-xs text-gray-700 uppercase bg-neutral-400 dark:bg-gray-700 dark:text-gray-400">
                <tr>
                    <th scope="col" class="px-6 py-3">{ "CID#" }</th>
                    <th scope="col" class="px-6 py-3">{ "Fullname" }</th>
                    <th scope="col" class="px-6 py-3">{ "Building Tower" }</th>
                    <th scope="col" class="px-6 py-3">{ "Building Room" }</th>
                    <th scope="col" class="px-6 py-3">{ "Status" }</th>
                    <th scope="col" class="px-6 py-3">{ "Date Begin" }</th>
                    <th scope="col" class="px-6 py-3">{ "Date Modified" }</th>
                    <th scope="col" class="px-6 py-3">
                        <span class="sr-only">{ "Details" }</span>
                    </th>
                </tr>
            </thead>
            <tbody>
                {
                    for data.monitors.iter().map(|monitor| {
                        html! {
                            <tr class="bg-white border-b bg-gray-400 dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600">
                                <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                    { &monitor.id }
                                </th>
                                <td class="px-6 py-4">
                                    { &monitor.client_name }
                                </td>
                                <td class="px-6 py-4">
                                    { &monitor.building_tower }
                                </td>
                                <td class="px-6 py-4">
                                    { &monitor.building_room }
                                </td>
                                <td class="px-6 py-4">
                                    { &monitor.device_state }
                                </td>
                                <td class="px-6 py-4">
                                    { &monitor.date_begin }
                                </td>
                                <td class="px-6 py-4">
                                    { &monitor.date_modified }
                                </td>
                                <td class="px-6 py-4 text-center">
                                    <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{ "Details" }</a>
                                </td>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}
