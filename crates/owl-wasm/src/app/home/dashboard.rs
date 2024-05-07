use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{app::AppRoutes, hooks::use_user_context, services::reports::get_dashboard_data};

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();

    if !user_ctx.is_authenticated() {
        navigator.push(&AppRoutes::Login);
    }

    let dashboard_counts = use_async_with_options(
        async move { get_dashboard_data().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(counts) = &dashboard_counts.data {
        html! {
            <div class="flex flex-col w-full h-full dark:bg-gray-800 dark:text-gray-200">
                <div class="flex flex-col mb-5">
                    <h1 class="text-2xl font-bold">{ "Dashboard" }</h1>
                    <div class="flex font-medium text-6md gap-2">
                        { "Welcome" }
                        <p class="text-blue-700 capitalize">{ &*user_ctx.firstname.clone() }</p>
                        { "everything looks great." }
                    </div>
                </div>
                <div class="w-full flex flex-col mb-20 gap-8">
                    <div class="flex flex-row gap-5">
                        <div class="w-2/4 px-3">
                            <div class="w-full bg-gray-400 dark:bg-slate-900 flex-col justify-between text-gray-700 dark:text-blue-400 rounded-lg flex items-center mb-6 xl:mb-0 border dark:border-slate-700">
                                <div class="w-full flex flex-row justify-between p-6">
                                    <div class="dark:text-gray-400">
                                        <p class="font-semibold text-3xl">{ &counts.data.active_users }</p>
                                        <p>{ "Registered Users" }</p>
                                    </div>
                                    <div class="p-3 bg-slate-500 dark:bg-slate-800 rounded-xl border border-slate-700 flex items-center">
                                        <Icon icon_id={IconId::FontAwesomeSolidUsers} width={"28px".to_owned()} height={"28px".to_owned()} />
                                    </div>
                                </div>
                                <Link<AppRoutes> to={AppRoutes::UsersRoot} classes="w-full flex flex-row justify-between items-center bg-slate-500 dark:bg-slate-800 px-6 rounded-b-lg py-[3px] dark:hover:text-blue-600 hover:text-blue-500 cursor-pointer">
                                    <p class="text-4md">
                                        { "View all users" }
                                    </p>
                                    <Icon icon_id={IconId::BootstrapArrowRightCircle} width={"18px".to_owned()} height={"18px".to_owned()}/>
                                </Link<AppRoutes>>
                            </div>
                        </div>
                        <div class="w-2/4 px-3">
                            <div class="w-full bg-gray-400 dark:bg-slate-900 flex-col justify-between text-gray-700 dark:text-blue-400 rounded-lg flex items-center mb-6 xl:mb-0 border dark:border-slate-700">
                                <div class="w-full flex flex-row justify-between p-6">
                                    <div class="dark:text-gray-400">
                                        <p class="font-semibold text-3xl">{ &counts.data.admins }</p>
                                        <p>{ "Admins Employeed" }</p>
                                    </div>
                                    <div class="p-3 bg-slate-500 dark:bg-slate-800 rounded-xl border border-slate-700 flex items-center">
                                        <Icon icon_id={IconId::FontAwesomeSolidUsersGear} width={"28px".to_owned()} height={"28px".to_owned()} />
                                    </div>
                                </div>
                                <Link<AppRoutes> to={AppRoutes::AdminsRoot} classes="w-full flex flex-row justify-between items-center bg-slate-500 dark:bg-slate-800 px-6 rounded-b-lg py-[3px] dark:hover:text-blue-600 hover:text-blue-500 cursor-pointer">
                                    <p class="text-4md">
                                        { "View all admins" }
                                    </p>
                                    <Icon icon_id={IconId::BootstrapArrowRightCircle} width={"18px".to_owned()} height={"18px".to_owned()}/>
                                </Link<AppRoutes>>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-row gap-5">
                        <div class="w-2/4 px-3">
                            <div class="w-full bg-gray-400 dark:bg-slate-900 flex-col justify-between text-gray-700 dark:text-blue-400 rounded-lg flex items-center mb-6 xl:mb-0 border dark:border-slate-700">
                                <div class="w-full flex flex-row justify-between p-6">
                                    <div class="dark:text-gray-400">
                                        <p class="font-semibold text-3xl">{ &counts.data.total_devices }</p>
                                        <p>{ "Devices Installed" }</p>
                                    </div>
                                    <div class="p-3 bg-slate-500 dark:bg-slate-800 rounded-xl border border-slate-700 flex items-center">
                                        <Icon icon_id={IconId::LucideWebcam} width={"28px".to_owned()} height={"28px".to_owned()} />
                                    </div>
                                </div>
                                <Link<AppRoutes> to={AppRoutes::DevicesRoot} classes="w-full flex flex-row justify-between items-center bg-slate-500 dark:bg-slate-800 px-6 rounded-b-lg py-[3px] dark:hover:text-blue-600 hover:text-blue-500 cursor-pointer">
                                    <p class="text-4md">
                                        { "View all devices" }
                                    </p>
                                    <Icon icon_id={IconId::BootstrapArrowRightCircle} width={"18px".to_owned()} height={"18px".to_owned()}/>
                                </Link<AppRoutes>>
                            </div>
                        </div>
                        <div class="w-2/4 px-3">
                            <div class="w-full bg-gray-400 dark:bg-slate-900 flex-col justify-between text-gray-700 dark:text-blue-400 rounded-lg flex items-center mb-6 xl:mb-0 border dark:border-slate-700">
                                <div class="w-full flex flex-row justify-between p-6">
                                    <div class="dark:text-gray-400">
                                        <p class="font-semibold text-3xl">{ &counts.data.monitors }</p>
                                        <p>{ "Door Opened" }</p>
                                    </div>
                                    <div class="p-3 bg-slate-500 dark:bg-slate-800 rounded-xl border border-slate-700 flex items-center">
                                        <Icon icon_id={IconId::LucideMonitor} width={"28px".to_owned()} height={"28px".to_owned()} />
                                    </div>
                                </div>
                                <Link<AppRoutes> to={AppRoutes::Monitoring} classes="w-full flex flex-row justify-between items-center bg-slate-500 dark:bg-slate-800 px-6 rounded-b-lg py-[3px] dark:hover:text-blue-600 hover:text-blue-500 cursor-pointer">
                                    <p class="text-4md">
                                        { "View realtime monitoring" }
                                    </p>
                                    <Icon icon_id={IconId::BootstrapArrowRightCircle} width={"18px".to_owned()} height={"18px".to_owned()}/>
                                </Link<AppRoutes>>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
