use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::hooks::use_navigator;

use crate::{
    app::UserRoutes,
    components::{
        alerts::SuccessAlert,
        home::users::{
            add_member_modal::AddMemberModal, delete_user_btn::DeleteUserButton,
            members_card::MembersCard, search_member::SearchMemberComponent,
        },
        props_error::PropsError,
    },
    services::{accounts::get_one_user, members::user_members},
    types::AllMembersInfoWrapper,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: i64,
}

#[function_component(ViewUser)]
pub fn view_user(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let id = props.user_id;
    let show_modal = use_state(|| false);
    let message = use_state(|| String::default());

    let res_members = use_state(|| AllMembersInfoWrapper { members: vec![] });
    let is_not_empty = use_state(|| false);

    let resmembers = res_members.clone();
    let isnot_empty = is_not_empty.clone();
    let search_callback = Callback::from(move |filtered_data: AllMembersInfoWrapper| {
        isnot_empty.set(true);
        resmembers.set(filtered_data);
    });

    let user_members = use_async_with_options(
        async move { user_members(id).await },
        UseAsyncOptions::enable_auto(),
    );

    let one_user = use_async_with_options(
        async move { get_one_user(id).await },
        UseAsyncOptions::enable_auto(),
    );

    // let is_show_modal = {
    //     let show_modal = show_modal.clone();
    //     Callback::from(move |_| {
    //         show_modal.set(true);
    //     })
    // };

    // let modal_callback = {
    //     let show_modal = show_modal.clone();
    //     let message = message.clone();
    //     Callback::from(move |(modal_state, msg)| {
    //         message.set(msg);
    //         show_modal.set(modal_state);
    //     })
    // };

    let delete_callback = Callback::from(move |_| navigator.push(&UserRoutes::UsersList));

    if let Some(user) = &one_user.data {
        html! {
            <div class="h-full w-full flex flex-col gap-10">
                <SuccessAlert alert_msg={(*message).clone()} />
                <h1 class="text-2xl font-bold">{ "Client Information details" }</h1>
                <div class="grid gap-5 md:grid-cols-3">
                    <div class="flex flex-col items-center justify-center gap-5">
                        <img src="https://img.freepik.com/free-vector/illustration-businessman_53876-5856.jpg?t=st=1709475853~exp=1709479453~hmac=c828619b151f43e33085076888676a6c564fe5b88e4a20baba5b94f2be13766d&w=740" alt="admin" class="rounded-lg h-52" />
                        <div>
                            <DeleteUserButton user_id={id.clone()} callback={delete_callback.clone()} />
                        </div>
                    </div>
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
            //     <div class="flex justify-between">
            //         <button class="flex items-center gap-2 middle none center mr-4 rounded-lg bg-green-500 py-3 px-5 font-sans text-xs font-bold uppercase text-white shadow-md shadow-green-500/20 transition-all hover:shadow-lg hover:shadow-green-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none" onclick={is_show_modal} >
            //             <Icon icon_id={IconId::FontAwesomeSolidUserPlus} width={"20px".to_owned()} height={"20px".to_owned()}/>
            //             { "Add new member" }
            //         </button>
            //         <h1 class="text-2xl font-bold">{ "Members List" }</h1>
            //         {
            //             if let Some(users) = &user_members.data {
            //
            //                 html! {
            //                     <SearchMemberComponent search_user={(*users).clone()} callback={search_callback.clone()} />
            //                 }
            //             } else {
            //                 html! {
            //                     <div class="rounded-lg text-center">
            //                         <span class="text-2xl font-medium">{ "No data available to search" }</span>
            //                     </div>
            //                 }
            //             }
            //         }
            //     </div>
            //     <div class="flex flex-wrap gap-5 items-center justify-center">
            //         {
            //             if !(*is_not_empty).clone() {
            //                 if let Some(members) = &user_members.data {
            //                     html! {
            //                         <MembersCard members={members.clone()} />
            //                     }
            //                 } else {
            //                     html! {
            //                         <PropsError errors={user_members.error.clone()} />
            //                     }
            //                 }
            //             } else {
            //                 html! {
            //                     <MembersCard members={(*res_members).clone()} />
            //                 }
            //             }
            //         }
            //     </div>
            //     {
            //         if (*show_modal).clone() {
            //             html! {
            //                 <AddMemberModal client_id={id.clone()} callback={modal_callback} />
            //             }
            //         } else {
            //             html! {}
            //         }
            //     }
            </div>
        }
    } else {
        html! {
            <PropsError errors={one_user.error.clone()} />
        }
    }
}
