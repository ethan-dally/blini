use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub enum TimeRemaining {
    Ample,
    SoftLimit,
    HardLimit,
}

#[derive(Debug)]
pub struct TimeManager {
    turn_start: Instant,
    soft_limit: u32,
    hard_limit: u32,
}

#[allow(clippy::integer_division)]
//time is in ms, so the floor divide seems like a resonable approach
impl TimeManager {
    fn soft_limit(time: u32, increment: u32) -> u32 {
        time / 22 + increment / 2
    }

    fn hard_limit(time: u32, increment: u32) -> u32 {
        time / 17 + increment / 2
    }

    pub fn new(time: u32, increment: u32) -> TimeManager {
        let turn_start = Instant::now();
        let soft_limit = TimeManager::soft_limit(time, increment);
        let hard_limit = TimeManager::hard_limit(time, increment);
        debug_assert!(hard_limit > soft_limit);
        TimeManager {
            turn_start,
            soft_limit,
            hard_limit,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    // A u32 in milliseconds can hold 1.1k hours.
    pub fn poll(&self) -> TimeRemaining {
        let spent = self.turn_start.elapsed().as_millis() as u32;
        if spent > self.hard_limit {
            return TimeRemaining::HardLimit;
        }
        if spent > self.soft_limit {
            return TimeRemaining::SoftLimit;
        }
        TimeRemaining::Ample
    }
}

#[cfg(test)]
mod tests {
    use std::{
        thread::sleep,
        time::Duration,
    };

    use super::*;

    #[test]
    fn time_manager_limits() {
        assert_eq!(TimeManager::soft_limit(1000, 0), 45);
        assert_eq!(TimeManager::hard_limit(1000, 0), 58);

        assert_eq!(TimeManager::soft_limit(1000, 100), 95);
        assert_eq!(TimeManager::hard_limit(1000, 100), 108);
    }

    #[test]
    fn time_manager_poll() {
        let manager = TimeManager::new(1000, 0);

        assert!(matches!(manager.poll(), TimeRemaining::Ample));

        sleep(Duration::from_millis(50));
        assert!(matches!(manager.poll(), TimeRemaining::SoftLimit));

        sleep(Duration::from_millis(15));
        assert!(matches!(manager.poll(), TimeRemaining::HardLimit));
    }
}
