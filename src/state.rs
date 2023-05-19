use std::{collections::BTreeMap, vec};

use wasm_bindgen::JsValue;

#[derive(Clone, PartialEq)]
pub struct State {
    pub dates: BTreeMap<String, (Vec<IncomeEntry>, Vec<ExpenseEntry>)>,
    pub income_entries: IncomeEntries,
    pub expense_entries: ExpenseEntries,
}

impl State {
    pub fn new() -> Self {
        Self {
            dates: BTreeMap::new(),
            income_entries: Default::default(),
            expense_entries: Default::default(),
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

    //TODO unify submission logic

    pub fn submit_income_form(&mut self, entry: IncomeEntry) {
        gloo_console::log!(JsValue::from(format!("income form entry: {entry:?}")));
        self.income_entries.push(entry.clone());
        if let Some((income_entries, _)) = self.dates.get_mut(&entry.date) {
            income_entries.push(entry);
        } else {
            self.dates.insert(entry.date.clone(), (vec![entry], vec![]));
        };
    }

    pub fn submit_expense_form(&mut self, entry: ExpenseEntry) {
        gloo_console::log!(JsValue::from(format!("expense form entry: {entry:?}")));
        self.expense_entries.push(entry.clone());
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
    pub income_entries: Vec<IncomeEntry>,
    pub expense_entries: Vec<ExpenseEntry>,
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
pub struct IncomeEntry {
    pub value: u32,
    pub date: String,
}

pub type IncomeEntries = Vec<IncomeEntry>;

#[derive(Debug, PartialEq, Clone)]
pub struct ExpenseEntry {
    pub value: u32,
    pub date: String,
}

pub type ExpenseEntries = Vec<ExpenseEntry>;
