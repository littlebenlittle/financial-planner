use itertools::Itertools;
use std::{collections::BTreeMap, error::Error};

pub type TransactionId = u32;
pub type Dollars = u32;
pub type Date = chrono::NaiveDate;
pub type DateSummaries = BTreeMap<Date, DateSummary>;
pub type Transactions = BTreeMap<TransactionId, Transaction>;

#[derive(Clone, PartialEq)]
pub struct State {
    pub transactions: Transactions,
    last_id: u32,
}

impl State {
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
