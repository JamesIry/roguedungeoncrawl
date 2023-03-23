use crate::prelude::*;

pub fn advance_level_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    other_entities: Query<(Entity, Option<&Carried>), Without<Player>>,
) {
    let (player_entity, mut player) = player_query.single_mut();
    player.map_level += 1;

    let not_carried = other_entities
        .iter()
        .filter(|(_entity, carried)| match carried {
            Some(Carried(carried_by)) => *carried_by != player_entity,
            None => true,
        });

    not_carried.for_each(|(entity, _)| commands.entity(entity).despawn());
}
