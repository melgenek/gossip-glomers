use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

struct Entry<A> {
    time: Instant,
    timer_key: A,
}

impl<A> PartialEq<Self> for Entry<A> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<A> Eq for Entry<A> {}

impl<A> PartialOrd<Self> for Entry<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time < other.time {
            Some(Ordering::Greater)
        } else if self.time == other.time {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<A> Ord for Entry<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Timer<A> {
    timers: BinaryHeap<Entry<A>>,
}

impl<A> Timer<A> {
    pub fn new() -> Timer<A> {
        Timer {
            timers: BinaryHeap::new()
        }
    }

    pub fn add_timer(&mut self, time: Instant, timer_key: A) {
        self.timers.push(Entry { time, timer_key })
    }

    pub fn remove_expired_timers(&mut self, now: Instant) -> Vec<A> {
        let mut expired_timers = vec![];

        while let Some(Entry { time, .. }) = self.timers.peek() {
            if time <= &now {
                expired_timers.push(self.timers.pop().unwrap().timer_key);
            } else {
                break;
            }
        }
        expired_timers
    }

    pub fn duration_until_next_timer(&self, now: Instant) -> Duration {
        self.timers.peek().map_or_else(|| Duration::from_millis(0), |entry| entry.time.duration_since(now))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use std::time::{Duration, Instant};

    use crate::common::timer::Timer;

    #[test]
    fn should_remove_expired_timers() {
        let mut timer = Timer::new();
        let now = Instant::now();
        timer.add_timer(now.add(Duration::from_millis(1)), 1);
        timer.add_timer(now.add(Duration::from_millis(2)), 2);
        timer.add_timer(now.add(Duration::from_millis(3)), 3);
        timer.add_timer(now.add(Duration::from_millis(6)), 6);
        timer.add_timer(now.add(Duration::from_millis(7)), 7);

        let expired_timers = timer.remove_expired_timers(now.add(Duration::from_millis(5)));

        assert_eq!(expired_timers, vec![1, 2, 3]);
        assert_eq!(timer.duration_until_next_timer(now), Duration::from_millis(6));

        let expired_timers = timer.remove_expired_timers(now.add(Duration::from_millis(10)));

        assert_eq!(expired_timers, vec![6, 7]);
        assert_eq!(timer.duration_until_next_timer(now), Duration::from_millis(0));
    }
}
