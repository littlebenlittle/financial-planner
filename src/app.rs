use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            // First I name the agents involved
            <TimelineMenu />
            <BalanceTimeline />
            <IncomeForm />
            <ExpenseForm />
        </main>
    }
}
