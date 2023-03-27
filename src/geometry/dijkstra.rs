use crate::prelude::*;

use float_ord::FloatOrd;
use std::{collections::BinaryHeap, f32::MAX};

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
        let mut queue: BinaryHeap<QueueEntry> =
            starts.iter().map(|p| QueueEntry(*p, 0.0)).collect();
        while let Some(QueueEntry(index, distance)) = queue.pop() {
            if distance <= max_depth && distance < self.map[index] {
                self.map[index] = distance;
                let exits = map.get_available_exits(index);
                exits
                    .iter()
                    .map(|(index, cost)| QueueEntry(*index, distance + cost))
                    .for_each(|exit| queue.push(exit));
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

struct QueueEntry(usize, f32);
impl Eq for QueueEntry {}
impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && FloatOrd(self.1) == FloatOrd(other.1)
    }
}

// Compare "backwards" because the queue want to sort max first
// where we want min first
impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        FloatOrd(other.1).cmp(&FloatOrd(self.1))
    }
}
impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
