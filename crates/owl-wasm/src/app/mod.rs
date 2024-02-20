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
    #[at("/users")]
    Users,
    #[at("/devices")]
    Devices,
    #[at("/monitoring")]
    Monitoring,
    #[at("/settings")]
    Settings,
    #[at("/admins")]
    Admins,
    #[at("/404")]
    NotFound,
}

/// Admin Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AdminRoutes {
    #[at("/register")]
    AdminRegister,
    #[at("/update")]
    AdminUpdate,
    #[at("/admins")]
    Admins,
    #[at("/404")]
    NotFound,
}

/// User Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum UserRoutes {
    #[at("/users")]
    Users,
    #[at("/register")]
    UserRegister,
    #[at("/update")]
    UserUpdate,
    #[at("/404")]
    NotFound,
}

pub fn switch_main(route: AppRoutes) -> Html {
    match route {
        AppRoutes::Home => html! {
            <SideLayout>
                <DashboardPage />
            </SideLayout>
        },
        AppRoutes::Login => html! { <Login /> },
        AppRoutes::ForgotPassword => html! { <ForgotPassword /> },
        AppRoutes::Monitoring => html! {
            <SideLayout>
                <MonitoringPage />
            </SideLayout>
        },
        AppRoutes::Devices => html! {
            <SideLayout>
                <DevicesPage />
            </SideLayout>
        },
        AppRoutes::Admins => html! { <Switch<AdminRoutes> render={switch_admin} /> },
        AppRoutes::Users => html! { <Switch<UserRoutes> render={switch_user} /> },
        AppRoutes::Settings => html! {
            <SideLayout>
                <SettingsPage />
            </SideLayout>
        },
        AppRoutes::NotFound => html! { <NotFound /> },
    }
}

pub fn switch_user(route: UserRoutes) -> Html {
    match route {
        UserRoutes::Users => html! {
            <SideLayout>
                <UsersPage />
            </SideLayout>
        },
        UserRoutes::UserUpdate => html! {
            <SideLayout>
                <UpdateUser />
            </SideLayout>
        },
        UserRoutes::UserRegister => html! {
            <SideLayout>
                <RegisterUser />
            </SideLayout>
        },
        UserRoutes::NotFound => html! { <NotFound /> },
    }
}

pub fn switch_admin(route: AdminRoutes) -> Html {
    match route {
        AdminRoutes::Admins => html! {
            <SideLayout>
                <AdminsPage />
            </SideLayout>
        },
        AdminRoutes::AdminUpdate => html! {
            <SideLayout>
                <UpdateAdmin />
            </SideLayout>
        },
        AdminRoutes::AdminRegister => html! {
            <SideLayout>
                <RegisterAdmin />
            </SideLayout>
        },
        AdminRoutes::NotFound => html! { <NotFound /> },
    }
}

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="h-screen w-screen text-white">
            <BrowserRouter>
                <UserContextProvider>
                    <Switch<AppRoutes> render={switch_main} />
                </UserContextProvider>
            </BrowserRouter>
        </div>
    }
}
