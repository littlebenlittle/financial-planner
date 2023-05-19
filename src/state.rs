use itertools::Itertools;
use std::collections::BTreeMap;

pub type TransactionId = u32;
pub type Date = String;
pub type Dollars = u32;
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

    pub fn date_summaries(&self) -> DateSummaries {
        let dates = self.transactions.values().map(|e| e.date.clone()).unique();
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

    pub fn insert(&mut self, tr: Transaction) {
        let id = self.next_id();
        self.transactions.insert(id, tr);
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
    pub date: String,
}
