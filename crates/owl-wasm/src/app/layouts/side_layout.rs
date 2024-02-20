use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::AppRoutes, assets::icons::ArrowLeft, hooks::use_user_context};
use yew_icons::{Icon, IconId};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SideLayout)]
pub fn side_layout(props: &Props) -> Html {
    let open_state = use_state(|| false);
    let def_class =
        use_state(|| "h-screen p-5 pt-8 relative duration-300 w-64 border-r border-slate-700");
    let def_class_arrow = use_state(|| {
        "bg-slate-700 w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer"
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
                    .set("h-screen p-4 pt-8 relative duration-200 w-64 border-r border-slate-700");
                def_class_arrow.set("bg-slate-700 w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer");
                h1_class.set("text-8md origin-left font-medium uppercase duration-300");
                span_class.set("text-base font-medium flex-1 duration-300");
            } else {
                open_state.set(true);
                def_class
                    .set("h-screen p-4 pt-8 relative duration-300 w-20 border-r border-slate-700");
                def_class_arrow.set("bg-slate-700 w-[25px] rounded-full absolute -right-3 top-9 border border-[#16B1BB] cursor-pointer rotate-180");
                h1_class.set("text-8md origin-left font-medium uppercase duration-300 scale-0");
                span_class.set("text-base font-medium flex-1 duration-300 scale-0");
            }
        })
    };

    let user_ctx = use_user_context();

    html! {
        <div class="flex flex-row dark:bg-gray-800 dark:text-gray-200">
            <div class={*def_class.clone()}>
                <div class={*def_class_arrow.clone()} onclick={set_open}>
                    <ArrowLeft />
                </div>
                <div class="inline-flex items-center">
                    <img src="public/images/owl-logo.png" class="w-[50px] block float-left mr-2 duration-300" alt="OWL logo" />
                    <h1 class={*h1_class.clone()}>{ "owl web server" }</h1>
                </div>
                <ul class="pt-2">
                    <li>
                        <Link<AppRoutes> to={AppRoutes::Home} classes={*li_class.clone()}>
                            <span class="text-2xl block float-left">
                                <Icon icon_id={IconId::LucideLayoutDashboard} width={"28px".to_owned()} height={"28px".to_owned()} />
                            </span>
                            <span class={*span_class.clone()}>{ "Dashboard" }</span>
                        </Link<AppRoutes>>
                    </li>
                    <li>
                        <Link<AppRoutes> to={AppRoutes::Users} classes={*li_class.clone()}>
                            <span class="text-2xl block float-left">
                                <Icon icon_id={IconId::FontAwesomeSolidUsers} width={"28px".to_owned()} height={"28px".to_owned()} />
                            </span>
                            <span class={*span_class.clone()}>{ "Users" }</span>
                        </Link<AppRoutes>>
                    </li>
                    {
                        if user_ctx.role_user.clone() == "Super".to_string() {
                            html! {
                                <li>
                                    <Link<AppRoutes> to={AppRoutes::Admins} classes={*li_class.clone()}>
                                        <span class="text-2xl block float-left">
                                            <Icon icon_id={IconId::FontAwesomeSolidUsersGear} width={"28px".to_owned()} height={"28px".to_owned()} />
                                        </span>
                                        <span class={*span_class.clone()}>{ "Admins" }</span>
                                    </Link<AppRoutes>>
                                </li>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <li>
                        <Link<AppRoutes> to={AppRoutes::Devices} classes={*li_class.clone()}>
                            <span class="text-2xl block float-left">
                                <Icon icon_id={IconId::LucideWebcam} width={"28px".to_owned()} height={"28px".to_owned()} />
                            </span>
                            <span class={*span_class.clone()}>{ "Devices" }</span>
                        </Link<AppRoutes>>
                    </li>
                    <li>
                        <Link<AppRoutes> to={AppRoutes::Monitoring} classes={*li_class.clone()}>
                            <span class="text-2xl block float-left">
                                <Icon icon_id={IconId::LucideMonitor} width={"28px".to_owned()} height={"28px".to_owned()} />
                            </span>
                            <span class={*span_class.clone()}>{ "RT Monitoring" }</span>
                        </Link<AppRoutes>>
                    </li>
                    <li>
                        <Link<AppRoutes> to={AppRoutes::Settings} classes={*li_class.clone()}>
                            <span class="text-2xl block float-left">
                                <Icon icon_id={IconId::HeroiconsMiniSolidCog8Tooth} width={"28px".to_owned()} height={"28px".to_owned()} />
                            </span>
                            <span class={*span_class.clone()}>{ "Settings" }</span>
                        </Link<AppRoutes>>
                    </li>
                </ul>
            </div>
            <div class="p-8 w-full">
                { for props.children.iter() }
            </div>
        </div>
    }
}
