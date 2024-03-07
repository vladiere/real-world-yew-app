use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{
    components::home::users::confirm_modal::ConfirmModal, services::accounts::delete_user,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: i64,
    pub callback: Callback<i64>,
}

#[function_component(DeleteUserButton)]
pub fn delete_user_button(props: &Props) -> Html {
    let show_modal = use_state(|| false);

    let on_remove = {
        let user_id = props.user_id.clone();
        use_async(async move { delete_user(user_id).await })
    };

    let onclick = {
        let on_remove = on_remove.clone();
        Callback::from(move |_| {
            on_remove.run();
        })
    };

    {
        use_effect_with(
            (props.callback.clone(), on_remove.clone(), props.user_id),
            move |(callback, removed, user_id)| {
                if removed.data.is_some() {
                    callback.emit(*user_id);
                }
                || ()
            },
        )
    }

    let show_confirm = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(true))
    };

    let modal_callback = {
        let show_modal = show_modal.clone();
        Callback::from(move |modal_state| show_modal.set(modal_state))
    };

    html! {
        <>
            <button class="font-medium text-red-500 dark:text-red-500" onclick={show_confirm}>
                <Icon icon_id={IconId::BootstrapTrash} width={"20px".to_owned()} height={"20px".to_owned()}/>
            </button>
            {
                if (*show_modal).clone() {
                    html! {
                        <ConfirmModal callback={modal_callback} confirm_cb={onclick} />
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
