use yew::prelude::*;

#[function_component(RegisterUser)]
pub fn register_user() -> Html {
    html! {
        <div class="flex flex-col gap-5">
            <h1 class="text-2xl font-bold">{ "User Registration" }</h1>
            <form>
                <div class="grid gap-6 mb-6 md:grid-cols-2">
                    <div>
                        <label for="first_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "First name" }</label>
                        <input type="text" id="first_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user first name" required=true />
                    </div>
                    <div>
                        <label for="company" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Middle name" }</label>
                        <input type="text" id="company" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user middle name" required=true />
                    </div>
                    <div>
                        <label for="last_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Last name" }</label>
                        <input type="text" id="last_name" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user last name" required=true />
                    </div>
                    <div>
                        <label for="tower" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Tower" }</label>
                        <input type="text" id="tower" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter tower for user" required=true />
                    </div>
                    <div>
                        <label for="room" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Room" }</label>
                        <input type="text" id="room" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter room for user" required=true />
                    </div>
                    <div>
                        <label for="package" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Select an option" }</label>
                        <select id="package" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                            <option selected=true>{ "Choose a package" }</option>
                            <option value="normal">{ "Normal" }</option>
                            <option value="vip">{ "VIP" }</option>
                            <option value="vvip">{ "VVIP" }</option>
                        </select>
                    </div>
                </div>
                <div class="mb-6">
                    <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Email address" }</label>
                    <input type="email" id="email" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter user email address" required=true />
                </div>
                <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{ "Submit" }</button>
            </form>
        </div>
    }
}
