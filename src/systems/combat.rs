use std::cmp::max;

use crate::prelude::*;

pub fn combat_system(
    mut commands: Commands,
    attacks: Query<(Entity, &WantsToAttack)>,
    attackers: Query<(Entity, &Damage)>,
    targets: Query<(Entity, &Health, Option<&Player>)>,
    weapons: Query<(&Carried, &Damage, &Weapon)>,
) {
    attacks.iter().for_each(|(message, wants_to_attack)| {
        let attacker = wants_to_attack.attacker;
        let target = wants_to_attack.target;

        let (target_health, target_is_player) = targets
            .iter()
            .filter(|t| t.0 == target)
            .map(|(_, h, p)| (h, p.is_some()))
            .next()
            .unwrap();

        let attacker_base_damage = attackers
            .iter()
            .filter(|a| a.0 == attacker)
            .map(|(_, dmg)| dmg.0)
            .next()
            .unwrap();

        let weapon_damage: i32 = weapons
            .iter()
            .filter(|(carried, _, weapon)| carried.0 == wants_to_attack.attacker && weapon.equipped)
            .map(|(_, dmg, _)| dmg.0)
            .sum();

        let final_damage = attacker_base_damage + weapon_damage;

        let new_health = max(0, target_health.current - final_damage);
        commands.entity(target).insert(Health {
            current: new_health,
            max: target_health.max,
        });
        if new_health <= 0 && !target_is_player {
            commands.entity(target).despawn();
        }

        commands.entity(message).despawn();
    });
}
