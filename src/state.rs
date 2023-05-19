use std::{collections::BTreeMap, vec};

use wasm_bindgen::JsValue;

#[derive(Clone, PartialEq)]
pub struct State {
    pub dates: BTreeMap<String, (Vec<IncomeFormEntry>, Vec<ExpenseFormEntry>)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            dates: BTreeMap::new(),
        }
    }

    pub fn dates(&self) -> Vec<Date> {
        let mut dates = Vec::new();
        for (date, (income_entries, expense_entries)) in &self.dates {
            dates.push(Date {
                date: date.clone(),
                income_entries: income_entries.clone(),
                expense_entries: expense_entries.clone(),
            })
        }
        dates
    }

    pub fn submit_income_form(&mut self, entry: IncomeFormEntry) {
        let msg = format!("income form entry: {entry:?}");
        gloo_console::log!(JsValue::from(msg));
        if let Some((income_entries, _)) = self.dates.get_mut(&entry.date) {
            income_entries.push(entry)
        } else {
            self.dates.insert(entry.date.clone(), (vec![entry], vec![]));
        };
    }

    pub fn submit_expense_form(&mut self, entry: ExpenseFormEntry) {
        let msg = format!("expense form entry: {entry:?}");
        gloo_console::log!(JsValue::from(msg));
        if let Some((_, expense_entries)) = self.dates.get_mut(&entry.date) {
            expense_entries.push(entry)
        } else {
            self.dates.insert(entry.date.clone(), (vec![], vec![entry]));
        };
    }
}

#[derive(PartialEq, Clone)]
pub struct Date {
    pub date: String,
    pub income_entries: Vec<IncomeFormEntry>,
    pub expense_entries: Vec<ExpenseFormEntry>,
}

impl Date {
    pub fn total_income(&self) -> u32 {
        self.income_entries.iter().map(|a| a.value).sum()
    }
    pub fn total_expenses(&self) -> u32 {
        self.expense_entries.iter().map(|a| a.value).sum()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IncomeFormEntry {
    pub value: u32,
    pub date: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpenseFormEntry {
    pub value: u32,
    pub date: String,
}
