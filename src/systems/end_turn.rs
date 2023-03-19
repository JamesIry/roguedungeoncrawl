use crate::prelude::*;

pub fn end_turn_system(
    mut turn_state: ResMut<TurnState>,
    map: Res<Map>,
    player: Query<(&Health, &Position), With<Player>>,
    amulet: Query<&Position, With<AmuletOfYala>>,
) {
    let mut new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::GameOver => *turn_state,
        TurnState::Victory => *turn_state,
        TurnState::NextLevel => *turn_state,
    };

    player.iter().for_each(|(hp, player_pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver
        } else if map.tile_at(player_pos.0) == TileType::Exit {
            new_state = TurnState::NextLevel;
        } else {
            amulet.iter().for_each(|amulet_pos| {
                if *player_pos == *amulet_pos {
                    new_state = TurnState::Victory;
                }
            });
        }
    });

    *turn_state = new_state;
}
