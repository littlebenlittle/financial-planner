use std::{collections::BTreeMap, error::Error};

use crate::state::*;
use chrono::NaiveDate;
use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::ops::Sub;
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
            <Timeline
                title={"Timeline"}
                canvas_id={"my_canvas"}
                dates={state.date_summaries()}
            />
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
pub struct DateSummariesListProps {
    pub dates: DateSummaries,
    pub title: String,
}

#[function_component(DateSummariesList)]
pub fn date_summaries_list(props: &DateSummariesListProps) -> Html {
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
        move |_| {
            let date = match date.parse::<Date>() {
                Ok(date) => Some(date),
                Err(e) => {
                    gloo_console::log!(format!("{e:?}"));
                    None
                }
            };
            let value = match value.parse::<u32>() {
                Ok(value) => Some(value),
                Err(e) => {
                    gloo_console::log!(format!("{e:?}"));
                    None
                }
            };
            if date.is_some() && value.is_some() {
                submit.emit((date.unwrap(), value.unwrap()));
            }
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

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub dates: DateSummaries,
    pub title: String,
    pub canvas_id: String,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    let canvas_id = props.canvas_id.clone();
    let dates = props.dates.clone();
    let start_date_handle = use_state(String::new);
    let on_start_date_change = {
        let start_date_handle = start_date_handle.clone();
        move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                start_date_handle.set(input.value());
            }
        }
    };
    use_effect({
        let start_date = (*start_date_handle).clone();
        move || {
            match start_date.parse() {
                Err(e) => gloo_console::log!(format!("{e:?}")),
                Ok(start_date) => match draw_timeline(&canvas_id, dates, start_date) {
                    Err(e) => gloo_console::log!(format!("{e:?}")),
                    _ => {}
                },
            }
            || {}
        }
    });
    let start_date = (*start_date_handle).clone();
    html! {
    <section>
        <h3>{props.title.clone()}</h3>
        <canvas
            id={props.canvas_id.clone()}
            style={"width: 80%; height: auto;"}
        />
        <p>{"Start Date: "}</p>
        <input onchange={on_start_date_change}
            type="date"
            value={start_date}
        />
    </section>
    }
}

fn draw_timeline(
    canvas_id: &str,
    dates: DateSummaries,
    start_date: Date,
) -> Result<(), Box<dyn Error>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        //.caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Dollars")
        .x_desc("Date")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let income_data = dates
        .iter()
        .map(|(d, s)| (d.sub(start_date).num_days() as u32, s.income))
        .collect_vec();
    gloo_console::log!(format!("{income_data:?}"));

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(income_data),
    )?;

    root.present()?;
    Ok(())
}
