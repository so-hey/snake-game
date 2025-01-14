use bevy::prelude::*;
use burn::{
    backend::Wgpu,
    tensor::{Tensor, TensorData},
};
use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

use crate::components::Position;

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
    pub fn reset(&mut self) {
        self.last_update = Instant::now();
    }
}

#[derive(Resource)]
pub struct CounterSetting<T> {
    count: usize,
    phantom: PhantomData<T>,
}

impl<T> Default for CounterSetting<T> {
    fn default() -> Self {
        Self {
            count: 0,
            phantom: PhantomData,
        }
    }
}

impl<T> CounterSetting<T> {
    pub fn less_than(&self, max: usize) -> bool {
        self.count < max
    }
    pub fn increment(&mut self) {
        if self.count < usize::MAX {
            self.count += 1;
        }
    }
    pub fn decrement(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }
    pub fn reset(&mut self) {
        self.count = 0;
    }
    pub fn is_even(&self) -> bool {
        self.count % 2 == 0
    }
}

#[derive(Resource, Default)]
pub struct FoodCenter {
    cnt: i32,
    pos: Position,
}

impl FoodCenter {
    pub fn get_pos(&self) -> Position {
        self.pos
    }
    pub fn add(&mut self, pos: &Position) {
        self.cnt += 1;
        if self.cnt <= 0 {
            return;
        }
        self.pos = (self.pos * self.cnt + *pos) / self.cnt;
    }
    pub fn remove(&mut self, pos: &Position) {
        self.cnt -= 1;
        if self.cnt < 1 {
            self.cnt = 0;
            return;
        }
        self.pos = (self.pos * self.cnt - *pos) / self.cnt;
    }
}

#[derive(Resource)]
pub struct MenuData {
    pub button_entity: Entity,
}

#[derive(Resource, Default)]
pub struct PlayerScore {
    history: [[[f32; 28]; 28]; 2],
    food_count: u8,
}
impl PlayerScore {
    pub fn add(&mut self, pos: Position) {
        let i = pos.x() as usize / 28;
        let x = pos.x() as usize % 28;
        let y = pos.y() as usize;

        self.history[i][x][y] += 1.;
    }
    pub fn increment(&mut self) {
        self.food_count += 1;
    }
    pub fn get_score(&self) -> u8 {
        type MyBackend = Wgpu<f32, i32>;

        let device = burn::backend::wgpu::WgpuDevice::default();
        let artifact_dir = "data";

        let mut score = self.food_count;

        for i in 0..2 {
            let data = self.history[i]
                .iter()
                .flat_map(|row| row.iter())
                .copied()
                .collect();
            let tensor_data = TensorData::new(data, [1, 28, 28]);
            score += crate::inference::infer::<MyBackend>(
                artifact_dir,
                device.clone(),
                Tensor::from_data(tensor_data.clone(), &device),
            );
        }

        score
    }
}
