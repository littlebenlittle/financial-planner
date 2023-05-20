mod app;
mod date_summaries_list;
mod state;
mod timeline;
mod transactions_form;
mod transactions_list;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

mod components {
    pub use super::date_summaries_list::DateSummariesList;
    pub use super::timeline::Timeline;
    pub use super::transactions_form::TransactionForm;
    pub use super::transactions_list::TransactionsList;
}
