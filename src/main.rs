mod app;
mod timeline;
mod transactions_form;
mod transactions_list;
mod app_state;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

mod components {
    pub use super::timeline::Timeline;
    pub use super::transactions_form::TransactionForm;
    pub use super::transactions_list::TransactionsList;
}
