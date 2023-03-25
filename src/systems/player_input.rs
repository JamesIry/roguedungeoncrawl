use std::collections::HashSet;

use crate::prelude::*;

pub fn player_input_system(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<TurnState>>,
    players: Query<(Entity, &Position), (With<Player>, Without<Item>, Without<Enemy>)>,
    items: Query<
        (Entity, Option<&Position>, &DCName, Option<&Carried>),
        (With<Item>, Without<Player>, Without<Enemy>),
    >,
    enemies: Query<(Entity, &Position), (With<Enemy>, Without<Player>, Without<Item>)>,
) {
    if key.any_pressed([
        KeyCode::Left,
        KeyCode::Right,
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::G,
        KeyCode::Space,
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ]) {
        let (player, player_pos) = players.single();
        let delta = if key.pressed(KeyCode::Left) {
            Point::new(-1, 0)
        } else if key.pressed(KeyCode::Right) {
            Point::new(1, 0)
        } else if key.pressed(KeyCode::Up) {
            Point::new(0, -1)
        } else if key.pressed(KeyCode::Down) {
            Point::new(0, 1)
        } else if key.pressed(KeyCode::G) {
            items
                .iter()
                .filter(|(_, item_pos, _, _)| *item_pos == Some(player_pos))
                .for_each(|(item_entity, _, _, _)| {
                    commands.entity(item_entity).remove::<Position>();
                    commands.entity(item_entity).insert(Carried(player));
                });

            Point::zero()
        } else if key.pressed(KeyCode::Key1) {
            use_item(1, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key2) {
            use_item(2, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key3) {
            use_item(3, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key4) {
            use_item(4, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key5) {
            use_item(5, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key6) {
            use_item(6, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key7) {
            use_item(7, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key8) {
            use_item(8, &mut commands, player, items)
        } else if key.pressed(KeyCode::Key9) {
            use_item(9, &mut commands, player, items)
        } else {
            Point::zero()
        };

        let destination = player_pos.0 + delta;
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
    let mut item_number = 1;
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
