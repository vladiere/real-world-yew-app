use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{
    components::{
        alerts::SuccessAlert,
        home::users::{add_member_modal::AddMemberModal, members_card::MembersCard},
        props_error::PropsError,
    },
    services::{accounts::get_one_user, members::user_members},
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: i64,
}

#[function_component(ViewUser)]
pub fn view_user(props: &Props) -> Html {
    let id = props.user_id;
    let show_modal = use_state(|| false);
    let message = use_state(|| String::default());

    let user_members = use_async_with_options(
        async move { user_members(id).await },
        UseAsyncOptions::enable_auto(),
    );

    let one_user = use_async_with_options(
        async move { get_one_user(id).await },
        UseAsyncOptions::enable_auto(),
    );

    let is_show_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            show_modal.set(true);
        })
    };

    let modal_callback = {
        let show_modal = show_modal.clone();
        let message = message.clone();
        Callback::from(move |(modal_state, msg)| {
            message.set(msg);
            show_modal.set(modal_state);
        })
    };

    if let Some(user) = &one_user.data {
        html! {
            <div class="h-full w-full flex flex-col gap-10">
                <SuccessAlert alert_msg={(*message).clone()} />
                <h1 class="text-2xl font-bold">{ "Client Information details" }</h1>
                <div class="grid gap-5 md:grid-cols-3">
                    <img src="https://img.freepik.com/free-vector/illustration-businessman_53876-5856.jpg?t=st=1709475853~exp=1709479453~hmac=c828619b151f43e33085076888676a6c564fe5b88e4a20baba5b94f2be13766d&w=740" alt="admin" class="rounded-lg h-52" />
                    <div class="col-span-2 grid gap-3 md:grid-cols-3 items-center">
                        <div class="flex flex-col gap-2">
                            <span class="font-bold text-8md">{ "Client ID Number: " }</span>
                            <span class="font-bold text-8md">{ "Fullname: " }</span>
                            <span class="font-bold text-8md">{ "Recent Address: " }</span>
                            <span class="font-bold text-8md">{ "Gender: " }</span>
                            <span class="font-bold text-8md">{ "Civil Status: " }</span>
                            <span class="font-bold text-8md">{ "Occupation: " }</span>
                            <span class="font-bold text-8md">{ "Email Address: " }</span>
                            <span class="font-bold text-8md">{ "Tower: " }</span>
                            <span class="font-bold text-8md">{ "Room: " }</span>
                            <span class="font-bold text-8md">{ "Date Enrolled: " }</span>
                        </div>
                        <div class="col-span-2 flex flex-col gap-2">
                            <span class="font-medium text-4md">{ id }</span>
                            <span class="font-medium text-4md capitalize">{ format!("{}, {} {}.", user.account.lastname.clone(), user.account.firstname.clone(), user.account.middlename.clone().chars().next().unwrap_or_default()) }</span>
                            <span class="font-medium text-4md capitalize">{ user.account.recent_address.clone() }</span>
                            <span class="font-medium text-4md">{ user.account.gender.clone() }</span>
                            <span class="font-medium text-4md">{ user.account.civil_status.clone() }</span>
                            <span class="font-medium text-4md capitalize">{ user.account.occupation.clone() }</span>
                            <span class="font-medium text-4md">{ user.account.email_address.clone() }</span>
                            <span class="font-medium text-4md capitalize">{ user.account.tower.clone() }</span>
                            <span class="font-medium text-4md capitalize">{ user.account.room.clone() }</span>
                            <span class="font-medium text-4md">{ user.account.date_enrolled.clone() }</span>
                        </div>
                    </div>
                </div>
                <div class="flex justify-between">
                    <button class="flex items-center gap-2 middle none center mr-4 rounded-lg bg-green-500 py-3 px-5 font-sans text-xs font-bold uppercase text-white shadow-md shadow-green-500/20 transition-all hover:shadow-lg hover:shadow-green-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none" onclick={is_show_modal} >
                        <Icon icon_id={IconId::FontAwesomeSolidUserPlus} width={"20px".to_owned()} height={"20px".to_owned()}/>
                        { "Add new member" }
                    </button>
                    <h1 class="text-2xl font-bold">{ "Members List" }</h1>
                    <div class="relative">
                        <input class="w-full h-10 pl-10 pr-5 text-sm rounded-full appearance-none focus:outline-none bg-gray-100 dark:bg-gray-800 border border-gray-100 dark:border-gray-700" placeholder="Search..." />
                        <button class="absolute top-0 left-0 mt-3 ml-4">
                            <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 stroke-current" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                        </button>
                    </div>
                </div>
                <div class="flex flex-wrap gap-5 items-center justify-center">
                    {
                        if let Some(members) = &user_members.data {
                            html! {
                                <MembersCard members={members.clone()} />
                            }
                        } else {
                            html! {
                                <PropsError errors={user_members.error.clone()} />
                            }
                        }
                    }
                </div>
                {
                    if (*show_modal).clone() {
                        html! {
                            <AddMemberModal client_id={id.clone()} callback={modal_callback} />
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    } else {
        html! {
            <PropsError errors={one_user.error.clone()} />
        }
    }
}
