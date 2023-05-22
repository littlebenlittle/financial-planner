use chrono::Duration;
use itertools::Itertools;
use std::{ops::Add, rc::Rc, str::FromStr};
use yew::Reducible;

// #[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub type Date = chrono::NaiveDate;

// impl Add<Duration> for Date {
//     type Output = Self;
//     fn add(self, rhs: Duration) -> Self::Output {
//         Self(self.0 + rhs)
//     }
// }
//
// impl FromStr for Date {
//     type Err = <chrono::NaiveDate as FromStr>::Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self(s.parse()?))
//     }
// }

pub type Dollars = u32;
pub type TransactionId = u16;

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
    /// returns the date of the earliest summary. Returns `None` if
    /// list is empty.
    pub fn start_date(&self) -> Option<Date> {
        // if list is known to be sorted and non-empty, this is more efficient:
        // `Some(self.0[0].date)`
        self.iter().map(|s| s.date).min()
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

impl Default for TransactionsListData {
    fn default() -> Self {
        Self {
            transactions: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    CreateTransaction(Transaction),
    DeleteTransaction(TransactionId),
    SetDateRange { from: Date, to: Date },
}

impl Action {
    pub fn try_into_create_id(&self) -> Option<TransactionId> {
        match self {
            Self::CreateTransaction(tr) => Some(tr.id),
            _ => None,
        }
    }
    pub fn try_into_delete_id(&self) -> Option<TransactionId> {
        match self {
            Self::DeleteTransaction(id) => Some(*id),
            _ => None,
        }
    }
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
        compute_timeline_data(&self.0)
    }
    pub fn transactions_list_data(&self) -> TransactionsListData {
        compute_transactions_list_data(&self.0)
    }
    pub fn transactions(&self) -> Vec<Transaction> {
        compute_transactions(&self.0)
    }
}

fn compute_timeline_data(actions: &Vec<Action>) -> TimelineData {
    // Spec:
    // 1. Every date in range is part of the data
    // 2. The data is sorted by the date of the transaction
    // 3. Every date in the data has an income summary equal to the sum
    //    of the value of every non-deleted income record ocurring on that date
    // 4. Every date in the data has an expense summary equal to the sum
    //    of the value of every non-deleted expense record ocurring on that date
    // 5. Every date in the data has a balance summary equal to the sum
    //    of the value of every non-deleted income record ocurring before that date
    //    less the sum of the value of every non-deleted expense record ocurring before
    //    that date
    Default::default()
}

fn compute_transactions_list_data(actions: &Vec<Action>) -> TransactionsListData {
    // Spec:
    // 1. Every non-deleted transaction is present in the data
    // 2. No deleted transaction is present in the data
    // 3. The data is sorted by the date of the transaction
    Default::default()
}

fn compute_transactions(log: &Vec<Action>) -> Vec<Transaction> {
    // Spec:
    // 1. Every non-deleted transaction is present in the data
    // 2. No deleted transaction is present in the data
    let mut transactions = Vec::<Transaction>::new();
    let mut id_iter = 0..;
    for action in log {
        match action {
            Action::DeleteTransaction(id) => {
                if let Some((n, _tr)) = transactions.iter().find_position(|tr| tr.id == *id) {
                    transactions.remove(n);
                }
            }
            Action::CreateTransaction(tr) => {
                transactions.push(tr.clone());
            }
            _ => {}
        }
    }
    Default::default()
}

#[cfg(test)]
mod test {

    use super::{
        compute_timeline_data, compute_transactions, compute_transactions_list_data, Action, Date,
        Dollars, TimelineData, Transaction, TransactionId, TransactionKind, TransactionsListData,
    };
    use chrono::NaiveDate;
    use itertools::Itertools;
    use quickcheck::Arbitrary;

    fn arbitrary_range<T: Clone>(
        g: &mut quickcheck::Gen,
        range: impl Iterator<Item = T>,
    ) -> Option<T> {
        g.choose((range.collect_vec()).as_slice())
            .map(|t| (*t).clone())
    }

    #[derive(Clone)]
    struct DateWrapper(Date);

    impl Arbitrary for DateWrapper {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let date = NaiveDate::from_ymd_opt(
                    arbitrary_range(g, 1900_i32..2100).unwrap(),
                    arbitrary_range(g, 1_u32..13).unwrap(),
                    arbitrary_range(g, 1_u32..32).unwrap(),
                );
                if date.is_some() {
                    return Self(date.unwrap());
                }
            }
        }
    }

    impl Arbitrary for Action {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match g.choose(&[1, 2, 3]).unwrap() {
                1 => Self::DeleteTransaction(TransactionId::arbitrary(g)),
                2 => Self::CreateTransaction(Transaction::arbitrary(g)),
                3 => Self::SetDateRange {
                    from: DateWrapper::arbitrary(g).0,
                    to: DateWrapper::arbitrary(g).0,
                },
                _ => unreachable!(),
            }
        }
    }

    impl Arbitrary for Transaction {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Transaction {
                value: Dollars::arbitrary(g),
                kind: *g.choose(&[TransactionKind::Income, TransactionKind::Expense]).unwrap(),
                date: DateWrapper::arbitrary(g).0,
                id: TransactionId::arbitrary(g),
            }
        }
    }

    // 1. For every non-deleted transaction, there exists exactly one
    //    date summary whose date is equal to the transaction date
    #[quickcheck]
    fn test_compute_timeline_data_1(log: Vec<Action>) -> bool {
        let summaries = compute_timeline_data(&log);
        let transactions = compute_transactions(&log);
        for transaction in transactions {
            if summaries.iter().map(|s| s.date).contains(&transaction.date) {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }

    // compute transactions
    // 1. This list is empty only if
    //    - there are no RecordTransaction actions
    //    - every RecordTransaction action is eventually followed by
    //      DeleteTransaction action with the same transaction id
    #[quickcheck]
    fn test_compute_transactions_1(log: Vec<Action>) -> bool {
        let transactions = compute_transactions(&log);
        if transactions.is_empty() {
            return for_all_eventually(
                &log,
                |a| a.try_into_create_id(),
                |b| b.try_into_delete_id(),
                |a_id, b_id| a_id == b_id,
            )
            .is_some();
        }
        true
    }

    fn for_all_eventually<P, Q, R>(seq: &Vec<Action>, p: P, q: Q, r: R) -> Option<(&Action, &Action)>
    where
        P: Fn(&Action) -> Option<TransactionId>,
        Q: Fn(&Action) -> Option<TransactionId>,
        R: Fn(&TransactionId, &TransactionId) -> bool,
    {
        let firsts = seq.iter().filter_map(p).enumerate();
        let seconds = seq.iter().filter_map(q).enumerate();
        for ((i, f), (j, s)) in firsts.zip(seconds) {
            if i < j && r(&f, &s) {
                return Some((&seq[i], &seq[j]))
            }
        }
        return None
    }
}
