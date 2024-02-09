use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    pub img_class: String,
}

#[function_component(OwlLogo)]
pub fn owl_logo(props: &LogoProps) -> Html {
    html! {
        <img src="public/images/owl-logo.png" class={props.img_class.clone()} alt="OWL Logo" />
    }
}
