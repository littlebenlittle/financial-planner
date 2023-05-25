use crate::app_state::*;
use crate::components::*;
use yew::prelude::*;

use TransactionKind::{Expense, Income};

#[function_component(App)]
pub fn app() -> Html {
    let log_handle = use_reducer(Log::default);
    let counter_handle = use_state(|| TransactionId::default());
    let log = (*log_handle).clone();

    let set_date_range = {
        let log = log_handle.clone();
        move |(from, to)| log.dispatch(Entry::SetDate((from, to).into()))
    };

    let delete_transaction = {
        let log = log_handle.clone();
        move |id| log.dispatch(Entry::Delete(id))
    };

    let report_income = {
        let log = log_handle.clone();
        let counter = counter_handle.clone();
        move |(date, value)| {
            log.dispatch(Entry::Create(Transaction {
                value,
                kind: Income,
                date,
            }));
            counter.set(*counter + 1);
        }
    };

    let report_expense = {
        let log = log_handle.clone();
        let counter = counter_handle.clone();
        move |(date, value)| {
            log.dispatch(Entry::Create(Transaction {
                value,
                kind: Expense,
                date,
            }));
            counter.set(*counter + 1);
        }
    };

    html! {
        <main>
            <p><b>{concat!{
                "This app is for demonstration purposes only. It is not intended to secure ",
                "private information. Any information entered into this app should be ",
                "considered effectively public information.",
            }}</b></p>
            <Timeline
                title={"Timeline"}
                canvas_id={"my_canvas"}
                data={log.timeline_data()}
                set_date_range={set_date_range}
            />
            <TransactionsList
                title={"Transactions List"}
                data={log.transaction_records()}
                delete_transaction={delete_transaction}
            />
            <TransactionForm
                title={"Income Form"}
                submit={report_income}
            />
            <TransactionForm
                title={"Expense Form"}
                submit={report_expense}
            />
        </main>
    }
}
