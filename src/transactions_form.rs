use crate::app_state::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TransactionFormProps {
    pub title: String,
    pub submit: Callback<(Date, Dollars)>,
}

#[function_component(TransactionForm)]
pub fn transactions_form(props: &TransactionFormProps) -> Html {
    let value_handle = use_state(String::default);
    let date_handle = use_state(String::default);
    let value = (*value_handle).clone();
    let date = (*date_handle).clone();

    let submit = {
        let value_handle = value_handle.clone();
        let date_handle = date_handle.clone();
        let submit = props.submit.clone();
        let value = (*value_handle).clone();
        let date = (*date_handle).clone();
        move |_| {
            let date = match date.parse::<Date>() {
                Ok(date) => Some(date),
                Err(e) => {
                    gloo_console::log!(format!("{e:?}"));
                    None
                }
            };
            match value.parse::<u16>() {
                Ok(value) => {
                    if date.is_some() {
                        submit.emit((date.unwrap(), value as Dollars));
                    }
                }
                Err(e) => {
                    gloo_console::log!(format!("{e:?}"));
                }
            };
        }
    };

    let on_value_change = {
        let value_handle = value_handle.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                value_handle.set(input.value());
            }
        }
    };

    let on_date_change = {
        let date_handle = date_handle.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                date_handle.set(input.value());
            }
        }
    };

    html! {
        <section class={classes!("w3-container", "w3-content")}>
            <h3>{props.title.clone()}</h3>
            <input onchange={on_value_change}
                type="text"
                value={value}
            />
            <input onchange={on_date_change}
                type="date"
                value={date}
            />
            <button onclick={submit}>{"Submit"}</button>
        </section>
    }
}
