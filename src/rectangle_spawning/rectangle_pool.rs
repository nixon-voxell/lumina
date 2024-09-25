use crate::rectangle_spawning::rectangle_entity::RectangleConfig;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Resource)]
pub struct RectanglePool {
    available: VecDeque<RectangleConfig>,
    max_size: usize,
}

impl RectanglePool {
    pub fn new(max_size: usize) -> Self {
        Self {
            available: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn get(&mut self) -> Option<RectangleConfig> {
        self.available.pop_front()
    }

    #[allow(dead_code)]
    pub fn return_to_pool(&mut self, config: RectangleConfig) {
        if self.available.len() < self.max_size {
            self.available.push_back(config);
        }
    }

    pub fn preload(&mut self, count: usize) {
        for _ in 0..count {
            let config = RectangleConfig::default();
            self.available.push_back(config);
        }
    }

    #[allow(dead_code)]
    pub fn available_count(&self) -> usize {
        self.available.len()
    }
}
