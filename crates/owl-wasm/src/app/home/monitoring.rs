use yew::prelude::*;

#[function_component(MonitoringPage)]
pub fn monitoring_page() -> Html {
    html! {
        <div class="flex flex-col gap-5">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold">{ "Real time monitoring" }</h1>

                <div class="relative">
                    <input class="w-full h-10 pl-10 pr-5 text-sm rounded-full appearance-none focus:outline-none bg-gray-100 dark:bg-gray-800 border border-gray-100 dark:border-gray-700" placeholder="Search..."/>
                    <button class="absolute top-0 left-0 mt-3 ml-4">
                        <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 stroke-current" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                    </button>
                </div>
            </div>
            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                <table class="w-full text-sm text-left rtl:text-center text-gray-900 dark:text-gray-400">
                    <thead class="border-b dark:border-0 text-xs text-gray-700 uppercase bg-neutral-400 dark:bg-gray-700 dark:text-gray-400">
                        <tr>
                            <th scope="col" class="px-6 py-3">{ "CID#" }</th>
                            <th scope="col" class="px-6 py-3">{ "Fullname" }</th>
                            <th scope="col" class="px-6 py-3">{ "IP Address" }</th>
                            <th scope="col" class="px-6 py-3">{ "Device Package" }</th>
                            <th scope="col" class="px-6 py-3">{ "Date" }</th>
                            <th scope="col" class="px-6 py-3">
                                <span class="sr-only">{ "Details" }</span>
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
                                { "192.168.0.1" }
                            </td>
                            <td class="px-6 py-4">
                                { "Normal" }
                            </td>
                            <td class="px-6 py-4">
                                { "2024-12-20" }
                            </td>
                            <td class="px-6 py-4 text-center">
                                <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{ "Details" }</a>
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
                                { "192.168.0.2" }
                            </td>
                            <td class="px-6 py-4">
                                { "VIP" }
                            </td>
                            <td class="px-6 py-4">
                                { "2024-11-20" }
                            </td>
                            <td class="px-6 py-4 text-center">
                                <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{ "Details" }</a>
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
                                { "192.168.0.3" }
                            </td>
                            <td class="px-6 py-4">
                                { "VVIP" }
                            </td>
                            <td class="px-6 py-4">
                                { "2024-11-02" }
                            </td>
                            <td class="px-6 py-4 text-center">
                                <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{ "Details" }</a>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
