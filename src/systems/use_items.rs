use crate::prelude::*;

pub fn use_items_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    activations: Query<(Entity, &ActivateItem)>,
    items: Query<
        (
            Entity,
            Option<&ProvidesHealing>,
            Option<&ProvidesDungeonMap>,
        ),
        Without<Weapon>,
    >,
    mut weapons: Query<(Entity, &Carried, &mut Weapon)>,
    mut healed: Query<(Entity, &mut Health)>,
) {
    activations
        .iter()
        .for_each(|(activate_entity, activation)| {
            let mut is_weapon = false;
            items
                .iter()
                .filter(|(item_entity, _, _)| *item_entity == activation.item)
                .for_each(|(_, optional_healing, optional_map)| {
                    if let Some(healing) = optional_healing {
                        //  healing_to_apply.push((activate.used_by, healing.amount));
                        for (_, mut health) in
                            healed.iter_mut().filter(|h| h.0 == activation.used_by)
                        {
                            health.current = i32::min(health.max, health.current + healing.amount);
                        }
                    }
                    if optional_map.is_some() {
                        map.revealed.iter_mut().for_each(|t| {
                            if *t == Revealed::NotSeen {
                                *t = Revealed::Mapped
                            }
                        });
                    }
                });

            weapons
                .iter_mut()
                .filter(|(weapon_entity, _, _)| *weapon_entity == activation.item)
                .for_each(|(_, _, mut weapon)| {
                    weapon.equipped = true;
                    is_weapon = true;
                });

            if is_weapon {
                weapons
                    .iter_mut()
                    .filter(|(weapon_entity, Carried(carried_by), _)| {
                        *weapon_entity != activation.item && *carried_by == activation.used_by
                    })
                    .for_each(|(_, _, mut weapon)| {
                        weapon.equipped = false;
                    });
            } else {
                commands.entity(activation.item).despawn();
            }
            commands.entity(activate_entity).despawn();
        });
}
