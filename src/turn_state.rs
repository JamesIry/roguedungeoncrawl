use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Resource)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
