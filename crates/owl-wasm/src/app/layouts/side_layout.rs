use yew::prelude::*;

use crate::assets::icons::{ArrowLeft, OwlLogo};
use crate::components::home::header::Header;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SideLayout)]
pub fn side_layout(props: &Props) -> Html {
    let open_state = use_state(|| false);
    let def_class =
        use_state(|| "h-screen p-5 pt-8 relative duration-300 w-72 border-r border-slate-200");
    let def_class_arrow = use_state(|| {
        "bg-white w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer"
    });

    let set_open = {
        let def_class = def_class.clone();
        let open_state = open_state.clone();
        let def_class_arrow = def_class_arrow.clone();

        Callback::from(move |_| {
            if *open_state {
                open_state.set(false);
                def_class
                    .set("h-screen p-5 pt-8 relative duration-300 w-72 border-r border-slate-200");
                def_class_arrow.set("bg-white w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer");
            } else {
                open_state.set(true);
                def_class
                    .set("h-screen p-5 pt-8 relative duration-300 w-20 border-r border-slate-200");
                def_class_arrow.set("bg-white w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer rotate-180");
            }
        })
    };

    html! {
        <div class="flex">
            <div class={*def_class.clone()}>
                <div class={*def_class_arrow.clone()} onclick={set_open}>
                    <ArrowLeft />
                </div>
                <div class="inline-flex">
                    <OwlLogo />
                </div>
            </div>
            <div class="p-5">
                { for props.children.iter() }
            </div>
        </div>
    }
}
