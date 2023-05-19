mod app;
mod state;

use app::App;
use state::{State, Date};

fn main() {
    yew::Renderer::<App>::new().render();
}
