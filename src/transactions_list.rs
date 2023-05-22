use crate::app_state::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TransactionsListProps {
    pub data: TransactionsListData,
    pub delete_transaction: Callback<TransactionId>,
    pub title: String,
}

#[function_component(TransactionsList)]
pub fn transactions_list(props: &TransactionsListProps) -> Html {
    html! {
    <section>
        <h3>{props.title.clone()}</h3>
        <ol>
            {for props.data.transactions.iter().map(|tr| html!{
                <TransactionsListItem
                    value={tr.value}
                    kind={tr.kind}
                    date={tr.date}
                    id={tr.id}
                    delete_transaction={props.delete_transaction.clone()}
                />
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
