use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
#[read_component(Carried)]
#[write_component(Weapon)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    let mut unequip = Vec::new();

    let mut equip = Vec::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(activate_entity, activate)| {
            let activated_item_ref = ecs.entry_ref(activate.item);
            let mut remove = true;
            if let Ok(activated_item_ref) = activated_item_ref {
                if let Ok(healing) = activated_item_ref.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }
                if let Ok(_mapper) = activated_item_ref.get_component::<ProvidesDungeonMap>() {
                    map.revealed.iter_mut().for_each(|t| {
                        if *t == Revealed::NotSeen {
                            *t = Revealed::Mapped
                        }
                    });
                }
                if activated_item_ref.get_component::<Weapon>().is_ok() {
                    <(Entity, &Carried, &Weapon)>::query()
                        .iter(ecs)
                        .filter(|(weapon_entity, Carried(carried_by), weapon)| {
                            activate.item != **weapon_entity
                                && *carried_by == activate.used_by
                                && weapon.equipped
                        })
                        .for_each(|(weapon_entity, _, _)| {
                            unequip.push(*weapon_entity);
                        });

                    equip.push(activate.item);
                    remove = false;
                }
            }
            if remove {
                commands.remove(activate.item);
            }
            commands.remove(*activate_entity);
        });

    for (entity, heal_amount) in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(*entity) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal_amount);
            }
        }
    }

    set_equipped_status(ecs, &unequip, false);
    set_equipped_status(ecs, &equip, true);
}

fn set_equipped_status(ecs: &mut SubWorld, weapons: &[Entity], equpped_status: bool) {
    for weapon_entity in weapons.iter() {
        if let Ok(mut weapon_ref) = ecs.entry_mut(*weapon_entity) {
            if let Ok(weapon) = weapon_ref.get_component_mut::<Weapon>() {
                weapon.equipped = equpped_status;
            }
        }
    }
}
