use web_sys::{console, js_sys::JsString};
use yew::prelude::*;

use crate::{assets::icons::ArrowLeft, hooks::use_user_context};
use yew_icons::{Icon, IconId};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SideLayout)]
pub fn side_layout(props: &Props) -> Html {
    let open_state = use_state(|| false);
    let def_class =
        use_state(|| "h-screen p-5 pt-8 relative duration-300 w-64 border-r border-slate-200");
    let def_class_arrow = use_state(|| {
        "bg-white w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer"
    });
    let h1_class = use_state(|| "text-8md origin-left font-medium uppercase");
    let li_class = use_state(|| {
        "text-sm flex items-center gap-x-4 cursor-pointer p-2 hover:bg-slate-700 rounded-md mt-2 duration-300"
    });
    let span_class = use_state(|| "text-base font-medium flex-1");

    let set_open = {
        let def_class = def_class.clone();
        let open_state = open_state.clone();
        let def_class_arrow = def_class_arrow.clone();
        let h1_class = h1_class.clone();
        let span_class = span_class.clone();

        Callback::from(move |_| {
            if *open_state {
                open_state.set(false);
                def_class
                    .set("h-screen p-4 pt-8 relative duration-200 w-64 border-r border-slate-300");
                def_class_arrow.set("bg-white w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer");
                h1_class.set("text-8md origin-left font-medium uppercase duration-300");
                span_class.set("text-base font-medium flex-1 duration-300");
            } else {
                open_state.set(true);
                def_class
                    .set("h-screen p-4 pt-8 relative duration-300 w-20 border-r border-slate-300");
                def_class_arrow.set("bg-white w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer rotate-180");
                h1_class.set("text-8md origin-left font-medium uppercase duration-300 scale-0");
                span_class.set("text-base font-medium flex-1 duration-300 scale-0");
            }
        })
    };

    let user_ctx = use_user_context();

    let onclick = {
        Callback::from(move |_| {
            // Logout the current admin
            user_ctx.logout();
        })
    };

    html! {
        <div class="flex">
            <div class={*def_class.clone()}>
                <div class={*def_class_arrow.clone()} onclick={set_open}>
                    <ArrowLeft />
                </div>
                <div class="inline-flex items-center">
                    <img src="public/images/owl-logo.png" class="w-[50px] block float-left mr-2 duration-300" alt="OWL logo" />
                    <h1 class={*h1_class.clone()}>{ "owl web server" }</h1>
                </div>
                <ul class="pt-2">
                    <li class={*li_class.clone()}>
                        <span class="text-2xl block float-left">
                            <Icon icon_id={IconId::LucideLayoutDashboard} width={"28px".to_owned()} height={"28px".to_owned()} />
                        </span>
                        <span class={*span_class.clone()}>{ "Dashboard" }</span>
                    </li>
                    <li class={*li_class.clone()}>
                        <span class="text-2xl block float-left">
                            <Icon icon_id={IconId::FontAwesomeSolidClipboardUser} width={"28px".to_owned()} height={"28px".to_owned()} />
                        </span>
                        <span class={*span_class.clone()}>{ "Profiling" }</span>
                    </li>
                    <li class={*li_class.clone()}>
                        <span class="text-2xl block float-left">
                            <Icon icon_id={IconId::LucideWebcam} width={"28px".to_owned()} height={"28px".to_owned()} />
                        </span>
                        <span class={*span_class.clone()}>{ "Devices" }</span>
                    </li>
                    <li class={*li_class.clone()}>
                        <span class="text-2xl block float-left">
                            <Icon icon_id={IconId::LucideMonitor} width={"28px".to_owned()} height={"28px".to_owned()} />
                        </span>
                        <span class={*span_class.clone()}>{ "RT Monitoring" }</span>
                    </li>
                    <li class={*li_class.clone()} {onclick}>
                        <span class="text-2xl block float-left">
                            <Icon icon_id={IconId::LucideLogOut} width={"28px".to_owned()} height={"28px".to_owned()} />
                        </span>
                        <span class={*span_class.clone()}>{ "Logout" }</span>
                    </li>
                </ul>
            </div>
            <div class="p-5">
                { for props.children.iter() }
            </div>
        </div>
    }
}
