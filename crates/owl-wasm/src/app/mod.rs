//! Routes by yew_router.

pub mod editor;
pub mod forgot_password;
pub mod home;
pub mod layouts;
pub mod login;
pub mod not_found;
pub mod profile;
pub mod register;
pub mod settings;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::layouts::SideLayout;
use crate::components::user_context_provider::UserContextProvider;

use login::Login;

use self::forgot_password::ForgotPassword;
use self::home::admins::AdminsPage;
use self::home::dashboard::DashboardPage;
use self::home::device_add::DeviceAdd;
use self::home::device_update::DeviceUpdate;
use self::home::devices::DevicesPage;
use self::home::monitoring::MonitoringPage;
use self::home::register_admin::RegisterAdmin;
use self::home::register_user::RegisterUser;
use self::home::update_admin::UpdateAdmin;
use self::home::update_user::UpdateUser;
use self::home::users::UsersPage;
use self::not_found::NotFound;
use self::settings::SettingsPage;

/// App Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoutes {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/devices")]
    DevicesRoot,
    #[at("/devices/*")]
    Devices,
    #[at("/monitoring")]
    Monitoring,
    #[at("/settings")]
    Settings,
    #[at("/admins")]
    AdminsRoot,
    #[at("/admins/*")]
    Admins,
    #[at("/users")]
    UsersRoot,
    #[at("/users/*")]
    Users,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Device Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum DeviceRoutes {
    #[at("/devices")]
    DevicesList,
    #[at("/devices/add")]
    DeviceRegister,
    #[at("/admins/update")]
    DeviceUpdate,
    #[not_found]
    #[at("/devices/404")]
    NotFound,
}

/// Admin Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AdminRoutes {
    #[at("/admins")]
    AdminsList,
    #[at("/admins/register")]
    AdminRegister,
    #[at("/admins/update")]
    AdminUpdate,
    #[not_found]
    #[at("/admins/404")]
    NotFound,
}

/// User Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum UserRoutes {
    #[at("/users")]
    UsersList,
    #[at("/users/register")]
    UserRegister,
    #[at("/users/update")]
    UserUpdate,
    #[not_found]
    #[at("/users/404")]
    NotFound,
}

fn switch_main(route: AppRoutes) -> Html {
    match route {
        AppRoutes::Home => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <DashboardPage />
                </div>
            </SideLayout>
        },
        AppRoutes::Login => html! { <Login /> },
        AppRoutes::ForgotPassword => html! { <ForgotPassword /> },
        AppRoutes::Monitoring => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <MonitoringPage />
                </div>
            </SideLayout>
        },
        AppRoutes::Devices | AppRoutes::DevicesRoot => {
            html! { <Switch<DeviceRoutes> render={switch_device} />}
        }
        AppRoutes::Admins | AppRoutes::AdminsRoot => {
            html! { <Switch<AdminRoutes> render={switch_admin} /> }
        }
        AppRoutes::Users | AppRoutes::UsersRoot => {
            html! { <Switch<UserRoutes> render={switch_user} /> }
        }
        AppRoutes::Settings => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <SettingsPage />
                </div>
            </SideLayout>
        },
        AppRoutes::NotFound => html! { <NotFound /> },
    }
}

fn switch_device(route: DeviceRoutes) -> Html {
    match route {
        DeviceRoutes::DevicesList => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <DevicesPage />
                </div>
            </SideLayout>
        },
        DeviceRoutes::DeviceRegister => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <DeviceAdd />
                </div>
            </SideLayout>
        },
        DeviceRoutes::DeviceUpdate => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <DeviceUpdate />
                </div>
            </SideLayout>
        },
        DeviceRoutes::NotFound => html! { <Redirect<AppRoutes> to={AppRoutes::NotFound} /> },
    }
}

fn switch_user(route: UserRoutes) -> Html {
    match route {
        UserRoutes::UsersList => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <UsersPage />
                </div>
            </SideLayout>
        },
        UserRoutes::UserUpdate => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <UpdateUser />
                </div>
            </SideLayout>
        },
        UserRoutes::UserRegister => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <RegisterUser />
                </div>
            </SideLayout>
        },
        UserRoutes::NotFound => html! { <Redirect<AppRoutes> to={AppRoutes::NotFound} /> },
    }
}

fn switch_admin(route: AdminRoutes) -> Html {
    match route {
        AdminRoutes::AdminsList => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <AdminsPage />
                </div>
            </SideLayout>
        },
        AdminRoutes::AdminUpdate => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <UpdateAdmin />
                </div>
            </SideLayout>
        },
        AdminRoutes::AdminRegister => html! {
            <SideLayout>
                <div class="h-full overflow-scroll">
                    <RegisterAdmin />
                </div>
            </SideLayout>
        },
        AdminRoutes::NotFound => html! { <Redirect<AppRoutes> to={AppRoutes::NotFound} /> },
    }
}

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="h-screen w-screen text-gray-600 bg-gray-200">
            <BrowserRouter>
                <UserContextProvider>
                    <Switch<AppRoutes> render={switch_main} />
                </UserContextProvider>
            </BrowserRouter>
        </div>
    }
}
