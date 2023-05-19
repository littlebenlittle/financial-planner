use wasm_bindgen::JsValue;

#[derive(Clone, PartialEq)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn dates(&self) -> Vec<Date> {
        vec![Date{}, Date{}, Date{}]
    }
    
    pub fn submit_income_form(&mut self, entry: IncomeFormEntry) {
        let msg = format!("income form entry: {entry:?}");
        gloo_console::log!(JsValue::from(msg))
    }

}

#[derive(PartialEq)]
pub struct Date {}

#[derive(Debug)]
pub struct IncomeFormEntry {
    pub value: String,
    pub date: String
}
