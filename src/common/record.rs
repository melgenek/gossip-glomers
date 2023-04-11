use std::cmp::Ordering;
use std::time::Instant;

#[derive(Debug)]
pub struct Record<A> {
    pub timestamp: Instant,
    pub value: A,
}

impl<A> PartialEq<Self> for Record<A> {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl<A> Eq for Record<A> {}

impl<A> PartialOrd<Self> for Record<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.timestamp < other.timestamp {
            Some(Ordering::Greater)
        } else if self.timestamp == other.timestamp {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<A> Ord for Record<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
