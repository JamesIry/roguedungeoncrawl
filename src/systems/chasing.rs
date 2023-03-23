use crate::prelude::*;

pub fn chasing_system(
    map: Res<Map>,
    mut commands: Commands,
    movers: Query<(Entity, &Position, &ChasingPlayer, &FieldOfView)>,
    positions: Query<(Entity, &Position, &Health, Option<&Player>)>,
    player: Query<&Position, With<Player>>,
) {
    if let Ok(player_pos) = player.get_single() {
        let player_idx = map.point2d_to_index(player_pos.0);

        let search_targets = vec![player_idx];
        let djikstra_map = DijkstraMap::new(
            map.width(),
            map.height(),
            &search_targets,
            map.as_ref(),
            1024.0,
        );

        movers.iter().for_each(|(entity, pos, _, fov)| {
            if !fov.visible_tiles.contains(&player_pos.0) {
                return;
            }
            let idx = map.point2d_to_index(pos.0);
            if let Some(destination) =
                DijkstraMap::find_lowest_exit(&djikstra_map, idx, map.as_ref())
            {
                let distance = DistanceAlg::Pythagoras.distance2d(pos.0, player_pos.0);
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
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
