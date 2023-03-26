use crate::prelude::*;

use float_ord::FloatOrd;
use std::{collections::VecDeque, f32::MAX};

pub struct DijkstraMap {
    pub map: Vec<f32>,
}

impl DijkstraMap {
    pub fn new<T>(width: T, height: T, starts: &[usize], map: &Map, max_depth: f32) -> Self
    where
        T: TryInto<usize>,
    {
        let width = width.try_into().ok().unwrap();
        let height = height.try_into().ok().unwrap();
        let mut d = Self {
            map: vec![MAX; width * height],
        };
        d.build(starts, map, max_depth);
        d
    }

    fn build(&mut self, starts: &[usize], map: &Map, max_depth: f32) {
        let mut queue: VecDeque<(usize, f32)> = starts.iter().map(|p| (*p, 0.0)).collect();
        while let Some((index, distance)) = queue.pop_front() {
            if distance <= max_depth && distance < self.map[index] {
                self.map[index] = distance;
                let exits = map.get_available_exits(index);
                exits
                    .iter()
                    .map(|(index, cost)| (*index, distance + cost))
                    .for_each(|exit| queue.push_back(exit));
            }
        }
    }

    pub fn find_lowest_exit(&self, idx: usize, map: &Map) -> Option<usize> {
        map.get_available_exits(idx)
            .iter()
            .map(|(exit, _)| (*exit, self.map[*exit]))
            .filter(|(_, distance)| *distance < MAX)
            .min_by_key(|(_, distance)| FloatOrd(*distance))
            .map(|(index, _)| index)
    }
}
