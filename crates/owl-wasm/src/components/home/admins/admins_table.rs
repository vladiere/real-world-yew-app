use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::Link;

use crate::{
    app::AdminRoutes, components::status_component::StatusComponent, types::AllAdminInfoWrapper,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub admin: AllAdminInfoWrapper,
}

#[function_component(AdminsTable)]
pub fn admins_table(props: &Props) -> Html {
    let all_admin = &props.admin;

    html! {
        <table class="w-full text-sm text-left rtl:text-right text-gray-900 dark:text-gray-400">
            <thead class="border-b dark:border-0 text-xs text-gray-700 uppercase bg-neutral-400 dark:bg-gray-700 dark:text-gray-400">
                <tr>
                    <th scope="col" class="px-6 py-3">{ "ID" }</th>
                    <th scope="col" class="px-6 py-3">{ "Fullname" }</th>
                    <th scope="col" class="px-6 py-3">{ "Username" }</th>
                    <th scope="col" class="px-6 py-3">{ "Email address" }</th>
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
                    for all_admin.admins.iter().map(|admin| {
                        html! {
                            <tr class="border-b bg-gray-400 dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600">
                                <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                                    { &admin.user_id }
                                </th>
                                <td class="px-6 py-4 capitalize">
                                    { format!("{}, {} {}.", &admin.lastname, &admin.firstname, &admin.middlename.chars().next().unwrap_or_default()) }
                                </td>
                                <td class="px-6 py-4">
                                    { &admin.username }
                                </td>
                                <td class="px-6 py-4">
                                    { &admin.email_address }
                                </td>
                                <td class="px-6 py-4">
                                    { &admin.date_enrolled }
                                </td>
                                <td class="px-6 py-4 capitalize">
                                    <StatusComponent stats={(*admin.status).to_string()} />
                                </td>
                                <td class="px-6 py-4 text-right flex gap-5">
                                    <Link<AdminRoutes> to={AdminRoutes::AdminView { id: admin.id.clone() }} classes="font-medium text-green-500 dark:text-green-500">
                                        <Icon icon_id={IconId::LucideView} width={"20px".to_owned()} height={"20px".to_owned()}/>
                                    </Link<AdminRoutes>>
                                </td>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}
