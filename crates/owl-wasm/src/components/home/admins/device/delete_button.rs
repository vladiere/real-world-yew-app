use crate::{
    app::DeviceRoutes, components::confirm_modal::ConfirmModal, services::devices::delete_device,
};
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(DeleteDeviceButton)]
pub fn delete_button(props: &Props) -> Html {
    let is_show = use_state(|| false);
    let id = props.id;
    let navigator = use_navigator().expect("failed to create navigator");

    let on_remove = { use_async(async move { delete_device(id).await }) };

    let callback_modal = {
        let is_show = is_show.clone();
        Callback::from(move |state| {
            is_show.set(state);
        })
    };

    let show_modal = {
        let is_show = is_show.clone();
        Callback::from(move |_| {
            is_show.set(true);
        })
    };

    let onclick = {
        let on_remove = on_remove.clone();
        let is_show = is_show.clone();
        Callback::from(move |_| {
            is_show.set(true);
            on_remove.run();
            navigator.push(&DeviceRoutes::DevicesList);
        })
    };

    html! {
        <>
            <button type="button" class="bg-transparent outline-none py-2.5 px-5 text-sm font-medium text-red-500 hover:underline" onclick={show_modal.clone()} >{ "Remove" }</button>
            {
                if *is_show.clone() {
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
