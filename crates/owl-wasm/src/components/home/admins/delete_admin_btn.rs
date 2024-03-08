use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{components::confirm_modal::ConfirmModal, services::accounts::remove_one_admin};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub admin_id: i64,
    pub callback: Callback<i64>,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &Props) -> Html {
    let btn_state = use_state(|| false);
    let is_show = use_state(|| false);

    let on_remove = {
        let admin_id = props.admin_id.clone();
        use_async(async move { remove_one_admin(admin_id).await })
    };

    let onclick = {
        let on_remove = on_remove.clone();
        let btn_state = btn_state.clone();

        Callback::from(move |_| {
            btn_state.set(true);
            on_remove.run();
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
            (
                props.callback.clone(),
                on_remove.clone(),
                props.admin_id,
                btn_state.clone(),
            ),
            move |(callback, removed, admin_id, btn_state)| {
                if removed.data.is_some() {
                    callback.emit(*admin_id);
                }
                btn_state.set(false);
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
