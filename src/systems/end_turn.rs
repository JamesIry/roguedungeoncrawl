use crate::prelude::*;

pub fn end_turn_system(mut turn_state: ResMut<TurnState>) {
    *turn_state = match *turn_state {
        TurnState::InitGame => TurnState::InitLevel,
        TurnState::NextLevel => TurnState::InitLevel,
        TurnState::InitLevel => TurnState::AwaitingInput,
        TurnState::AwaitingInput => *turn_state,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::GameOver => *turn_state,
        TurnState::Victory => *turn_state,
    };
}
