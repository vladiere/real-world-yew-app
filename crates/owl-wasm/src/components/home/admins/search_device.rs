use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::types::DeviceForSelectWrapper;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub search_device: DeviceForSelectWrapper,
    pub callback: Callback<DeviceForSelectWrapper>,
}

#[function_component(SearchDeviceComponent)]
pub fn search_component(props: &Props) -> Html {
    let search_query = use_state(|| String::new());

    let on_search_input = {
        let search_query = search_query.clone();
        let callback = props.callback.clone();
        let devices = props.search_device.to_owned();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let query = input.value();
            search_query.set(query.clone());
            let filtered_devices: DeviceForSelectWrapper = DeviceForSelectWrapper {
                devices: devices
                    .devices
                    .iter()
                    .filter(|device| {
                        device
                            .device_name
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                            || device
                                .device_tower
                                .to_lowercase()
                                .contains(&query.to_lowercase())
                            || device
                                .device_room
                                .to_lowercase()
                                .contains(&query.to_lowercase())
                            || device
                                .device_state
                                .to_lowercase()
                                .contains(&query.to_lowercase())
                            || device
                                .device_id
                                .to_lowercase()
                                .contains(&query.to_lowercase())
                    })
                    .cloned()
                    .collect(),
            };
            callback.emit(filtered_devices);
        })
    };

    html! {
        <div class="relative">
            <input class="w-full h-10 pl-10 pr-5 text-sm rounded-full appearance-none focus:outline-none bg-gray-100 dark:bg-gray-800 border border-gray-100 dark:border-gray-700" placeholder="Search..." oninput={on_search_input} value={(*search_query).clone()} />
            <button class="absolute top-0 left-0 mt-3 ml-4">
                <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 stroke-current" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
            </button>
        </div>
    }
}
