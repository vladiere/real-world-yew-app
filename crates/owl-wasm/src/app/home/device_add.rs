use yew::prelude::*;

#[function_component(DeviceAdd)]
pub fn device_add() -> Html {
    html! {
       <div class="flex flex-col gap-5">
            <h1 class="text-2xl font-bold">{ "Add new device" }</h1>
            <form>
                <div class="grid gap-6 mb-6 md:grid-cols-2">
                    <div>
                        <label for="ip_address" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "IP Address" }</label>
                        <input type="text" id="ip_address" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter device IP Address" required=true />
                    </div>
                    <div>
                        <label for="tower" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Tower" }</label>
                        <input type="text" id="tower" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter tower for device" required=true />
                    </div>
                    <div>
                        <label for="room" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Room" }</label>
                        <input type="text" id="room" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter room for device" required=true />
                    </div>
                    <div>
                        <label for="client_owner" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{ "Client Owner" }</label>
                        <input type="email" id="client_owner" class="outline-none bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter client owner" required=true />
                    </div>
                </div>
                <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{ "Submit" }</button>
            </form>
        </div>
    }
}
