use crate::prelude::*;

pub fn init_game_system(ecs: &mut World) {
    let gamedata = ecs.resource::<GameData>().to_owned();

    ecs.clear_all();

    ecs.insert_resource(gamedata);
    ecs.insert_resource(TurnState::InitGame);
}
