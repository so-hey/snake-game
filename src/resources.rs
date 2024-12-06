use bevy::prelude::*;
use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

#[derive(Resource)]
pub struct IntervalSetting<T> {
    last_update: Instant,
    phantom: PhantomData<T>,
}

impl<T> Default for IntervalSetting<T> {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            phantom: PhantomData,
        }
    }
}

impl<T> IntervalSetting<T> {
    pub fn check(&self, time: Instant, diff: Duration) -> bool {
        time - self.last_update >= diff
    }

    pub fn update(&mut self, time: Instant) {
        self.last_update = time;
    }
}
