use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub callback: Callback<bool>,
    pub confirm_cb: Callback<()>,
}

#[function_component(ConfirmModal)]
pub fn confirm_modal(props: &Props) -> Html {
    let cancel_modal = {
        let cb = props.callback.clone();
        Callback::from(move |_| {
            cb.emit(false);
        })
    };

    let confirm_modal = {
        let ccb = props.confirm_cb.clone();
        Callback::from(move |_| {
            ccb.emit(());
        })
    };

    html! {
        <div class="overflow-y-auto overflow-x-hidden fixed z-50 flex justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full">
            <div class="h-screen w-screen absolute bg-black opacity-60">
            </div>
            <div class="relative p-4 w-full max-w-2xl max-h-full">
                <div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
                    <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
                            { "Confirm remove" }
                        </h3>
                        <button type="button" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white" data-modal-hide="static-modal" onclick={cancel_modal.clone()} >
                            <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                            </svg>
                            <span class="sr-only">{ "Close modal" }</span>
                        </button>
                    </div>
                    <div class="p-4 md:p-5 space-y-4">
                        <p class="text-center leading-relaxed text-gray-500 dark:text-gray-400">
                            { "Are you sure you want to delete this client?" }
                        </p>
                    </div>
                    <div class="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600">
                        <button data-modal-hide="static-modal" type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={confirm_modal.clone()}>{ "Confirm" }</button>
                        <button data-modal-hide="static-modal" type="button" class="py-2.5 px-5 ms-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700" onclick={cancel_modal.clone()} >{ "Cancel" }</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
