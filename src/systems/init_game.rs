use crate::{prelude::*, GAME_DATA_PATH};

pub fn init_game_system(mut commands: Commands, entities: Query<Entity>) {
    let gamedata = GameData::load(GAME_DATA_PATH);
    commands.insert_resource(gamedata);

    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
