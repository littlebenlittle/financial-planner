use std::collections::BTreeMap;

use crate::components::*;
use crate::state::*;
use chrono::Duration;
use yew::prelude::*;

use TransactionKind::{Expense, Income};

#[function_component(App)]
pub fn app() -> Html {
    let transactions_handle = use_state_eq(Transactions::example);
    let timeline_start_handle = use_state_eq(|| chrono::offset::Local::now().date_naive());
    let timeline_end_handle = use_state_eq(|| chrono::offset::Local::now().date_naive());
    let start = timeline_start_handle.clone();
    let end = timeline_end_handle.clone();
    let transactions = (*transactions_handle).clone();
    html! {
        <main>
            <Timeline
                title={"Timeline"}
                canvas_id={"my_canvas"}
                data={transactions.timeline_data(*start, *end)}
                set_start_date={
                    let start = timeline_start_handle.clone();
                    move |date: Date| {
                        start.set(date);
                    }
                }
                set_end_date={
                    let end = timeline_end_handle.clone();
                    move |date: Date| {
                        end.set(date);
                    }
                }

            />
            <DateSummariesList
                title={"Date Summaries"}
                dates={transactions.date_summaries()}
            />
            <TransactionsList
                title={"Transactions List"}
                transactions={transactions.transactions()}
                delete_transaction={
                    let transactions = transactions_handle.clone();
                    move |id: TransactionId| {
                        let mut new_transactions = (*transactions).clone();
                        new_transactions.delete(id);
                        transactions.set(new_transactions);
                    }
                }
            />
            <TransactionForm
                title={"Income Form"}
                submit={
                    let transactions = transactions_handle.clone();
                    move |(date, value): (Date, Dollars)| {
                        let mut new_transactions = (*transactions).clone();
                        new_transactions.insert(Transaction { kind: Income, value, date });
                        transactions.set(new_transactions);
                    }
                }
            />
            <TransactionForm
                title={"Expense Form"}
                submit={
                    let transactions = transactions_handle.clone();
                    move |(date, value): (Date, Dollars)| {
                        let mut new_transactions = (*transactions).clone();
                        new_transactions.insert(Transaction { kind: Expense, value, date });
                        transactions.set(new_transactions);
                    }
                }
            />
        </main>
    }
}
