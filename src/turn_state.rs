use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Resource)]
pub enum TurnState {
    InitGame,
    NextLevel,
    InitLevel,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
}
