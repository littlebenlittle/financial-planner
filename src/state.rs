use chrono::{Duration, NaiveDate};
use itertools::Itertools;
use std::{collections::BTreeMap, error::Error};

pub type TransactionId = u32;
pub type Dollars = u32;
pub type Income = Dollars;
pub type Expenses = Dollars;
pub type Balance = Dollars;
pub type TimelineData = Vec<(Income, Expenses, Balance)>;
pub type Date = chrono::NaiveDate;
pub type DateSummaries = BTreeMap<Date, DateSummary>;
pub type TransactionRecord = (TransactionId, Transaction);

#[derive(Clone, PartialEq)]
pub struct Transactions {
    transactions: BTreeMap<TransactionId, Transaction>,
    last_id: TransactionId,
}

impl Transactions {
    pub fn new() -> Self {
        Self {
            transactions: Default::default(),
            last_id: 0,
        }
    }

    pub fn example() -> Self {
        use TransactionKind::*;
        let mut this = Self::new();
        this.insert((Income, 150, "2023-05-05".parse().unwrap()));
        this.insert((Income, 200, "2023-05-09".parse().unwrap()));
        this.insert((Expense, 300, "2023-05-15".parse().unwrap()));
        this
    }

    pub fn date_summaries(&self) -> DateSummaries {
        let dates = self
            .transactions
            .values()
            .map(|tr| tr.date.clone())
            .unique();
        let mut result = BTreeMap::new();
        for date in dates {
            let transactions: Vec<Transaction> = self
                .transactions
                .values()
                .filter(|e| e.date == date)
                .map(|tr| tr.clone())
                .collect_vec();
            result.insert(
                date,
                DateSummary {
                    income: transactions
                        .iter()
                        .filter(|e| e.kind == TransactionKind::Income)
                        .map(|e| e.value)
                        .sum(),
                    expenses: transactions
                        .iter()
                        .filter(|e| e.kind == TransactionKind::Expense)
                        .map(|e| e.value)
                        .sum(),
                },
            );
        }
        result
    }

    pub fn delete(&mut self, id: TransactionId) {
        self.transactions.remove(&id);
    }

    pub fn insert(&mut self, tr: impl Into<Transaction>) {
        let id = self.next_id();
        self.transactions.insert(id, tr.into());
    }

    fn next_id(&mut self) -> TransactionId {
        self.last_id += 1;
        self.last_id
    }

    pub fn timeline_data(&self, start: Date, end: Date) -> Option<TimelineData> {
        let n_days = {
            let n_days = (end - start).num_days();
            if n_days < 0 {
                return None
            }
            n_days as usize
        };
        let transactions: Vec<(u32, Transaction)> = self
            .transactions
            .iter()
            .filter(|(id, tr)| tr.date >= start && tr.date < end)
            .map(|(id, tr)| ((*id).clone(), (*tr).clone()))
            .collect_vec();
        let mut timeline_data = Vec::with_capacity(n_days);
        let mut balance = 0;
        for (i, date) in start.iter_days().take(n_days).enumerate() {
            let mut daily_income = 0;
            let mut daily_expenses = 0;
            for (_id, tr) in transactions.iter().filter(|(id, tr)| tr.date == date ) {
                match tr.kind {
                    TransactionKind::Income => {
                        balance += tr.value;
                        daily_income += tr.value;
                    }
                    TransactionKind::Expense => {
                        balance -= tr.value;
                        daily_expenses += tr.value;
                    }
                }
            }
            timeline_data[i] = (daily_income, daily_expenses, balance);
        }
        Some(timeline_data)
    }
    
    pub fn transactions(&self) -> BTreeMap<TransactionId, Transaction> {
        self.transactions.clone()
    }
    
}

pub struct Pipeline<T: IntoIterator<Item = TransactionRecord>>(T);

impl<T: IntoIterator<Item = TransactionRecord>> Pipeline<T> {
    pub fn kind(
        self,
        kind: TransactionKind,
    ) -> Pipeline<impl IntoIterator<Item = TransactionRecord>> {
        Pipeline(self.0.into_iter().filter(move |(id, tr)| tr.kind == kind))
    }

    pub fn before(self, date: Date) -> Pipeline<impl IntoIterator<Item = TransactionRecord>> {
        Pipeline(self.0.into_iter().filter(move |(id, tr)| tr.date < date))
    }

    pub fn after(self, date: Date) -> Pipeline<impl IntoIterator<Item = TransactionRecord>> {
        Pipeline(self.0.into_iter().filter(move |(id, tr)| tr.date >= date))
    }

    pub fn between(
        self,
        start: Date,
        end: Date,
    ) -> Pipeline<impl IntoIterator<Item = TransactionRecord>> {
        self.after(start).before(end)
    }

    pub fn values(self) -> impl Iterator<Item = Dollars> {
        self.0.into_iter().map(|(id, tr)| tr.value)
    }
}

impl<T: IntoIterator<Item = TransactionRecord>> From<T> for Pipeline<T> {
    fn from(value: T) -> Self {
        Pipeline(value)
    }
}

impl<T: IntoIterator<Item = TransactionRecord>> IntoIterator for Pipeline<T> {
    type Item = T::Item;
    type IntoIter = T::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub trait AsPipeline: IntoIterator<Item = TransactionRecord> + Sized + Clone {
    fn pipeline(&self) -> Pipeline<Self>;
}

impl<T: IntoIterator<Item = TransactionRecord> + Sized + Clone> AsPipeline for T {
    fn pipeline(&self) -> Pipeline<Self> {
        Pipeline(self.clone())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DateSummary {
    pub income: Dollars,
    pub expenses: Dollars,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    pub kind: TransactionKind,
    pub value: Dollars,
    pub date: Date,
}

impl From<(TransactionKind, Dollars, Date)> for Transaction {
    fn from(value: (TransactionKind, Dollars, Date)) -> Self {
        Transaction {
            kind: value.0,
            value: value.1,
            date: value.2,
        }
    }
}
