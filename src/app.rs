use crate::app_state::*;
use crate::components::*;
use yew::prelude::*;

use TransactionKind::{Income, Expense};

#[function_component(App)]
pub fn app() -> Html {
    let state_handle = use_reducer(State::default);
    let counter_handle = use_state(|| TransactionId::default());
    let state = (*state_handle).clone();

    let set_date_range = {
        let state = state_handle.clone();
        move |(from, to)| state.dispatch(Action::SetDateRange { from, to })
    };

    let delete_transaction = {
        let state = state_handle.clone();
        move |id| state.dispatch(Action::DeleteTransaction(id))
    };

    let report_income = {
        let state = state_handle.clone();
        let counter = counter_handle.clone();
        move |(date, value)| {
            state.dispatch(Action::CreateTransaction(Transaction {
                value,
                kind: Income,
                date,
                id: (*counter),
            }));
            counter.set(*counter + 1);
        }
    };

    let report_expense = {
        let state = state_handle.clone();
        let counter = counter_handle.clone();
        move |(date, value)| {
            state.dispatch(Action::CreateTransaction(Transaction {
                value,
                kind: Expense,
                date,
                id: (*counter),
            }));
            counter.set(*counter + 1);
        }
    };

    html! {
        <main>
            <Timeline
                title={"Timeline"}
                canvas_id={"my_canvas"}
                data={state.timeline_data()}
                set_date_range={set_date_range}
                histogram={false}
            />
            <TransactionsList
                title={"Transactions List"}
                data={state.transactions_list_data()}
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
