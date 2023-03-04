pub mod automata;
pub mod drunkard;
pub mod empty;
pub mod prefab;
pub mod square;

use crate::prelude::*;
use serde::Deserialize;

pub mod prelude {
    pub use crate::map_builder::automata::*;
    pub use crate::map_builder::drunkard::*;
    pub use crate::map_builder::empty::*;
    pub use crate::map_builder::prefab::*;
    pub use crate::map_builder::square::*;
    pub use crate::map_builder::*;
}

pub trait MapBuilder {
    fn build(
        &self,
        rng: &mut RandomNumberGenerator,
        width: i32,
        height: i32,
        num_monsters: usize,
    ) -> BuiltMap;
}

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct MapTheme {
    seen_wall: char,
    seen_floor: char,
    seen_exit: char,
    mapped_wall: char,
    mapped_floor: char,
    mapped_exit: char,
    not_seen: char,
}

impl MapTheme {
    pub fn tile_to_render(&self, tile_type: TileType, revealed: Revealed) -> char {
        match (tile_type, revealed) {
            (TileType::Wall, Revealed::Seen) => self.seen_wall,
            (TileType::Floor, Revealed::Seen) => self.seen_floor,
            (TileType::Exit, Revealed::Seen) => self.seen_exit,
            (TileType::Wall, Revealed::Mapped) => self.mapped_wall,
            (TileType::Floor, Revealed::Mapped) => self.mapped_floor,
            (TileType::Exit, Revealed::Mapped) => self.mapped_exit,
            (_, Revealed::NotSeen) => self.not_seen,
        }
    }
}

pub struct BuiltMap {
    pub map: Map,
    pub entity_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

pub fn determine_entity_spawn_points(
    map: &Map,
    player_pos: Point,
    rng: &mut RandomNumberGenerator,
    num_monsters: usize,
) -> Vec<Point> {
    let mut spawnable_tiles = map
        .tiles
        .iter()
        .enumerate()
        .filter(|(idx, tile)| {
            **tile == TileType::Floor
                && DistanceAlg::Pythagoras.distance2d(player_pos, map.index_to_point2d(*idx)) > 10.0
        })
        .map(|(idx, _)| map.index_to_point2d(idx))
        .collect::<Vec<Point>>();

    let mut spawns = Vec::new();
    for _ in 0..num_monsters {
        rng.random_slice_index(&spawnable_tiles)
            .iter()
            .for_each(|target_index| {
                spawns.push(spawnable_tiles[*target_index]);
                spawnable_tiles.remove(*target_index);
            });
    }
    spawns
}
