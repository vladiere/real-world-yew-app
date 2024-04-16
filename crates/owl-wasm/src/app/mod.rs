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
use self::home::device_info::DeviceInfo;
use self::home::devices::DevicesPage;
use self::home::monitoring::MonitoringPage;
use self::home::register_admin::RegisterAdmin;
use self::home::register_user::RegisterUser;
use self::home::users::UsersPage;
use self::home::view_admin::ViewAdmin;
use self::home::view_user::ViewUser;
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
    #[at("/devices/info/:id")]
    DevicesInfo { id: i64 },
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
    #[at("/admins/view/:id")]
    AdminView { id: i64 },
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
    #[at("/users/view/:id")]
    UserView { id: i64 },
    #[not_found]
    #[at("/users/404")]
    NotFound,
}

fn switch_main(route: AppRoutes) -> Html {
    match route {
        AppRoutes::Home => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
                    <DashboardPage />
                </div>
            </SideLayout>
        },
        AppRoutes::Login => html! { <Login /> },
        AppRoutes::ForgotPassword => html! { <ForgotPassword /> },
        AppRoutes::Monitoring => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
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
                <div class="h-screen overflow-y-scroll">
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
                <div class="h-screen overflow-y-scroll">
                    <DevicesPage />
                </div>
            </SideLayout>
        },
        DeviceRoutes::DeviceRegister => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
                    <DeviceAdd />
                </div>
            </SideLayout>
        },
        DeviceRoutes::DevicesInfo { id } => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
                    <DeviceInfo device_id={id} />
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
                <div class="h-screen overflow-y-scroll">
                    <UsersPage />
                </div>
            </SideLayout>
        },
        UserRoutes::UserView { id } => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
                    <ViewUser user_id={id}/>
                </div>
            </SideLayout>
        },
        UserRoutes::UserRegister => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
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
                <div class="h-screen overflow-y-scroll">
                    <AdminsPage />
                </div>
            </SideLayout>
        },
        AdminRoutes::AdminView { id } => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
                    <ViewAdmin admin_id={id}/>
                </div>
            </SideLayout>
        },
        AdminRoutes::AdminRegister => html! {
            <SideLayout>
                <div class="h-screen overflow-y-scroll">
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
        <div class="h-[calc(100vh-1rem)] w-screen text-gray-600 bg-gray-200">
            <BrowserRouter>
                <UserContextProvider>
                    <Switch<AppRoutes> render={switch_main} />
                </UserContextProvider>
            </BrowserRouter>
        </div>
    }
}
