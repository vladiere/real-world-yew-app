use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

use crate::services::accounts::remove_one_admin;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub admin_id: i64,
    pub callback: Callback<i64>,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &Props) -> Html {
    let on_remove = {
        let admin_id = props.admin_id.clone();
        use_async(async move { remove_one_admin(admin_id).await })
    };

    let onclick = {
        let on_remove = on_remove.clone();
        Callback::from(move |_| {
            on_remove.run();
        })
    };

    {
        use_effect_with(
            (props.callback.clone(), on_remove.clone(), props.admin_id),
            move |(callback, removed, admin_id)| {
                if removed.data.is_some() {
                    callback.emit(*admin_id);
                }
                || ()
            },
        )
    }

    html! {
        <button class="font-medium text-red-500 dark:text-red-500" {onclick}>
            <Icon icon_id={IconId::BootstrapTrash} width={"20px".to_owned()} height={"20px".to_owned()}/>
        </button>
    }
}
