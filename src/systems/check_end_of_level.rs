use crate::prelude::*;

pub fn check_end_of_level_system(
    mut next_state: ResMut<NextState<TurnState>>,
    map: Res<Map>,
    player: Query<(&Health, &Position), With<Player>>,
    amulet: Query<&Position, With<AmuletOfYala>>,
) {
    player.iter().for_each(|(hp, player_pos)| {
        if hp.current < 1 {
            next_state.set(TurnState::GameOver)
        } else if map.tile_at(player_pos.0) == TileType::Exit {
            next_state.set(TurnState::NextLevel);
        } else {
            amulet.iter().for_each(|amulet_pos| {
                if *player_pos == *amulet_pos {
                    next_state.set(TurnState::Victory);
                }
            });
        }
    });
}
