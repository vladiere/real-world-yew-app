use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::{
    app::UserRoutes, components::status_component::StatusComponent, types::AccountsInfoWrapper,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data: AccountsInfoWrapper,
}

#[function_component(UsersTable)]
pub fn users_table(props: &Props) -> Html {
    let all_users = &props.data;

    html! {
        <table class="w-full text-sm text-left rtl:text-right text-gray-900 dark:text-gray-400">
            <thead class="border-b dark:border-0 text-xs text-gray-700 uppercase bg-neutral-400 dark:bg-gray-700 dark:text-gray-400">
                <tr>
                    <th scope="col" class="px-6 py-3">{ "CID#" }</th>
                    <th scope="col" class="px-6 py-3">{ "Fullname" }</th>
                    <th scope="col" class="px-6 py-3">{ "Email Address" }</th>
                    <th scope="col" class="px-6 py-3">{ "Gender" }</th>
                    <th scope="col" class="px-6 py-3">{ "Building" }</th>
                    <th scope="col" class="px-6 py-3">{ "Unit" }</th>
                    // <th scope="col" class="px-6 py-3">{ "Package" }</th>
                    <th scope="col" class="px-6 py-3">{ "Date Enrolled" }</th>
                    <th scope="col" class="px-6 py-3">{ "Status" }</th>
                    <th scope="col" class="px-3 py-3">
                        <span class="sr-only">{ "View" }</span>
                        <span class="sr-only">{ "Remove" }</span>
                    </th>
                </tr>
            </thead>
            <tbody>
                {
                    for all_users.accounts.iter().map(|user| {
                        html! {
                            <tr class="bg-white border-b bg-gray-400 dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600">
                                <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                    { &user.id }
                                </th>
                                <td class="px-6 py-4 capitalize">
                                    { format!("{}, {} {}.", &user.lastname, &user.firstname, &user.middlename.chars().next().unwrap_or_default()) }
                                </td>
                                <td class="px-6 py-4">
                                    { &user.email_address }
                                </td>
                                <td class="px-6 py-4">
                                    { &user.gender }
                                </td>
                                <td class="px-6 py-4">
                                    { &user.tower }
                                </td>
                                <td class="px-6 py-4">
                                    { &user.room }
                                </td>
                                // <td class="px-6 py-4">
                                //     { &user.package }
                                // </td>
                                <td class="px-6 py-4">
                                    { &user.date_enrolled }
                                </td>
                                <td class="px-6 py-4">
                                    <StatusComponent stats={(*user.status).to_string()} />
                                </td>
                                <td class="px-6 py-4 text-right flex gap-5">
                                    <Link<UserRoutes> to={UserRoutes::UserView { id: user.id.clone() }} classes="font-medium text-green-500 dark:text-green-500">
                                        <Icon icon_id={IconId::LucideView} width={"20px".to_owned()} height={"20px".to_owned()}/>
                                    </Link<UserRoutes>>
                                </td>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}
