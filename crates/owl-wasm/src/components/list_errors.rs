use yew::prelude::*;

use crate::error::Error;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub error: Option<Error>,
}

#[function_component(ListErrors)]
pub fn list_errors(props: &Props) -> Html {
    if let Some(error) = &props.error {
        html! {
            <div class="flex items-center justify-center">
                <ul class="text-red-600 text-medium text-4md">
                    {
                        match error {
                            Error::UnprocessableEntity(error_info) => {
                                html! {
                                    <>
                                    {for error_info.errors.iter().map(|(key, value)| {
                                        html! {
                                            <li>
                                            { key }
                                            {for value.iter().map(|e| {
                                                html! {
                                                    <>{" "} {e}</>
                                                }
                                            })}
                                            </li>
                                        }
                                    })}
                                    </>
                                }
                            }
                            _ => {
                                html! {
                                    <li>{error.to_string()}</li>
                                }
                            }

                        }
                    }
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}
