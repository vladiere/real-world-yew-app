use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::{AdminRoutes, AppRoutes},
    hooks::use_user_context,
};

#[function_component(AdminsPage)]
pub fn admins_page() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    if &user_ctx.role_user != "Super" {
        navigator.push(&AppRoutes::Home);
    }

    html! {
        <div class="flex flex-col gap-5">
            <h1 class="text-2xl font-bold">{ "Admins List" }</h1>
            <div class="flex flex-row gap-10 justify-between">
                <Link<AdminRoutes> to={AdminRoutes::AdminRegister} classes="flex items-center gap-2 middle none center mr-4 rounded-lg bg-green-500 py-3 px-5 font-sans text-xs font-bold uppercase text-white shadow-md shadow-green-500/20 transition-all hover:shadow-lg hover:shadow-green-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none">
                    <svg fill="currentColor" width="24px" height="24px" viewBox="0 0 256 256" id="Flat" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M256,136a7.99977,7.99977,0,0,1-8,8H232v16a8,8,0,0,1-16,0V144H200a8,8,0,0,1,0-16h16V112a8,8,0,0,1,16,0v16h16A7.99977,7.99977,0,0,1,256,136ZM144.1427,157.55811a68,68,0,1,0-72.2854,0,119.88787,119.88787,0,0,0-55.77478,37.29394,8.00012,8.00012,0,0,0,6.12549,13.146l171.58447.00049a8.00012,8.00012,0,0,0,6.12549-13.146A119.88993,119.88993,0,0,0,144.1427,157.55811Z"></path></g></svg>
                    { "Add new Admin" }
                </Link<AdminRoutes>>
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
                            <th scope="col" class="px-6 py-3">{ "Fullname" }</th>
                            <th scope="col" class="px-6 py-3">{ "Username" }</th>
                            <th scope="col" class="px-6 py-3">{ "Email address" }</th>
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
                                { "kulas.admin" }
                            </td>
                            <td class="px-6 py-4">
                                { "kulas@email.com" }
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
                                { "admin.admin" }
                            </td>
                            <td class="px-6 py-4">
                                { "admin@email.com" }
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
                                { "nikulas.admin" }
                            </td>
                            <td class="px-6 py-4">
                                { "nikulas@email.com" }
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
