use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{
    app::UserRoutes,
    components::{
        home::users::{search_user::SearchUserComponent, users_table::UsersTable},
        loading::LoadingComponent,
    },
    services::accounts::get_all_users,
    types::AccountsInfoWrapper,
};

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    let res_accounts = use_state(|| AccountsInfoWrapper { accounts: vec![] });
    let is_not_empty = use_state(|| false);

    let all_users = use_async_with_options(
        async move { get_all_users().await },
        UseAsyncOptions::enable_auto(),
    );

    let resaccount = res_accounts.clone();
    let isnot_empty = is_not_empty.clone();
    let search_callback = Callback::from(move |filtered_data: AccountsInfoWrapper| {
        isnot_empty.set(true);
        resaccount.set(filtered_data);
    });

    html! {
        <div class="flex flex-col gap-5 p-8">
            <h1 class="text-2xl font-bold">{ "Users List" }</h1>
            <div class="flex flex-row gap-10 justify-between">
                <Link<UserRoutes> to={UserRoutes::UserRegister} classes="flex items-center gap-2 middle none center mr-4 rounded-lg bg-green-500 py-3 px-5 font-sans text-xs font-bold uppercase text-white shadow-md shadow-green-500/20 transition-all hover:shadow-lg hover:shadow-green-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none">
                        <Icon icon_id={IconId::FontAwesomeSolidUserPlus} width={"20px".to_owned()} height={"20px".to_owned()}/>
                    { "Add new User" }
                </Link<UserRoutes>>
                {
                    if let Some(users) = &all_users.data {

                        html! {
                            <SearchUserComponent search_user={(*users).clone()} callback={search_callback.clone()} />
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
                    if all_users.loading {
                        html! {
                            <LoadingComponent />
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if !(*is_not_empty).clone() {
                        if let Some(res_data) = &all_users.data {
                            html! {
                                <UsersTable data={(*res_data).clone()} />
                            }
                        } else {
                            // if !all_users.loading {
                            //     html! {
                            //         <div class="w-full h-1/4 flex flex-col items-center justify-center">
                            //             <span class="text-4xl font-bold">{ "Empty" }</span>
                            //             <span class="text-2xl font-medium">{ "No data available" }</span>
                            //         </div>
                            //     }
                            // } else {
                            //     html! {}
                            // }
                            html! {}
                        }
                    } else {
                        html! {
                            <UsersTable data={(*res_accounts).clone()} />
                        }
                    }
                }
            </div>
        </div>
    }
}
