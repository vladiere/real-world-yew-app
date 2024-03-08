use web_sys::HtmlInputElement;

use yew::prelude::*;

use crate::types::AllMembersInfoWrapper;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub search_user: AllMembersInfoWrapper,
    pub callback: Callback<AllMembersInfoWrapper>,
}

#[function_component(SearchMemberComponent)]
pub fn search_member_component(props: &Props) -> Html {
    let search_query = use_state(|| String::new());

    let on_search_input = {
        let search_query = search_query.clone();
        let callback = props.callback.clone();
        let users = props.search_user.to_owned();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let query = input.value();
            search_query.set(query.clone());
            let filtered_users: AllMembersInfoWrapper = AllMembersInfoWrapper {
                members: users
                    .members
                    .iter()
                    .filter(|admin| {
                        admin.firstname.contains(&query)
                            || admin.middlename.contains(&query)
                            || admin.lastname.contains(&query)
                            || admin.date_enrolled.contains(&query)
                            || admin.gender.contains(&query)
                    })
                    .cloned()
                    .collect(),
            };
            callback.emit(filtered_users);
        })
    };

    html! {
        <div class="relative">
            <input class="w-full h-10 pl-10 pr-5 text-sm rounded-full appearance-none focus:outline-none bg-gray-100 dark:bg-gray-800 border border-gray-100 dark:border-gray-700" placeholder="Search..." value={(*search_query).clone()} oninput={on_search_input} />
            <button class="absolute top-0 left-0 mt-3 ml-4">
                <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 stroke-current" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
            </button>
        </div>
    }
}
