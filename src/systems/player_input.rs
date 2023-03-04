use std::collections::HashSet;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Name)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .map(|(entity, pos)| (*entity, *pos))
                    .next()
                    .unwrap();

                let mut items = <(Entity, &Item, &Point)>::query();
                items
                    .iter(ecs)
                    .filter(|(_, _, &item_pos)| item_pos == player_pos)
                    .for_each(|(item_entity, _, _)| {
                        commands.remove_component::<Point>(*item_entity);
                        commands.add_component(*item_entity, Carried(player));
                    });

                Point::zero()
            }
            _ => {
                if *key >= VirtualKeyCode::Key1 && *key <= VirtualKeyCode::Key9 {
                    use_item(
                        ((*key as i32) - (VirtualKeyCode::Key1 as i32)) as usize,
                        ecs,
                        commands,
                    )
                } else {
                    Point::zero()
                }
            }
        };

        if let Some((player, destination)) = players
            .iter(ecs)
            .map(|(entity, pos)| (entity, *pos + delta))
            .next()
        {
            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

            if delta.x != 0 || delta.y != 0 {
                let mut hit_something = false;
                enemies
                    .iter(ecs)
                    .filter(|(_, pos)| **pos == destination)
                    .for_each(|(enemy, _)| {
                        hit_something = true;

                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *player,
                                target: *enemy,
                            },
                        ));
                    });

                if !hit_something {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *player,
                            destination,
                        },
                    ));
                }
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let mut item_number = 0;
    let mut item_set: HashSet<String> = HashSet::new();
    <(Entity, &Player)>::query()
        .iter(ecs)
        .for_each(|(player, _)| {
            <(Entity, &Item, &Carried, &Name)>::query()
                .iter(ecs)
                .filter(|(_, _, carried, _)| carried.0 == *player)
                .for_each(|(item_entity, _, _, name)| {
                    if !item_set.contains(&name.0) {
                        item_set.insert(name.0.clone());
                        if item_number == n {
                            commands.push((
                                (),
                                ActivateItem {
                                    used_by: *player,
                                    item: *item_entity,
                                },
                            ));
                        }
                        item_number += 1;
                    }
                })
        });

    Point::zero()
}
