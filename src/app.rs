use crate::state::*;
use crate::components::*;
use yew::prelude::*;

use TransactionKind::{Expense, Income};

#[function_component(App)]
pub fn app() -> Html {
    let state_handle = use_state_eq(State::example);
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
