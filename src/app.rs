use std::collections::BTreeMap;

use crate::state::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use TransactionKind::{Expense, Income};

#[function_component(App)]
pub fn app() -> Html {
    let state_handle = use_state_eq(State::new);
    let state = (*state_handle).clone();
    html! {
        <main>
            <DateSummariesList
                title={"Date Summaries"}
                dates={state.date_summaries()}
            />
            <TransactionsList
                title={"Transactions List"}
                transactions={state.transactions}
                delete_transaction={
                    let state = state_handle.clone();
                    move |id: TransactionId| {
                        let mut new_state = (*state).clone();
                        new_state.delete(id);
                        state.set(new_state);
                    }
                }
            />
            <TransactionForm
                title={"Income Form"}
                submit={
                    let state = state_handle.clone();
                    move |(date, value): (Date, Dollars)| {
                        let mut new_state = (*state).clone();
                        new_state.insert(Transaction { kind: Income, value, date });
                        state.set(new_state);
                    }
                }
            />
            <TransactionForm
                title={"Expense Form"}
                submit={
                    let state = state_handle.clone();
                    move |(date, value): (Date, Dollars)| {
                        let mut new_state = (*state).clone();
                        new_state.insert(Transaction { kind: Expense, value, date });
                        state.set(new_state);
                    }
                }
            />
        </main>
    }
}

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub dates: DateSummaries,
    pub title: String,
}

#[function_component(DateSummariesList)]
pub fn date_summaries_list(props: &TimelineProps) -> Html {
    html! {
        <section>
            <h3>{props.title.clone()}</h3>
            <ol>
                { for props.dates.iter().map(|(date, summary)| {
                    html!{
                        <DateSummaryEntry
                            date={date.to_owned()}
                            income={summary.income}
                            expenses={summary.expenses}
                        />
                    }
                }) }
            </ol>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct DateSummaryEntryProps {
    pub date: Date,
    pub income: Dollars,
    pub expenses: Dollars,
}

#[function_component(DateSummaryEntry)]
fn date_summary_entry(props: &DateSummaryEntryProps) -> Html {
    let date = props.date.clone();
    let income = props.income.clone();
    let expenses = props.expenses.clone();
    html! {
        <li>
            <p>{date}</p>
            <p>{"Income: "}{income}</p>
            <p>{"Expenses: "}{expenses}</p>
        </li>
    }
}

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
        move |_| match value.parse::<u32>() {
            Ok(value) => submit.emit((date.clone(), value)),
            Err(e) => gloo_console::log!(format!("{e:?}")),
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

#[derive(Properties, PartialEq)]
pub struct TransactionsListProps {
    transactions: BTreeMap<TransactionId, Transaction>,
    delete_transaction: Callback<TransactionId>,
    title: String,
}

#[function_component(TransactionsList)]
pub fn transactions_list(props: &TransactionsListProps) -> Html {
    html! {
    <section>
        <h3>{props.title.clone()}</h3>
        <ol>
            {for props.transactions.iter().map(|(id, tr): (&TransactionId, &Transaction)| {
                html!{
                    <TransactionsListItem
                        value={tr.value}
                        kind={tr.kind.clone()}
                        date={tr.date.clone()}
                        id={id}
                        delete_transaction={props.delete_transaction.clone()}
                    />
                }
            })}
        </ol>
    </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct TransactionsListItemProps {
    value: Dollars,
    kind: TransactionKind,
    date: Date,
    id: TransactionId,
    delete_transaction: Callback<TransactionId>,
}

#[function_component(TransactionsListItem)]
fn transactions_list_item(props: &TransactionsListItemProps) -> Html {
    let delete_transaction = props.delete_transaction.clone();
    let kind = props.kind.clone();
    let date = props.date.clone();
    let id = props.id.clone();
    html! {
        <>
        <p>{"Kind: "}{kind}</p>
        <p>{"Date: "}{date}</p>
        <p>{"Value: "}{props.value}</p>
        <button onclick={move |_| delete_transaction.emit(id)}>
            {"Delete"}
        </button>
        </>
    }
}
