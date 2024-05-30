use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{
    app::DeviceRoutes,
    components::{
        home::admins::{device_table::DevicesTable, search_device::SearchDeviceComponent},
        loading::LoadingComponent,
    },
    services::devices::select_devices,
    types::DeviceForSelectWrapper,
};

#[function_component(DevicesPage)]
pub fn devices_page() -> Html {
    let res_devices = use_state(|| DeviceForSelectWrapper { devices: vec![] });
    let is_not_empty = use_state(|| true);

    let devices = use_async_with_options(
        async move { select_devices().await },
        UseAsyncOptions::enable_auto(),
    );

    let resdevices = res_devices.clone();
    let isnot_empty = is_not_empty.clone();
    let search_callback = Callback::from(move |filtered_data: DeviceForSelectWrapper| {
        isnot_empty.set(false);
        resdevices.set(filtered_data);
    });

    let device_callback = {
        let devices = devices.clone();
        let res_devices = res_devices.clone();
        Callback::from(move |_| {
            devices.run();
            if let Some(data) = &devices.data {
                res_devices.set(data.clone());
            }
        })
    };

    html! {
        <div class="flex flex-col gap-5">
            <h1 class="text-2xl font-bold">{ "Devices List" }</h1>
            <div class="flex flex-row gap-10 justify-between">
                <Link<DeviceRoutes> to={DeviceRoutes::DeviceRegister} classes="flex items-center gap-2 middle none center mr-4 rounded-lg bg-green-500 py-3 px-5 font-sans text-xs font-bold uppercase text-white shadow-md shadow-green-500/20 transition-all hover:shadow-lg hover:shadow-green-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none">
                    <Icon icon_id={IconId::BootstrapNodePlusFill} width={"24px".to_owned()} height={"24px".to_owned()}/>
                    { "Add new Device" }
                </Link<DeviceRoutes>>

                {
                    if let Some(device) = &devices.data {
                        html! {
                            <SearchDeviceComponent search_device={(*device).clone()} callback={search_callback.clone()} />
                        }
                    } else {
                        html! {
                            <div class="rounded-lg text-center">
                                <span class="text-2xl font-medium">{ "No data available to search" }</span>
                            </div>
                        }
                    }
                }
            </div>
            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                {
                    if devices.loading {
                        html! {
                            <LoadingComponent />
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if *is_not_empty.clone() {
                        if let Some(data) = &devices.data {
                            html! {
                                <DevicesTable data={data.clone()} callback={device_callback.clone()} />
                            }
                        } else {
                            html! {
                                <DevicesTable data={(*res_devices).clone()} callback={device_callback.clone()} />
                            }
                        }
                    } else {
                        html! {
                            <DevicesTable data={(*res_devices).clone()} callback={device_callback.clone()} />
                        }
                    }
                }
            </div>
        </div>
    }
}
