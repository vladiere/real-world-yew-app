//! Admin context provider.

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::error::Error;
use crate::services::{auth::*, get_access, set_token};
use crate::types::AdminInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// Admin context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(AdminInfo::default);
    let current_user = use_async(async move { current().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_access().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with(current_user, move |current_user| {
            if let Some(user_info) = &current_user.data {
                user_ctx.set(user_info.admin.clone());
            }

            if let Some(error) = &current_user.error {
                match error {
                    Error::Unauthorized(_) | Error::Forbidden => {
                        set_token("access_token", None);
                        set_token("refresh_token", None);
                    }
                    _ => (),
                }
            }
            || ()
        })
    }

    html! {
        <ContextProvider<UseStateHandle<AdminInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<AdminInfo>>>
    }
}
