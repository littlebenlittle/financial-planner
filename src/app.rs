use crate::app_state::*;
use crate::components::*;
use chrono::Duration;
use yew::prelude::*;

use TransactionKind::{Expense, Income};

fn today_plus(days: i64) -> Date {
    chrono::Local::now().date_naive() + Duration::days(days)
}

#[function_component(App)]
pub fn app() -> Html {
    let log = use_reducer(|| {
        use Entry::{Create, SetDate};
        Log::from(vec![
            SetDate((today_plus(0), today_plus(30)).into()),
            Create(Transaction {
                value: 100,
                kind: Income,
                date: today_plus(1),
            }),
            Create(Transaction {
                value: 100,
                kind: Income,
                date: today_plus(2),
            }),
            Create(Transaction {
                value: 100,
                kind: Expense,
                date: today_plus(3),
            }),
            Create(Transaction {
                value: 500,
                kind: Expense,
                date: today_plus(4),
            }),
        ])
    });
    let start_date = use_state(|| today_plus(0));
    let end_date = use_state(|| today_plus(30));

    let set_start_date = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let log = log.clone();
        move |date_string: String| match date_string.parse::<Date>() {
            Ok(date) => {
                start_date.set(date);
                log.dispatch(Entry::SetDate((date, *end_date).into()));
            }
            Err(e) => gloo_console::log!(format!("start date: {e:?}")),
        }
    };

    let set_end_date = {
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let log = log.clone();
        move |date_string: String| match date_string.parse::<Date>() {
            Ok(date) => {
                end_date.set(date);
                log.dispatch(Entry::SetDate((*start_date, date).into()));
            }
            Err(e) => gloo_console::log!(format!("end date: {e:?}")),
        }
    };

    let delete_transaction = {
        let log = log.clone();
        move |id| log.dispatch(Entry::Delete(id))
    };

    let report_income = {
        let log = log.clone();
        move |(date, value)| {
            log.dispatch(Entry::Create(Transaction {
                value,
                kind: Income,
                date,
            }));
        }
    };

    let report_expense = {
        let log = log.clone();
        move |(date, value)| {
            log.dispatch(Entry::Create(Transaction {
                value,
                kind: Expense,
                date,
            }));
        }
    };

    html! {
    <main
        class={classes!("w3-container", "w3-content")}
        style={"max-width: 1200px;"}
    >
        <div class={classes!("w3-container", "w3-panel", "w3-orange")}>
            <p><b>{concat!{
                "This app is for demonstration purposes only. It is not intended to secure ",
                "private information. Any information entered into this app should be ",
                "considered effectively public information.",
            }}</b></p>
        </div>
        <div class={classes!("w3-row")}>
            <div class={classes!("w3-col", "l9", "m9", "s9")}>
                <MainAppArea
                    transaction_records={log.transaction_records()}
                    {delete_transaction}
                    {report_income}
                    {report_expense}
                    start_date={*start_date}
                    end_date={*end_date}
                    timeline_data={log.timeline_data()}
                    {set_start_date}
                    {set_end_date}
                />
            </div>
            <div
                id={"debug-window"}
                class={classes!("w3-col", "l3", "m3", "s3")}
            >
                <DebugWindow log={log.entries()} />
            </div>
        </div>
    </main>
    }
}

#[derive(Properties, PartialEq)]
struct MainAppAreaProps {
    transaction_records: Vec<TransactionRecord>,
    delete_transaction: Callback<TransactionId>,
    report_income: Callback<(Date, Dollars)>,
    report_expense: Callback<(Date, Dollars)>,
    timeline_data: TimelineData,
    start_date: Date,
    end_date: Date,
    set_start_date: Callback<String>,
    set_end_date: Callback<String>,
}

#[function_component(MainAppArea)]
fn main_app_area(props: &MainAppAreaProps) -> Html {
    html! {
    <>
    <TransactionsList
        title={"Transactions List"}
        data={props.transaction_records.clone()}
        delete_transaction={props.delete_transaction.clone()}
    />
    <div class={classes!("w3-container", "w3-content")}>
        <h3>{"Transaction Forms"}</h3>
        <TransactionForm
            title={"Income Form"}
            submit={props.report_income.clone()}
        />
        <TransactionForm
            title={"Expense Form"}
            submit={props.report_expense.clone()}
        />
    </div>
    <Timeline
        title={"Timeline"}
        canvas_id={"my_canvas"}
        data={props.timeline_data.clone()}
        start_date={props.start_date.to_string()}
        end_date={props.end_date.to_string()}
        set_start_date={props.set_start_date.clone()}
        set_end_date={props.set_end_date.clone()}
    />
    </>
    }
}
