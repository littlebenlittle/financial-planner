use crate::app_state::*;
use crate::components::*;
use chrono::Duration;
use yew::prelude::*;

use TransactionKind::{Expense, Income};

fn today() -> Date {
    chrono::Local::now().date_naive()
}

#[function_component(App)]
pub fn app() -> Html {
    let log = use_reducer(|| {
        let mut log = Log::default();
        log.append(Entry::SetDate(
            (today(), today() + Duration::days(30)).into(),
        ));
        log
    });
    let start_date = use_state(|| today());
    let end_date = use_state(|| (today() + Duration::days(30)));

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
        <main>
            <p><b>{concat!{
                "This app is for demonstration purposes only. It is not intended to secure ",
                "private information. Any information entered into this app should be ",
                "considered effectively public information.",
            }}</b></p>
            <DebugWindow log={log.entries()} />
            <TransactionsList
                title={"Transactions List"}
                data={log.transaction_records()}
                {delete_transaction}
            />
            <TransactionForm
                title={"Income Form"}
                submit={report_income}
            />
            <TransactionForm
                title={"Expense Form"}
                submit={report_expense}
            />
            <Timeline
                title={"Timeline"}
                canvas_id={"my_canvas"}
                data={log.timeline_data()}
                start_date={(*start_date).to_string()}
                end_date={(*end_date).to_string()}
                {set_start_date}
                {set_end_date}
            />
        </main>
    }
}
