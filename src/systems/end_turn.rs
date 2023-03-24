use crate::prelude::*;

pub fn end_turn_system(
    current_state: Res<State<TurnState>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    let new_state = match current_state.0 {
        TurnState::InitGame => Some(TurnState::InitLevel),
        TurnState::NextLevel => Some(TurnState::InitLevel),
        TurnState::InitLevel => Some(TurnState::AwaitingInput),
        TurnState::AwaitingInput => None,
        TurnState::PlayerTurn => Some(TurnState::MonsterTurn),
        TurnState::MonsterTurn => Some(TurnState::AwaitingInput),
        TurnState::GameOver => None,
        TurnState::Victory => None,
    };

    if let Some(new_state) = new_state {
        next_state.set(new_state);
    }
}
