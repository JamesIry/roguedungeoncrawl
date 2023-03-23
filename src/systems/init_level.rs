use crate::prelude::*;

pub fn init_level_system(
    mut commands: Commands,
    gamedata: Res<GameData>,
    mut player_opt: Query<(&Player, &mut Position, &mut FieldOfView)>,
) {
    let map_level = if let Ok((player, _, _)) = player_opt.get_single() {
        player.map_level
    } else {
        0
    };

    let mut rng = RandomNumberGenerator::new();

    let map_level_def = &gamedata.game_levels[map_level];

    let map_builder = map_level_def.get_builder(&gamedata);

    let BuiltMap {
        mut map,
        mut entity_spawns,
        player_start,
        amulet_start,
    } = map_builder.build(
        &mut rng,
        gamedata.map_width,
        gamedata.map_height,
        gamedata.num_monsters,
    );

    gamedata.apply_prefab(
        &mut map,
        &mut rng,
        player_start,
        amulet_start,
        &mut entity_spawns,
    );

    let theme = map_level_def.get_theme(&gamedata);
    commands.insert_resource(*theme);
    commands.insert_resource(MapInfo {
        name: map_level_def.name.clone(),
    });

    let mut camera = DCCamera::new(
        gamedata.tile_display_width(),
        gamedata.tile_display_height(),
        gamedata.map_width,
        gamedata.map_height,
    );
    camera.center_on_point(player_start);
    commands.insert_resource(camera);

    // spawn stuff
    if map_level >= gamedata.game_levels.len() - 1 {
        gamedata.spawn_amulet_of_yala(&mut commands, amulet_start);
    } else {
        map.set_tile(amulet_start, TileType::Exit);
    }

    // spawn a player if there isn't already one, otherwise move the player
    // to the new starting location
    if let Ok((_, mut player_pos, mut player_fov)) = player_opt.get_single_mut() {
        player_pos.0.x = player_start.x;
        player_pos.0.y = player_start.y;
        player_fov.is_dirty = true;
    } else {
        gamedata.spawn_player(&mut commands, player_start);
    }
    gamedata.spawn_entities(&mut commands, &mut rng, map_level, &entity_spawns);

    commands.insert_resource(map);
}
