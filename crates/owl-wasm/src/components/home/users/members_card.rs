use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::types::AllMembersInfoWrapper;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub members: AllMembersInfoWrapper,
}

#[function_component(MembersCard)]
pub fn members_card(props: &Props) -> Html {
    let data_members = &props.members;

    html! {
        {
            for data_members.members.iter().map(|member| {
                html! {
                    <div class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 capitalize">
                        <h5 class="mb-2 text-8md font-bold tracking-tight text-gray-900 dark:text-white">
                        { "Fullname: " }{ format!("{}, {} {}.", &member.lastname, &member.firstname, &member.middlename.chars().next().unwrap_or_default()) }
                        </h5>
                        <div class="flex gap-10">
                            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{ "Age: " }{ &member.age }</p>
                            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{ "Gender: " }{ &member.gender }</p>
                        </div>
                        <p class="mb-3 font-normal text-2md text-gray-500 dark:text-gray-400">{ "Date Enrolled: " }{ &member.date_enrolled }</p>
                        <button class="font-medium text-red-500 dark:text-red-500">
                            <Icon icon_id={IconId::BootstrapTrash} width={"20px".to_owned()} height={"20px".to_owned()}/>
                        </button>
                    </div>
                }
            })
        }
    }
}
