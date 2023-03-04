use linked_hash_map::LinkedHashMap;

use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Weapon)]
pub fn hud(ecs: &SubWorld, #[resource] gamedata: &GameData, #[resource] map_info: &MapInfo) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let (player_health, message) = health_query
        .iter(ecs)
        .next()
        .map(|h| (h, "Explore the Dungeon. Cursor keys to move."))
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, message);
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

    let (player_entity, _) = <(Entity, &Player)>::query().iter(ecs).next().unwrap();

    draw_batch.print_color_right(
        Point::new(gamedata.text_display_width(), 1),
        &map_info.name,
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(Entity, &Item, &Name, &Carried)>::query();
    let mut y = 3;
    let mut item_map: LinkedHashMap<&String, (i32, bool)> = LinkedHashMap::new();

    item_query
        .iter(ecs)
        .filter(|(_, _, _, carried)| carried.0 == *player_entity)
        .for_each(|(item_entity, _, name, _)| {
            let name = &name.0;
            if let Ok(item_entity_ref) = ecs.entry_ref(*item_entity) {
                let weapon_equipped = if let Ok(weapon) = item_entity_ref.get_component::<Weapon>()
                {
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
