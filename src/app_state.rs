use itertools::Itertools;
use std::{collections::BTreeMap, iter::FlatMap, marker::PhantomData, ops::RangeFrom, rc::Rc};
use wrapper::Wrapper;
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionRecord {
    pub transaction: Transaction,
    pub id: TransactionId,
}

impl From<(TransactionId, Transaction)> for TransactionRecord {
    fn from(value: (TransactionId, Transaction)) -> Self {
        Self {
            transaction: Transaction {
                value: value.1.value,
                kind: value.1.kind,
                date: value.1.date,
            },
            id: value.0,
        }
    }
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
    pub transaction_records: Vec<TransactionRecord>,
}

impl Default for TransactionsListData {
    fn default() -> Self {
        Self {
            transaction_records: Default::default(),
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
    pub fn is_create(&self) -> bool {
        match self {
            Self::CreateTransaction(_) => true,
            _ => false,
        }
    }

    pub fn is_delete(&self) -> bool {
        match self {
            Self::DeleteTransaction(_) => true,
            _ => false,
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
    pub fn transactions(&self) -> Vec<TransactionRecord> {
        compute_transaction_records(&self.0)
    }
}

fn compute_timeline_data(log: &Vec<Action>) -> TimelineData {
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
    let (start_date, end_date) = compute_date_range(log);
    let mut data = TimelineData::default();
    let mut balance = 0;
    for date in start_date.iter_days().take_while(|d| d <= &end_date) {
        let income = compute_income_on(&log, &date);
        let expenses = compute_expenses_on(&log, &date);
        balance += income - expenses;
        data.0.push(DateSummary {
            date,
            income,
            expenses,
            balance,
        });
    }
    data
}

fn compute_date_range(log: &Vec<Action>) -> (Date, Date) {
    log.iter()
        .rev()
        .find_map(|a: &Action| match a {
            Action::SetDateRange { from, to } => Some((*from, *to)),
            _ => None,
        })
        .unwrap_or(("2023-01-01".parse().unwrap(), "2023-01-31".parse().unwrap()))
}

fn compute_income_on(log: &Vec<Action>, on: &Date) -> Dollars {
    compute_transaction_records(log)
        .iter()
        .filter_map(
            |TransactionRecord {
                 transaction: Transaction { date, value, .. },
                 ..
             }| if on == date { Some(value) } else { None },
        )
        .sum()
}

fn compute_expenses_on(log: &Vec<Action>, date: &Date) -> Dollars {
    Default::default()
}

fn compute_transactions_list_data(log: &Vec<Action>) -> TransactionsListData {
    // Spec:
    // 1. Every non-deleted transaction is present in the data
    // 2. No deleted transaction is present in the data
    // 3. The data is sorted by the date of the transaction
    let mut transaction_records = compute_transaction_records(log);
    transaction_records.sort_by(|a, b| a.transaction.date.cmp(&b.transaction.date));
    TransactionsListData {
        transaction_records,
    }
}

fn compute_transaction_records(log: &Vec<Action>) -> Vec<TransactionRecord> {
    // Spec:
    // 1. Every non-deleted transaction is present in the data
    // 2. No deleted transaction is present in the data
    let mut id_iter = 0..;
    let mut transactions = Vec::<TransactionRecord>::new();
    for action in log {
        match action {
            Action::DeleteTransaction(id) => {
                if let Some((n, _tr)) = transactions.iter().find_position(|tr| tr.id == *id) {
                    transactions.remove(n);
                }
            }
            Action::CreateTransaction(tr) => {
                let id = id_iter.next().unwrap();
                transactions.push((id, tr.clone()).into());
            }
            _ => {}
        }
    }
    transactions
}

#[derive(Debug, PartialEq, Clone)]
struct DateRange {
    start: Date,
    end: Date,
}

#[derive(Debug, PartialEq, Clone)]
enum Entry {
    Create(Transaction),
    Delete(TransactionId),
    SetDate(DateRange),
}

#[derive(Debug, PartialEq, Clone)]
struct Log {
    entries: Vec<Entry>,
}

impl Log {
    pub fn transaction_records(&self) -> Vec<TransactionRecord> {
        let mut transaction_records = BTreeMap::new();
        let mut id_iter = 0_u16..;
        for entry in self.entries {
            match entry {
                Entry::Create(t) => {
                    let id = id_iter.next().unwrap();
                    transaction_records.insert(id, t);
                }
                Entry::Delete(id) => {
                    transaction_records.remove(&id);
                }
                _ => {}
            }
        }
        transaction_records
            .iter()
            .map(|(id, t)| (*id, *t).into())
            .collect_vec()
    }

    pub fn create_entries(&self) -> Vec<(usize, Transaction)> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e {
                Entry::Create(t) => Some((i, *t)),
                _ => None,
            })
            .collect_vec()
    }

    pub fn delete_entries(&self) -> Vec<(usize, TransactionId)> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e {
                Entry::Delete(id) => Some((i, *id)),
                _ => None,
            })
            .collect_vec()
    }

