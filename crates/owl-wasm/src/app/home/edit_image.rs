use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct EditProps {
    pub user_id: i64,
}

#[function_component(EditImage)]
pub fn edit_image(props: &EditProps) -> Html {
    html! {
        <div class="w-full h-full flex items-center justify-center">
            <form class="flex flex-col gap-2 w-72">
                <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" for="file_input">{ "Upload file" }</label>
                <input class="block w-full text-sm text-gray-900 border border-gray-300 rounded-lg cursor-pointer bg-gray-50 dark:text-gray-400 focus:outline-none dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400" aria-describedby="file_input_help" id="file_input" type="file" />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-300" id="file_input_help">{ "PNG, JPG or JPEG (MAX. 800x400px)." }</p>
                <button type="submit" class="bg-green-500 text-dark px-3 py-2 rounded-md font-md">{ "Save" }</button>
            </form>
        </div>
    }
}
