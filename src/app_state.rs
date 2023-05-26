use itertools::Itertools;
use std::{collections::BTreeMap, rc::Rc};
use yew::Reducible;

pub type Date = chrono::NaiveDate;
pub type Dollars = i32;
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

impl Default for DateSummary {
    fn default() -> Self {
        Self {
            date: Default::default(),
            income: 0,
            expenses: 0,
            balance: 0,
        }
    }
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

impl IntoIterator for TimelineData {
    type IntoIter = <Vec<DateSummary> as IntoIterator>::IntoIter;
    type Item = <Vec<DateSummary> as IntoIterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DateRange {
    pub start: Date,
    pub end: Date,
}

impl DateRange {
    pub fn contains(&self, date: &Date) -> bool {
        self.start
            .iter_days()
            .take_while(|d| *d <= self.end)
            .contains(date)
    }
}

impl From<(Date, Date)> for DateRange {
    fn from(value: (Date, Date)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Entry {
    Create(Transaction),
    Delete(TransactionId),
    SetDate(DateRange),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Log {
    entries: Vec<Entry>,
}

impl Reducible for Log {
    type Action = Entry;
    fn reduce(self: Rc<Self>, event: Entry) -> Rc<Self> {
        let mut entries = self.entries.clone();
        entries.push(event);
        Self { entries }.into()
    }
}

impl Log {
    pub fn entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }
    
    pub fn append(&mut self, e: Entry) {
        self.entries.push(e)
    }

    pub fn transaction_records(&self) -> Vec<TransactionRecord> {
        let mut transaction_records = BTreeMap::new();
        let mut id_iter = 0_u16..;
        for entry in &self.entries {
            match entry {
                Entry::Create(t) => {
                    let id = id_iter.next().unwrap();
                    transaction_records.insert(id, t);
                }
                Entry::Delete(id) => {
                    transaction_records.remove(id);
                }
                _ => {}
            }
        }
        transaction_records
            .iter()
            // this clone could be removed using TransactionRecord<'a>
            .map(|(id, t)| (*id, (*t).clone()).into())
            .collect_vec()
    }

    pub fn create_entries(&self) -> Vec<(usize, TransactionRecord)> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e {
                Entry::Create(t) => Some((i, (i as TransactionId, (*t).clone()).into())),
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

    pub fn latest_create(&self, id: &TransactionId) -> Option<usize> {
        self.create_entries()
            .iter()
            .find_map(|(i, tr)| if tr.id == *id { Some(*i) } else { None })
    }

    pub fn date_range(&self) -> DateRange {
        self.entries
            .iter()
            .rev()
            .find_map(|e| match e {
                Entry::SetDate(date_range) => Some((*date_range).clone()),
                _ => None,
            })
            .unwrap_or(DateRange {
                start: "2023-01-01".parse().unwrap(),
                end: "2023-01-31".parse().unwrap(),
            })
    }

    pub fn timeline_data(&self) -> TimelineData {
        let DateRange { start, end } = self.date_range();
        let days = start.iter_days().take_while(|d| *d <= end).collect_vec();
        let mut timeline_data = Vec::<DateSummary>::with_capacity(days.len());
        let mut balance = 0i32;
        for (i, day) in days.iter().enumerate() {
            let mut date_summary = DateSummary::default();
            date_summary.date = *day;
            for tr in self.transaction_records() {
                if tr.transaction.date == date_summary.date {
                    match tr.transaction.kind {
                        TransactionKind::Income => {
                            date_summary.income = date_summary
                                .income
                                .checked_add(tr.transaction.value)
                                .unwrap_or(Dollars::MAX);
                            balance = balance
                                .checked_add(tr.transaction.value)
                                .unwrap_or(Dollars::MIN);
                        }
                        TransactionKind::Expense => {
                            date_summary.expenses = date_summary
                                .expenses
                                .checked_add(tr.transaction.value)
                                .unwrap_or(Dollars::MIN);
                            balance = balance
                                .checked_sub(tr.transaction.value)
                                .unwrap_or(Dollars::MIN);
                        }
                    }
                }
            }
            date_summary.balance = balance;
            timeline_data.insert(i, date_summary)
        }
        TimelineData(timeline_data)
    }
}

#[cfg(test)]
mod test {

    use std::marker::PhantomData;

    use super::{
        Date, DateRange, DateSummary, Dollars, Entry, Log, Transaction, TransactionId,
        TransactionKind, TransactionRecord,
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
    impl Predicate for () {}
    #[derive(Debug, PartialEq, Clone)]
    struct NonEmpty;
    impl Predicate for NonEmpty {}

    #[derive(Debug, PartialEq, Clone)]
    struct PredicatedLog<P: Predicate> {
        log: Log,
        _phantom_data: PhantomData<P>,
    }

    impl<P: Predicate> PredicatedLog<P> {
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

    impl Arbitrary for PredicatedLog<()> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                log: Log {
                    entries: Vec::<Entry>::arbitrary(g),
                },
                _phantom_data: PhantomData,
            }
        }
    }

    // 1. For every non-deleted transaction, there exists exactly one
    //    date summary whose date is equal to the transaction date
    #[quickcheck]
    fn test_timeline_data_1(log: PredicatedLog<NonEmpty>) -> bool {
        let log = log.into_inner();
        let summaries = log.timeline_data();
        let transaction_records = log.transaction_records();
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

    // after setting the date range, the timeline data
    // only contains dates in that range
    #[quickcheck]
    fn test_timeline_data_2(log: PredicatedLog<()>, date_range: DateRange) -> bool {
        let mut log = log.into_inner();
        log.entries.push(Entry::SetDate(date_range.clone()));
        for DateSummary { date, .. } in log.timeline_data() {
            if !date_range.contains(&date) {
                return false;
            }
        }
        return true;
    }

}
