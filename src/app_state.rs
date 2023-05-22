use yew::Reducible;
use std::rc::Rc;

// Primitives
pub type Date = ();
pub type Dollars = ();
pub type TransactionId = ();

// Compound types
pub type Transaction = ();

// Component data
pub type TimelineData = ();
pub type TransactionsListData = ();

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    ReportIncome(Date, Dollars),
    ReportExpense(Date, Dollars),
    DeleteTransaction(TransactionId),
    SetDateRange{
        from: Date,
        to: Date
    },
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
    fn reduce(mut self: Rc<Self>, event: Action) -> Rc<Self> {
        self.0.push(event);
        self
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
