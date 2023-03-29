use std::collections::{HashSet, VecDeque};

use crate::prelude::*;

pub fn movement_system(
    mut want_move_query: Query<(
        Entity,
        &WantsToMove,
        Option<&mut FieldOfView>,
        Option<&Player>,
    )>,
    mut map: ResMut<Map>,
    mut camera: ResMut<DCCamera>,

    mut commands: Commands,
) {
    for (entity, want_move, optional_fov, optional_player) in want_move_query.iter_mut() {
        if map.can_enter_tile(want_move.destination) {
            commands
                .entity(entity)
                .insert(Position(want_move.destination));

            if let Some(mut fov) = optional_fov {
                fov.is_dirty = true;
                if optional_player.is_some() {
                    camera.center_on_point(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        let was_revealed = map.reveal(*pos);
                        if was_revealed {
                            let neighbors = map.get_neighbors(map.point_to_index(*pos));
                            neighbors
                                .iter()
                                .for_each(|pt| find_encompassed_tiles(*pt, &mut map));
                        }
                    });
                }
            }
        }
        commands.entity(entity).remove::<WantsToMove>();
    }
}

/// Finds wall tiles that have been entirely encompassed
/// by Seen wall tiles. Such encompassed Wall tiles are
/// promoted from Unseen to Encompassed or from
/// Mapped to Seen
/// Basically this is a flood fill algorithm
fn find_encompassed_tiles(point: Point, map: &mut Map) {
    let mut visited = HashSet::new();
    let mut encompassed = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_front(point);
    while let Some(point) = queue.pop_back() {
        if map.in_bounds(point) {
            let index = map.point_to_index(point);
            if !visited.contains(&index) {
                visited.insert(index);
                let tile = map.tiles[index];
                let revealed = map.revealed[index];
                match tile {
                    TileType::Wall => {
                        if revealed == Revealed::Encompassed {
                            // already done the encompassed mark, don't need to do it again
                            return;
                        }
                        // if seen, we're at the edge and don't need to push more into the queue
                        // othrwise, more into the queue!
                        if revealed != Revealed::Seen {
                            encompassed.push(index);
                            map.get_neighbors(index)
                                .iter()
                                .for_each(|pt| queue.push_back(*pt))
                        }
                    }
                    // wandered past the end the area without hitting a seen wall,
                    // so things are encompassed
                    TileType::Floor => return,
                    TileType::Exit => return,
                }
            }
        }
    }

    encompassed.iter().for_each(|idx| {
        map.revealed[*idx] = match map.revealed[*idx] {
            Revealed::NotSeen => Revealed::Encompassed,
            Revealed::Encompassed => Revealed::Encompassed,
            Revealed::Mapped => Revealed::Seen,
            Revealed::Seen => Revealed::Seen,
        }
    });
}