    pub fn transaction_id_at(&self, index: usize) -> TransactionId {
        self.entries
            .iter()
            .filter(|e| match e {
                Entry::Create(_) => true,
                _ => false,
            })
            .count() as TransactionId
    }

}

#[cfg(test)]
mod test {

    use std::marker::PhantomData;

    use super::{
        compute_timeline_data, compute_transaction_records, compute_transactions_list_data, Action,
        Date, DateRange, Dollars, Entry, Log, TimelineData, Transaction, TransactionId,
        TransactionKind, TransactionRecord, TransactionsListData,
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

    impl DateWrapper {
        pub fn into_inner(self) -> Date {
            self.0
        }
    }

    impl Arbitrary for DateWrapper {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let date = NaiveDate::from_ymd_opt(
                    arbitrary_range(g, 2020..2025).unwrap(),
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
                kind: *g
                    .choose(&[TransactionKind::Income, TransactionKind::Expense])
                    .unwrap(),
                date: DateWrapper::arbitrary(g).0,
            }
        }
    }

    impl Arbitrary for TransactionRecord {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            TransactionRecord {
                transaction: Transaction::arbitrary(g),
                id: arbitrary_range(g, 1..50).unwrap(), // limit id range to get overlaps
            }
        }
    }

    // 1. For every non-deleted transaction, there exists exactly one
    //    date summary whose date is equal to the transaction date
    #[quickcheck]
    fn test_compute_timeline_data_1(log: Vec<Action>) -> bool {
        let summaries = compute_timeline_data(&log);
        let transaction_records = compute_transaction_records(&log);
        for tr in transaction_records {
            if summaries
                .iter()
                .map(|s| s.date)
                .contains(&tr.transaction.date)
            {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }

    impl Arbitrary for Entry {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match g.choose(&[1, 2, 3]).unwrap() {
                1 => Self::Create(Transaction::arbitrary(g)),
                2 => Self::Delete(TransactionId::arbitrary(g)),
                3 => Self::SetDate(DateRange::arbitrary(g)),
                _ => unreachable!(),
            }
        }
    }

    impl Arbitrary for DateRange {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                start: DateWrapper::arbitrary(g).into_inner(),
                end: DateWrapper::arbitrary(g).into_inner(),
            }
        }
    }

    trait Predicate {}
    #[derive(Debug, PartialEq, Clone)]
    struct NonEmpty;
    impl Predicate for NonEmpty {}

    #[derive(Debug, PartialEq, Clone)]
    struct PredicatedLog<P: Predicate> {
        log: Log,
        _phantom_data: PhantomData<P>,
    }

    impl PredicatedLog<NonEmpty> {
        fn into_inner(self) -> Log {
            self.log
        }
    }

    impl Arbitrary for PredicatedLog<NonEmpty> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut entries = Vec::<Entry>::arbitrary(g);
            while entries.len() == 0 {
                entries = Vec::<Entry>::arbitrary(g)
            }
            Self {
                log: Log { entries },
                _phantom_data: PhantomData,
            }
        }
    }

    // compute transactions list
    // When  the transactions list is empty
    // Then  every create entry is followed by a delete entry
    #[quickcheck]
    fn test_transaction_records_1(log: PredicatedLog<NonEmpty>) -> bool {
        let log = log.into_inner();
        let transaction_records = log.transaction_records();
        if transaction_records.is_empty() {
            for (i, transaction) in log.create_entries() {
                let create_id = log.transaction_id_at(i);
                let mut j = 0;
                for (jj, delete_id) in log.delete_entries().iter().skip(i) {
                    if create_id == *delete_id {
                        j = *jj;
                        break;
                    }
                }
                print!("counterexample: ({i},{j})");
                return false;
            }
        }
        true
    }

    // 2. If a transaction appears in the list, then there
    //    is no later action that deletes that transaction
    #[quickcheck]
    fn test_transaction_records_2(log: PredicatedLog<NonEmpty>) -> bool {
        let log = log.into_inner();
        let transaction_records = log.transaction_records();
        for transaction_record in transaction_records {
            let id = &transaction_record.id;
            match log.latest_create(&id) {
                None => {
                    println!("unspecified behavior: transaction in list but no corresponding create entry")
                }
                Some(create_index) => {
                    for (delete_index, delete_id) in log.delete_entries().iter().skip(create_index)
                    {
                        if delete_id == id {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
