use yew::prelude::*;

use crate::app::AppRoutes;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub btn_name: String,
    pub btn_link: AppRoutes,
    pub btn_logo: String,
}

#[function_component(SideButton)]
pub fn side_button(props: &Props) -> Html {
    html! {
        <div class="w-full text-[#16B1BB] text-8md py-3 px-2 text-center">
            <div class="w-full hover:bg-slate-800 py-2">
                <div class="flex items-center justify-center">
                    <div class="w-[60%]">
                        <div class="flex flex-row gap-2 items-center">
                            <img src={props.btn_logo.clone()} class="w-[5%]" atl={props.btn_logo.clone()}/>
                            // <Link<AppRoutes> to={props.btn_link}>
                            //     { props.btn_name }
                            // </Link>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
