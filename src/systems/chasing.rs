use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    if let Some((player_pos, _)) = player.iter(ecs).next() {
        let player_idx = map.point2d_to_index(*player_pos);

        let search_targets = vec![player_idx];
        let djikstra_map =
            DijkstraMap::new(map.width(), map.height(), &search_targets, map, 1024.0);

        movers.iter(ecs).for_each(|(entity, pos, _, fov)| {
            if !fov.visible_tiles.contains(player_pos) {
                return;
            }
            let idx = map.point2d_to_index(*pos);
            if let Some(destination) = DijkstraMap::find_lowest_exit(&djikstra_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
                };

                let mut attacked = false;
                positions
                    .iter(ecs)
                    .filter(|(_, target_pos, _)| **target_pos == destination)
                    .for_each(|(victim, _, _)| {
                        if ecs
                            .entry_ref(*victim)
                            .iter()
                            .flat_map(|entity| entity.get_component::<Player>())
                            .next()
                            .is_some()
                        {
                            commands.push((
                                (),
                                WantsToAttack {
                                    attacker: *entity,
                                    target: *victim,
                                },
                            ));
                        }
                        attacked = true;
                    });

                if !attacked {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination,
                        },
                    ));
                }
            }
        });
    }
}
