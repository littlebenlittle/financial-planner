use crate::app_state::{Date, Dollars};
use yew::prelude::*;

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
pub fn date_summary_entry(props: &DateSummaryEntryProps) -> Html {
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
