use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{
    components::home::admins::{
        monitoring_table::MonitoringTable, search_monitoring::SearchMonitorComponent,
    },
    services::monitoring::monitoring_select,
    types::MonitorForSelectWrapper,
};

#[function_component(MonitoringPage)]
pub fn monitoring_page() -> Html {
    let monitorings = use_state(|| MonitorForSelectWrapper { monitors: vec![] });
    let is_not_empty = use_state(|| true);

    let req_monitors = use_async_with_options(
        async move { monitoring_select().await },
        UseAsyncOptions::enable_auto(),
    );

    let reqmonitorings = monitorings.clone();
    let isnot_empty = is_not_empty.clone();
    let search_callback = Callback::from(move |filtered_data: MonitorForSelectWrapper| {
        isnot_empty.set(false);
        reqmonitorings.set(filtered_data);
    });

    if let Some(data) = &req_monitors.data {
        html! {
            <div class="flex flex-col gap-5">
                <div class="flex items-center justify-between">
                    <h1 class="text-2xl font-bold">{ "Real time monitoring" }</h1>
                    <SearchMonitorComponent search_monitor={data.clone()} callback={search_callback.clone()}/>
                </div>
                <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                    {
                        if !(*is_not_empty).clone() {
                            html! {
                                <MonitoringTable data={(*data).clone()} />
                            }
                        } else {
                            html! {
                                <MonitoringTable data={(*monitorings).clone()} />
                            }
                        }
                    }
                </div>
            </div>
        }
    } else {
        html! {
            <div class="flex flex-col gap-5">
                <div class="flex items-center justify-between">
                    <h1 class="text-2xl font-bold">{ "Real time monitoring" }</h1>
                    <div class="relative">
                        <input class="w-full h-10 pl-10 pr-5 text-sm rounded-full appearance-none focus:outline-none bg-gray-100 dark:bg-gray-800 border border-gray-100 dark:border-gray-700" placeholder="Search..."/>
                        <button class="absolute top-0 left-0 mt-3 ml-4">
                            <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 stroke-current" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                        </button>
                    </div>
                </div>
                <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                    <div class="w-full flex h-64 flex items-center justify-center">
                        <p class="text-4xl text-semibold">{ "No monitoring data" }</p>
                    </div>
                </div>
            </div>
        }
    }
}
