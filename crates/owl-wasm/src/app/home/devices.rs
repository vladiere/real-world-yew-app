use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::DeviceRoutes;

#[function_component(DevicesPage)]
pub fn devices_page() -> Html {
    html! {
        <div class="flex flex-col gap-5">
            <h1 class="text-2xl font-bold">{ "Devices List" }</h1>
            <div class="flex flex-row gap-10 justify-between">
                <Link<DeviceRoutes> to={DeviceRoutes::DeviceRegister} classes="flex items-center gap-2 middle none center mr-4 rounded-lg bg-green-500 py-3 px-5 font-sans text-xs font-bold uppercase text-white shadow-md shadow-green-500/20 transition-all hover:shadow-lg hover:shadow-green-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none">
                    <svg width="24px" height="24px" viewBox="0 0 48 48" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M16.8 9.18602C17.5496 7.83679 18.9717 7 20.5152 7H27.4848C29.0283 7 30.4504 7.83679 31.2 9.18602L32.4855 11.5H36.25C39.4256 11.5 42 14.0744 42 17.25V24.0436C39.9794 22.75 37.5773 22 35 22C33.8186 22 32.6739 22.1576 31.586 22.4529C30.5222 19.2834 27.5278 17 24 17C19.5817 17 16 20.5817 16 25C16 28.7945 18.6418 31.972 22.1865 32.7936C22.0639 33.5107 22 34.2479 22 35C22 36.7718 22.3545 38.4608 22.9963 40H11.75C8.57436 40 6 37.4256 6 34.25V17.25C6 14.0744 8.57436 11.5 11.75 11.5H15.5145L16.8 9.18602Z" fill="currentColor"></path> <path d="M24 19.5C20.9624 19.5 18.5 21.9624 18.5 25C18.5 27.6415 20.3622 29.8481 22.8454 30.3786C24.0153 27.3035 26.3187 24.7871 29.2451 23.34C28.5411 21.1138 26.459 19.5 24 19.5Z" fill="currentColor"></path> <path d="M35 46C41.0751 46 46 41.0751 46 35C46 28.9249 41.0751 24 35 24C28.9249 24 24 28.9249 24 35C24 41.0751 28.9249 46 35 46ZM35 28C35.5523 28 36 28.4477 36 29V34H41C41.5523 34 42 34.4477 42 35C42 35.5523 41.5523 36 41 36H36V41C36 41.5523 35.5523 42 35 42C34.4477 42 34 41.5523 34 41V36H29C28.4477 36 28 35.5523 28 35C28 34.4477 28.4477 34 29 34H34V29C34 28.4477 34.4477 28 35 28Z" fill="currentColor"></path> </g></svg>
                    { "Add new Device" }
                </Link<DeviceRoutes>>
                <div class="relative">
                    <input class="w-full h-10 pl-10 pr-5 text-sm rounded-full appearance-none focus:outline-none bg-gray-100 dark:bg-gray-800 border border-gray-100 dark:border-gray-700" placeholder="Search..."/>
                    <button class="absolute top-0 left-0 mt-3 ml-4">
                        <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 stroke-current" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                    </button>
                </div>
            </div>
            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                <table class="w-full text-sm text-left rtl:text-right text-gray-900 dark:text-gray-400">
                    <thead class="border-b dark:border-0 text-xs text-gray-700 uppercase bg-neutral-400 dark:bg-gray-700 dark:text-gray-400">
                        <tr>
                            <th scope="col" class="px-6 py-3">{ "CID#" }</th>
                            <th scope="col" class="px-6 py-3">{ "Owner" }</th>
                            <th scope="col" class="px-6 py-3">{ "Tower" }</th>
                            <th scope="col" class="px-6 py-3">{ "Room" }</th>
                            <th scope="col" class="px-6 py-3">{ "Date Enrolled" }</th>
                            <th scope="col" class="px-6 py-3">{ "Status" }</th>
                            <th scope="col" class="px-3 py-3">
                                <span class="sr-only">{ "Edit" }</span>
                                <span class="sr-only">{ "Remove" }</span>
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="bg-white border-b bg-gray-400 dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600">
                            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                { "124143" }
                            </th>
                            <td class="px-6 py-4">
                                { "Christian L. Perez" }
                            </td>
                            <td class="px-6 py-4">
                                { "A234B" }
                            </td>
                            <td class="px-6 py-4">
                                { "B4BB" }
                            </td>
                            <td class="px-6 py-4">
                                { "2024-12-20" }
                            </td>
                            <td class="px-6 py-4">
                                { "Active" }
                            </td>
                            <td class="px-6 py-4 text-right flex gap-5">
                                <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{ "Edit" }</a>
                                <a href="#" class="font-medium text-red-500 dark:text-red-500 hover:underline">{ "Remove" }</a>
                            </td>
                        </tr>
                        <tr class="bg-white border-b bg-gray-400 dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600">
                            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                { "23412" }
                            </th>
                            <td class="px-6 py-4">
                                { "Kulas T. Great" }
                            </td>
                            <td class="px-6 py-4">
                                { "HH333" }
                            </td>
                            <td class="px-6 py-4">
                                { "B62" }
                            </td>
                            <td class="px-6 py-4">
                                { "2024-11-20" }
                            </td>
                            <td class="px-6 py-4">
                                { "Enactive" }
                            </td>
                            <td class="px-6 py-4 text-right flex gap-5">
                                <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{ "Edit" }</a>
                                <a href="#" class="font-medium text-red-500 dark:text-red-500 hover:underline">{ "Remove" }</a>
                            </td>
                        </tr>
                        <tr class="bg-white border-b bg-gray-400 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-600">
                            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                { "523423" }
                            </th>
                            <td class="px-6 py-4">
                                { "Nikulas G. Goat" }
                            </td>
                            <td class="px-6 py-4">
                                { "H69" }
                            </td>
                            <td class="px-6 py-4">
                                { "A55" }
                            </td>
                            <td class="px-6 py-4">
                                { "2024-11-02" }
                            </td>
                            <td class="px-6 py-4">
                                { "Active" }
                            </td>
                            <td class="px-6 py-4 text-right flex gap-5">
                                <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{ "Edit" }</a>
                                <a href="#" class="font-medium text-red-500 dark:text-red-500 hover:underline">{ "Remove" }</a>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
