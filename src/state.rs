use yew::Html;

pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn dates(&self) -> Vec<Date> {
        vec![Date{}, Date{}, Date{}]
    }

}

#[derive(PartialEq)]
pub struct Date {}

