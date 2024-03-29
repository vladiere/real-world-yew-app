use std::fmt;
use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoutes;
use crate::services::set_token;
use crate::types::CurrentAdminInfo;

/// State handle for the [`use_user_context`] hook.
pub struct UseUserContextHandle {
    inner: UseStateHandle<CurrentAdminInfo>,
    navigator: Navigator,
}

impl UseUserContextHandle {
    pub fn login(&self, value: CurrentAdminInfo) {
        // Set global token after logged in
        set_token("access_token", Some(value.access_token.clone()));
        set_token("refresh_token", Some(value.refresh_token.clone()));
        self.inner.set(value);
        // Redirect to home page
        self.navigator.push(&AppRoutes::Home);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        set_token("access_token", None);
        set_token("refresh_token", None);
        self.inner.set(CurrentAdminInfo::default());
        // Redirect to home page
        self.navigator.push(&AppRoutes::Login);
    }
}

impl Deref for UseUserContextHandle {
    type Target = CurrentAdminInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            navigator: self.navigator.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

/// This hook is used to manage user context.
#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<CurrentAdminInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
