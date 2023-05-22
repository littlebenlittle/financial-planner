use std::rc::Rc;
use yew::Reducible;

// Primitives
pub type Date = chrono::NaiveDate;
pub type Dollars = u32;
pub type TransactionId = ();

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransactionKind {
    Income,
    Expense,
}

impl std::fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TransactionKind::*;
        match self {
            Income => write!(f, "Income"),
            Expense => write!(f, "Expense"),
        }
    }
}

// Compound types
#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    pub value: Dollars,
    pub kind: TransactionKind,
    pub date: Date,
    pub id: TransactionId,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DateSummary {
    pub date: Date,
    pub income: Dollars,
    pub expenses: Dollars,
    pub balance: Dollars,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TimelineData(Vec<DateSummary>);

impl TimelineData {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a DateSummary> {
        self.0.iter()
    }
}

impl Default for TimelineData {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionsListData {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    ReportIncome(Date, Dollars),
    ReportExpense(Date, Dollars),
    DeleteTransaction(TransactionId),
    SetDateRange { from: Date, to: Date },
}

#[derive(Debug, PartialEq, Clone)]
pub struct State(Vec<Action>);

impl Default for State {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Reducible for State {
    type Action = Action;
    fn reduce(self: Rc<Self>, event: Action) -> Rc<Self> {
        let mut inner = self.0.clone();
        inner.push(event);
        Self(inner).into()
    }
}

impl State {
    pub fn timeline_data(&self) -> TimelineData {
        unimplemented!()
    }
    pub fn transactions_list_data(&self) -> TransactionsListData {
        unimplemented!()
    }
}
