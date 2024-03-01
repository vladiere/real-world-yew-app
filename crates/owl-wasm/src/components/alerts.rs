use yew::prelude::*;

use crate::error::Error;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub alert_msg: Option<String>,
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct DangerProps {
    pub error: Option<Error>,
}
#[function_component(SuccessAlert)]
pub fn success_alert(props: &Props) -> Html {
    if let Some(message) = &props.alert_msg {
        if !message.is_empty() {
            html! {
                <div class="absolute top-2 right-2 inline-flex w-1/3 items-center rounded-lg bg-green-300 px-6 py-5 text-base text-green-700" role="alert">
                    <span class="mr-2">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-5 w-5">
                            <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd" />
                        </svg>
                    </span>
                    { message }
                </div>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}

#[function_component(DangerAlert)]
pub fn danger_alert(props: &DangerProps) -> Html {
    if let Some(error) = &props.error {
        html! {
            <div class="absolute top-2 right-2 inline-flex w-1/3 items-center rounded-lg bg-red-300 px-6 py-5 text-base text-red-700" role="alert">
                <span class="mr-2">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-5 w-5">
                        <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-1.72 6.97a.75.75 0 10-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 101.06 1.06L12 13.06l1.72 1.72a.75.75 0 101.06-1.06L13.06 12l1.72-1.72a.75.75 0 10-1.06-1.06L12 10.94l-1.72-1.72z" clip-rule="evenodd" />
                    </svg>
                </span>
                {
                    match error {
                        Error::UnprocessableEntity(error_info) => {
                            html! {
                                <>
                                    {for error_info.errors.iter().map(|(key, value)| {
                                        html! {
                                            <span>
                                                { key }
                                                {for value.iter().map(|e| {
                                                    html! {
                                                        <>{" "} {e}</>
                                                    }
                                                })}
                                            </span>
                                        }
                                    })}
                                </>
                            }
                        },
                        _ => {
                            html! {
                                <span>{ error.to_string() }</span>
                            }
                        }
                    }
                }
            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(WarningAlert)]
pub fn warning_alert(props: &Props) -> Html {
    if let Some(message) = &props.alert_msg {
        html! {
            <div class="absolute top-2 right-2 inline-flex w-1/3 items-center rounded-lg bg-yellow-300 px-6 py-5 text-base text-yellow-800" role="alert">
                <span class="mr-2">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-5 w-5">
                        <path fill-rule="evenodd" d="M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003zM12 8.25a.75.75 0 01.75.75v3.75a.75.75 0 01-1.5 0V9a.75.75 0 01.75-.75zm0 8.25a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd" />
                    </svg>
                </span>
                { message }
            </div>
        }
    } else {
        html! {}
    }
}
