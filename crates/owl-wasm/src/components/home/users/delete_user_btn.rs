use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{components::confirm_modal::ConfirmModal, services::accounts::delete_user};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub user_id: i64,
    pub callback: Callback<i64>,
}

#[function_component(DeleteUserButton)]
pub fn delete_user_button(props: &Props) -> Html {
    let btn_state = use_state(|| false);
    let is_show = use_state(|| false);

    let on_remove = {
        let user_id = props.user_id.clone();
        use_async(async move { delete_user(user_id).await })
    };

    let onclick = {
        let on_remove = on_remove.clone();
        let btn_state = btn_state.clone();
        Callback::from(move |_| {
            on_remove.run();
            btn_state.set(true)
        })
    };

    let show_modal = {
        let is_show = is_show.clone();
        Callback::from(move |_| {
            is_show.set(true);
        })
    };

    let callback_modal = {
        let is_show = is_show.clone();
        Callback::from(move |state| {
            is_show.set(state);
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

    html! {
        <>
            <button type="button" disabled={(*btn_state).clone()} class="font-medium text-red-500 dark:text-red-500" onclick={show_modal.clone()}>
                <Icon icon_id={IconId::BootstrapTrash} width={"20px".to_owned()} height={"20px".to_owned()}/>
            </button>
            {
                if (*is_show).clone() {
                    html! {
                        <ConfirmModal callback={callback_modal.clone()} confirm_cb={onclick} />
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
