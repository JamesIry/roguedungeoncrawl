use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let mut new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::GameOver => *turn_state,
        TurnState::Victory => *turn_state,
        TurnState::NextLevel => *turn_state,
    };

    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());

    player_hp.iter(ecs).for_each(|(hp, player_pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver
        } else if map.tile_at(*player_pos) == TileType::Exit {
            new_state = TurnState::NextLevel;
        } else {
            let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
            amulet.iter(ecs).for_each(|amulet_pos| {
                if *player_pos == *amulet_pos {
                    new_state = TurnState::Victory;
                }
            });
        }
    });

    *turn_state = new_state;
}
