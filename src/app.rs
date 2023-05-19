use crate::state::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state_handle = use_state_eq(State::new);
    let state = (*state_handle).clone();
    let submit_income = {
        let state = state_handle.clone();
        Callback::from(move |entry: IncomeFormEntry| {
            let mut new_state = (*state).clone();
            new_state.submit_income_form(entry);
            state.set(new_state)
        })
    };
    html! {
        <main>
            <Timeline dates={state.dates()} />
            <IncomeForm submit={submit_income}/>
            <ExpenseForm />
        </main>
    }
}

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    dates: Vec<Date>,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    fn date_to_html(date: &Date) -> Html {
        html! {
            <li>
                <p>{"Date"}</p>
            </li>
        }
    }
    html! {
        <section>
            <h3>{"Timeline"}</h3>
            <ol>
                { for props.dates.iter().map(date_to_html) }
            </ol>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct IncomeFormProps {
    pub submit: Callback<IncomeFormEntry>,
}

#[function_component(IncomeForm)]
pub fn income_form(props: &IncomeFormProps) -> Html {
    let value_handle = use_state(String::default);
    let date_handle = use_state(String::default);
    let value = (*value_handle).clone();
    let date = (*date_handle).clone();
    html! {
        <section>
            <p>{"Income Form"}</p>
            <input onchange={
                let value_handle = value_handle.clone();
                move |e: Event| {
                    if let Some(input) = e
                        .target()
                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    {
                        value_handle.set(input.value());
                    }
                }
            }
                type="text"
                value={value}
            />
            <input onchange={
                let date_handle = date_handle.clone();
                move |e: Event| {
                    if let Some(input) = e
                        .target()
                        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                    {
                        date_handle.set(input.value());
                    }
                }
            }
                type="date"
                value={date}
            />
            <button onclick={
                let value_handle = value_handle.clone();
                let date_handle = date_handle.clone();
                let submit_callback = props.submit.clone();
                move |_| {
                    submit_callback.emit(IncomeFormEntry {
                        value: (*value_handle).clone(),
                        date: (*date_handle).clone()
                    })
                }
            }>{"Submit"}</button>
        </section>
    }
}

#[function_component(ExpenseForm)]
pub fn expense_form() -> Html {
    html! {
        <section>
            <p>{"Expense Form"}</p>
        </section>
    }
}
