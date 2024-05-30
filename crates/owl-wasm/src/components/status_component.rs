use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct StatusProps {
    pub stats: String,
}

#[function_component(StatusComponent)]
pub fn status_component(prop: &StatusProps) -> Html {
    let stats = prop.stats.clone();

    html! {
        <label class="inline-flex items-center me-5 cursor-pointer">
          <input type="checkbox" value="" class="sr-only peer" checked={ if stats.as_str() == "Active" { true } else { false } } />
          <div class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-focus:ring-4 peer-focus:ring-green-300 dark:peer-focus:ring-green-800 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-green-600"></div>
        </label>
    }
    // match stats.as_str() {
    //     "Active" => {
    //         html! {
    //             <div class="w-full flex items-center justify-center">
    //                 <div class="p-2 bg-green-500 rounded-full"></div>
    //             </div>
    //         }
    //     }
    //     _ => {
    //         html! {
    //             <div class="w-full flex items-center justify-center">
    //                 <div class="p-2 bg-yellow-500 rounded-full"></div>
    //             </div>
    //         }
    //     }
    // }
}
