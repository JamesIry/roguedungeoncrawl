use std::cmp::max;

use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let targets: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(message, wants_to_attack)| {
            (*message, wants_to_attack.attacker, wants_to_attack.target)
        })
        .collect();

    targets.iter().for_each(|(message, attacker, target)| {
        let is_player = ecs
            .entry_ref(*target)
            .iter()
            .flat_map(|entity| entity.get_component::<Player>())
            .next()
            .is_some();

        let base_damage: i32 = ecs
            .entry_ref(*attacker)
            .iter()
            .flat_map(|entry_ref| entry_ref.get_component::<Damage>())
            .map(|dmg| dmg.0)
            .sum();

        let weapon_damage: i32 = <(&Carried, &Damage, &Weapon)>::query()
            .iter(ecs)
            .filter(|(carried, _, weapon)| carried.0 == *attacker && weapon.equipped)
            .map(|(_, dmg, _)| dmg.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        if let Some(mut health) = ecs
            .entry_mut(*target)
            .iter_mut()
            .flat_map(|entity| entity.get_component_mut::<Health>())
            .next()
        {
            health.current = max(0, health.current - final_damage);
            if health.current < 1 && !is_player {
                commands.remove(*target);
            }
        }

        commands.remove(*message);
    });
}
