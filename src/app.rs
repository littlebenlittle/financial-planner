
use yew::prelude::*;
use crate::{State, Date};

#[function_component(App)]
pub fn app() -> Html {
    let state = State::new();
    html! {
        <main>
            <Timeline dates={state.dates()} />
            <IncomeForm />
            <ExpenseForm />
        </main>
    }
}

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    dates: Vec<Date>
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    fn date_to_html(date: &Date) -> Html {
        html!{
            <li>
                <p>{"Date"}</p>
            </li>
        }
    }
    html! {
        <section>
            <h3>{"Timeline"}</h3>
            <ol>
                { for props.dates.iter().map(date_to_html) }
            </ol>
        </section>
    }
}

#[function_component(IncomeForm)]
pub fn income_form() -> Html {
    html! {
        <section>
            <p>{"Income Form"}</p>
        </section>
    }
}

#[function_component(ExpenseForm)]
pub fn expense_form() -> Html {
    html! {
        <section>
            <p>{"Expense Form"}</p>
        </section>
    }
}
