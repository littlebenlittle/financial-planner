use std::collections::BTreeMap;

use crate::app_state::*;
use crate::components::*;
use chrono::Duration;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state_handle = use_reducer(State::default);
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
        move |(date, dollars)| state.dispatch(Action::ReportIncome(date, dollars))
    };

    let report_expense = {
        let state = state_handle.clone();
        move |(date, dollars)| state.dispatch(Action::ReportExpense(date, dollars))
    };

    html! {
        <main>
            <Timeline
                title={"Timeline"}
                canvas_id={"my_canvas"}
                data={state.timeline_data()}
                set_date_range={set_date_range}
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
