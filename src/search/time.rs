use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

#[derive(Debug)]
pub struct TimeManager {
    turn_start: Instant,
    limit_max_depth: Option<u8>,
    limit_max_nodes: Option<u64>,
    limit_soft_time: Option<u32>,
    limit_hard_time: Option<u32>,
    stop: AtomicBool,
}

impl TimeManager {
    #[inline]
    pub fn new(
        depth: Option<u8>,
        nodes: Option<u64>,
        time_and_inc: Option<(u32, u32)>,
    ) -> TimeManager {
        let mut limit_soft_time = None;
        let mut limit_hard_time = None;
        if let Some((time, inc)) = time_and_inc {
            limit_soft_time = Some(TimeManager::soft_time(time, inc));
            limit_hard_time = Some(TimeManager::hard_time(time, inc));
        }

        TimeManager {
            turn_start: Instant::now(),
            limit_max_nodes: nodes,
            limit_max_depth: depth,
            limit_soft_time,
            limit_hard_time,
            stop: AtomicBool::new(false),
        }
    }

    #[inline]
    pub fn stop(&self) {
        self.stop.swap(true, Ordering::Relaxed);
    }

    #[inline]
    #[allow(clippy::integer_division)]
    #[allow(clippy::cast_possible_truncation)]
    // NPS will *probably* never going that fast
    // we want the floor divide here for a whole num
    pub fn calc_nps(&self, nodes: u64) -> u32 {
        // + 1 to ensure its never 0
        let nanos = self.turn_start.elapsed().as_nanos() + 1;
        (u128::from(nodes) * 1_000_000_000 / nanos) as u32
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    // millis in a u32 holds 1.1k hours
    pub fn time(&self) -> u32 {
        self.turn_start.elapsed().as_millis() as u32
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    // A u32 in milliseconds can hold 1.1k hours.
    pub fn soft_limit(&self) -> bool {
        if self.stop.load(Ordering::Relaxed) {
            return true;
        }
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
        if self.stop.load(Ordering::Relaxed) {
            return true;
        }
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
    //time is in ms, so the floor divide seems like a resonable approach
    fn hard_time(time: u32, increment: u32) -> u32 {
        time / 17 + increment / 2
    }
}
