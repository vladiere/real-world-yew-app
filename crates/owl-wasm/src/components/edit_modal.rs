use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(EditImageModal)]
pub fn edit_image_modal() -> Html {
    let show_modal = use_state(|| false);

    let show_edit_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(!(*show_modal)))
    };

    html! {
        <>
            <button class="text-green-500 p-[2px] rounded-md" onclick={show_edit_modal.clone()}>
                <Icon icon_id={IconId::FeatherEdit} width={"20px".to_owned()} height={"20px".to_owned()}/>
            </button>
            {
                if *show_modal.clone() {
                    html! {
                        <div class="absolute w-screen h-screen flex items-center justify-center relative">
                            <div class="bg-dark opacity-60 absolute w-screen h-screen"></div>
                            <div class="p-5 flex flex-col w-72">
                                { "This is the edit modal" }
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
