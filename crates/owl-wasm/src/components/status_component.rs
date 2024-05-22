use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct StatusProps {
    pub stats: String,
}

#[function_component(StatusComponent)]
pub fn status_component(prop: &StatusProps) -> Html {
    let stats = prop.stats.clone();

    match stats.as_str() {
        "Active" => {
            html! {
                <div class="w-full flex items-center justify-center">
                    <div class="p-2 bg-green-500 rounded-full"></div>
                </div>
            }
        }
        _ => {
            html! {
                <div class="w-full flex items-center justify-center">
                    <div class="p-2 bg-yellow-500 rounded-full"></div>
                </div>
            }
        }
    }
}
