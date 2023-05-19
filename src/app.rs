use crate::state::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state_handle = use_state_eq(State::new);
    let state = (*state_handle).clone();
    html! {
        <main>
            <Timeline dates={state.dates()} />
            <IncomeForm submit={
                let state = state_handle.clone();
                move |entry: IncomeFormEntry| {
                    let mut new_state = (*state).clone();
                    new_state.submit_income_form(entry);
                    state.set(new_state)
                }
            }/>
            <ExpenseForm submit={
                let state = state_handle.clone();
                move |entry: ExpenseFormEntry| {
                    let mut new_state = (*state).clone();
                    new_state.submit_expense_form(entry);
                    state.set(new_state)
                }
            }/>
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
                <p>{date.date.clone()}</p>
                <p>{"Income: "}{date.total_income()}</p>
                <p>{"Expenses: "}{date.total_expenses()}</p>
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

    let submit = {
        let value_handle = value_handle.clone();
        let date_handle = date_handle.clone();
        let submit_callback = props.submit.clone();
        let value = (*value_handle).clone();
        move |_| match value.parse::<u32>() {
            Ok(n) => {
                submit_callback.emit(IncomeFormEntry {
                    value: n,
                    date: (*date_handle).clone(),
                });
            }
            Err(e) => gloo_console::log!("{e:?}"),
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
        <section>
            <p>{"Income Form"}</p>
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

#[derive(Properties, PartialEq)]
pub struct ExpenseFormProps {
    pub submit: Callback<ExpenseFormEntry>,
}

#[function_component(ExpenseForm)]
pub fn expense_form(props: &ExpenseFormProps) -> Html {
    let value_handle = use_state(String::default);
    let date_handle = use_state(String::default);
    let value = (*value_handle).clone();
    let date = (*date_handle).clone();

    let submit = {
        let value_handle = value_handle.clone();
        let date_handle = date_handle.clone();
        let submit_callback = props.submit.clone();
        let value = (*value_handle).clone();
        move |_| match value.parse::<u32>() {
            Ok(n) => {
                submit_callback.emit(ExpenseFormEntry {
                    value: n,
                    date: (*date_handle).clone(),
                });
            }
            Err(e) => gloo_console::log!("{e:?}"),
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
        <section>
            <p>{"Expense Form"}</p>
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
