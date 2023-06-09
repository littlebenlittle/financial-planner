#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod app;
mod app_state;
mod timeline;
mod transactions_form;
mod transactions_list;
mod debug_window;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

mod components {
    pub use super::timeline::Timeline;
    pub use super::transactions_form::TransactionForm;
    pub use super::transactions_list::TransactionsList;
    pub use super::debug_window::DebugWindow;
}
