use std::collections::HashSet;

use crate::prelude::*;

pub fn player_input_system(
    mut commands: Commands,
    key: Res<KeyPress>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut players: Query<(Entity, &Position), (With<Player>, Without<Item>, Without<Enemy>)>,
    items: Query<
        (Entity, Option<&Position>, &DCName, Option<&Carried>),
        (With<Item>, Without<Player>, Without<Enemy>),
    >,
    enemies: Query<(Entity, &Position), (With<Enemy>, Without<Player>, Without<Item>)>,
) {
    if let Some(key) = key.0 {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_pos) = players.single_mut();

                items
                    .iter()
                    .filter(|(_, item_pos, _, _)| *item_pos == Some(player_pos))
                    .for_each(|(item_entity, _, _, _)| {
                        commands.entity(item_entity).remove::<Position>();
                        commands.entity(item_entity).insert(Carried(player));
                    });

                Point::zero()
            }
            _ => {
                let (player, _) = players.single_mut();
                if key >= VirtualKeyCode::Key1 && key <= VirtualKeyCode::Key9 {
                    use_item(
                        ((key as i32) - (VirtualKeyCode::Key1 as i32)) as usize,
                        &mut commands,
                        player,
                        items,
                    )
                } else {
                    Point::zero()
                }
            }
        };

        if let Some((player, destination)) = players
            .iter()
            .map(|(entity, pos)| (entity, pos.0 + delta))
            .next()
        {
            if delta.x != 0 || delta.y != 0 {
                let mut hit_something = false;
                enemies
                    .iter()
                    .filter(|(_, pos)| pos.0 == destination)
                    .for_each(|(enemy, _)| {
                        hit_something = true;

                        commands.spawn((
                            (),
                            WantsToAttack {
                                attacker: player,
                                target: enemy,
                            },
                        ));
                    });

                if !hit_something {
                    commands.entity(player).insert(WantsToMove { destination });
                }
            }
        }

        next_state.set(TurnState::PlayerTurn);
    }
}

fn use_item(
    n: usize,
    commands: &mut Commands,
    player: Entity,
    items: Query<
        (Entity, Option<&Position>, &DCName, Option<&Carried>),
        (With<Item>, Without<Player>, Without<Enemy>),
    >,
) -> Point {
    let mut item_number = 0;
    let mut item_set: HashSet<String> = HashSet::new();

    items
        .iter()
        .filter(|(_, _, _, carried)| carried.map(|c| c.0 == player).unwrap_or(false))
        .for_each(|(item_entity, _, name, _)| {
            if !item_set.contains(&name.0) {
                item_set.insert(name.0.clone());
                if item_number == n {
                    commands.spawn((
                        (),
                        ActivateItem {
                            used_by: player,
                            item: item_entity,
                        },
                    ));
                }
                item_number += 1;
            }
        });

    Point::zero()
}
