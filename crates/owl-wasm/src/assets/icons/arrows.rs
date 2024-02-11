use yew::prelude::*;

#[function_component(ArrowLeft)]
pub fn arrow_left() -> Html {
    html! {
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M6 12H18M6 12L11 7M6 12L11 17" stroke="#16B1BB" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
    }
}

#[function_component(ArrowRight)]
pub fn arrow_right() -> Html {
    html! {
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" transform="matrix(-1, 0, 0, 1, 0, 0)"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M6 12H18M6 12L11 7M6 12L11 17" stroke="#16B1BB" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
    }
}
