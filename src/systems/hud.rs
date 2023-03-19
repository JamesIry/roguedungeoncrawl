use linked_hash_map::LinkedHashMap;

use crate::prelude::*;

pub fn hud_system(
    gamedata: Res<GameData>,
    map_info: Res<MapInfo>,
    player_query: Query<(Entity, &Health), With<Player>>,
    item_query: Query<(&DCName, &Carried, Option<&Weapon>), (With<Item>, Without<Player>)>,
) {
    let (player_entity, player_health) = player_query.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        gamedata.text_display_width(),
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.print_color_right(
        Point::new(gamedata.text_display_width(), 1),
        &map_info.name,
        ColorPair::new(YELLOW, BLACK),
    );

    let mut y = 3;
    let mut item_map: LinkedHashMap<&String, (i32, bool)> = LinkedHashMap::new();

    item_query
        .iter()
        .filter(|(_, carried, _)| carried.0 == player_entity)
        .for_each(|(name, _, optional_weapon)| {
            let name = &name.0;

            let weapon_equipped = if let Some(weapon) = optional_weapon {
                weapon.equipped
            } else {
                false
            };
            if item_map.contains_key(name) {
                let (count, equipped) = item_map.get_mut(name).unwrap();
                *count += 1;
                *equipped = *equipped || weapon_equipped;
            } else {
                item_map.insert(name, (1, weapon_equipped));
            }
        });

    item_map.iter().for_each(|(name, (count, equipped))| {
        let equip_message = if *equipped { " (E)" } else { "" };

        let message = if *count > 1 {
            format!("{} : {} ({}){}", y - 2, name, count, equip_message)
        } else {
            format!("{} : {}{}", y - 2, name, equip_message)
        };
        draw_batch.print(Point::new(3, y), message);
        y += 1;
    });

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items caried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
