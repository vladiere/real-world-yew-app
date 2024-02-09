//! Routes by yew_router.

pub mod editor;
pub mod home;
pub mod login;
pub mod profile;
pub mod register;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::user_context_provider::UserContextProvider;

use login::Login;

use self::home::dashboard::DashboardPage;

/// App Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoutes {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/monitoring")]
    Monitoring,
    #[at("/report")]
    Report,
    #[at("/admin")]
    Admin,
    #[at("/404")]
    NotFound,
}

/// Admin Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AdminRoutes {
    #[at("/admin")]
    AdminPage,
    #[at("/register")]
    AdminRegister,
    #[at("/update")]
    AdminUpdate,
    #[at("/user")]
    User,
}

/// User Routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum UserRoutes {
    #[at("/register")]
    UserRegister,
    #[at("/update")]
    UserUpdate,
}

pub fn switch_main(route: AppRoutes) -> Html {
    match route {
        AppRoutes::Home => html! { <DashboardPage /> },
        AppRoutes::Login => html! { <Login /> },
        AppRoutes::ForgotPassword => html! { "forgot password page" },
        AppRoutes::Monitoring => html! { "monitoring page" },
        AppRoutes::Report => html! { "report page" },
        AppRoutes::Admin => html! { <Switch<AdminRoutes> render={switch_admin} /> },
        AppRoutes::NotFound => html! { "404 | Not found" },
    }
}

pub fn switch_admin(route: AdminRoutes) -> Html {
    match route {
        AdminRoutes::AdminPage => html! { "AdminPage" },
        AdminRoutes::AdminRegister => html! { "admin register" },
        AdminRoutes::AdminUpdate => html! { "admin update" },
        AdminRoutes::User => html! { <Switch<UserRoutes> render={switch_user} /> },
    }
}

pub fn switch_user(route: UserRoutes) -> Html {
    match route {
        UserRoutes::UserUpdate => html! { "user update" },
        UserRoutes::UserRegister => html! { "user register" },
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
