use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TimeManager {
    turn_start: Instant,
    limit_max_depth: Option<u8>,
    limit_max_nodes: Option<u64>,
    limit_soft_time: Option<u32>,
    limit_hard_time: Option<u32>,
}

impl TimeManager {
    pub fn new_time(time: u32, increment: u32) -> TimeManager {
        let limit_soft_time = TimeManager::soft_time(time, increment);
        let limit_hard_time = TimeManager::hard_time(time, increment);
        debug_assert!(limit_soft_time < limit_hard_time);
        TimeManager {
            turn_start: Instant::now(),
            limit_soft_time: Some(limit_soft_time),
            limit_hard_time: Some(limit_hard_time),
            limit_max_nodes: None,
            limit_max_depth: None,
        }
    }

    pub fn new_depth(depth: u8) -> TimeManager {
        TimeManager {
            turn_start: Instant::now(),
            limit_soft_time: None,
            limit_hard_time: None,
            limit_max_nodes: None,
            limit_max_depth: Some(depth),
        }
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    // A u32 in milliseconds can hold 1.1k hours.
    pub fn soft_limit(&self) -> bool {
        let Some(limit) = self.limit_soft_time else {
            return false;
        };
        let spent = self.turn_start.elapsed().as_millis() as u32;
        spent > limit
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    // A u32 in milliseconds can hold 1.1k hours.
    pub fn hard_limit(&self) -> bool {
        let Some(limit) = self.limit_hard_time else {
            return false;
        };
        let spent = self.turn_start.elapsed().as_millis() as u32;
        spent > limit
    }

    #[inline]
    pub fn depth_limit(&self, next_depth: u8) -> bool {
        let Some(max_depth) = self.limit_max_depth else {
            return false;
        };
        next_depth >= max_depth
    }

    #[inline]
    pub fn node_limit(&self, node_count: u64) -> bool {
        let Some(max_nodes) = self.limit_max_nodes else {
            return false;
        };
        max_nodes > node_count
    }

    #[allow(clippy::integer_division)]
    //time is in ms, so the floor divide seems like a resonable approach
    fn soft_time(time: u32, increment: u32) -> u32 {
        time / 22 + increment / 2
    }

    #[allow(clippy::integer_division)]
    fn hard_time(time: u32, increment: u32) -> u32 {
        time / 17 + increment / 2
    }
}
