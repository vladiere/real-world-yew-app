use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::{
    app::{AdminRoutes, AppRoutes},
    components::{
        home::admins::{admins_table::AdminsTable, search_admin::SearchComponent},
        loading::LoadingComponent,
    },
    hooks::use_user_context,
    services::accounts::get_all_admins,
    types::AllAdminInfoWrapper,
};

#[function_component(AdminsPage)]
pub fn admins_page() -> Html {
    let user_ctx = use_user_context();
    let is_not_empty = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let res_admins = use_state(|| AllAdminInfoWrapper { admins: vec![] });

    if &user_ctx.role_user != "Super" {
        navigator.push(&AppRoutes::Home);
    }

    let all_admin = use_async_with_options(
        async move { get_all_admins().await },
        UseAsyncOptions::enable_auto(),
    );

    let resadmins = res_admins.clone();
    let isnot_empty = is_not_empty.clone();
    let search_callback = Callback::from(move |filtered_data: AllAdminInfoWrapper| {
        isnot_empty.set(true);
        resadmins.set(filtered_data);
    });

    html! {
        <div class="flex flex-col gap-5 p-8">
            <h1 class="text-2xl font-bold">{ "Admins List" }</h1>
            <div class="flex flex-row gap-10 justify-between">
                <Link<AdminRoutes> to={AdminRoutes::AdminRegister} classes="flex items-center gap-2 middle none center mr-4 rounded-lg bg-green-500 py-3 px-5 font-sans text-xs font-bold uppercase text-white shadow-md shadow-green-500/20 transition-all hover:shadow-lg hover:shadow-green-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none">
                    <svg fill="currentColor" width="24px" height="24px" viewBox="0 0 256 256" id="Flat" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M256,136a7.99977,7.99977,0,0,1-8,8H232v16a8,8,0,0,1-16,0V144H200a8,8,0,0,1,0-16h16V112a8,8,0,0,1,16,0v16h16A7.99977,7.99977,0,0,1,256,136ZM144.1427,157.55811a68,68,0,1,0-72.2854,0,119.88787,119.88787,0,0,0-55.77478,37.29394,8.00012,8.00012,0,0,0,6.12549,13.146l171.58447.00049a8.00012,8.00012,0,0,0,6.12549-13.146A119.88993,119.88993,0,0,0,144.1427,157.55811Z"></path></g></svg>
                    { "Add new Admin" }
                </Link<AdminRoutes>>
                {
                    if let Some(admins) = &all_admin.data {

                        html! {
                            <SearchComponent search_admin={(*admins).clone()} callback={search_callback.clone()} />
                        }
                    } else {
                        html! {
                            <div class="rounded-lg text-center">
                                <span class="text-2xl font-medium">{ "No data available to search" }</span>
                            </div>
                        }
                    }
                }
            </div>
            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                {
                    if all_admin.loading {
                        html! {
                            <LoadingComponent />
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if !(*is_not_empty).clone() {
                        if let Some(res_data) = &all_admin.data {
                            html! {
                                <AdminsTable admin={(*res_data).clone()} />
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {
                            <AdminsTable admin={(*res_admins).clone()} />
                        }
                    }
                }
            </div>
        </div>
    }
}
