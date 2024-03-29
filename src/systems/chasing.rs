use crate::prelude::*;

pub fn chasing_system(
    mut map: ResMut<Map>,
    gamedata: Res<GameData>,
    mut commands: Commands,
    movers: Query<(Entity, &Position, &ChasingPlayer, &FieldOfView)>,
    positions: Query<(Entity, &Position, &Health, Option<&Player>)>,
    player: Query<&Position, With<Player>>,
) {
    if let Ok(player_pos) = player.get_single() {
        let dijkstra_map = map.dijkstra_map(player_pos.0, gamedata.max_monster_visibility);

        movers.iter().for_each(|(entity, pos, _, fov)| {
            if !fov.visible_tiles.contains(&player_pos.0) {
                return;
            }
            let idx = map.point_to_index(pos.0);
            if let Some(destination) = dijkstra_map.find_lowest_exit(idx, map.as_ref()) {
                let distance = pos.0.pythagorean_distance(player_pos.0);
                let destination = if distance > 1.2 {
                    map.index_to_point(destination)
                } else {
                    player_pos.0
                };

                let mut attacked = false;
                positions
                    .iter()
                    .filter(|(_, target_pos, _, _)| target_pos.0 == destination)
                    .for_each(|(victim, _, _, optional_player)| {
                        if optional_player.is_some() {
                            commands.spawn(WantsToAttack {
                                attacker: entity,
                                target: victim,
                            });
                            attacked = true;
                        }
                    });

                if !attacked {
                    commands.entity(entity).insert(WantsToMove { destination });
                }
            }
        });
    }
}
