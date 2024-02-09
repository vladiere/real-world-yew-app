use yew::prelude::*;
use yew_hooks::use_async;

use crate::components::home::header::Header;
use crate::types::AdminInfo;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    html! {
        <div class="flex flex-col">
            <Header />
            <div class="h-full">
                {"Another content"}
            </div>
        </div>
    }
}
