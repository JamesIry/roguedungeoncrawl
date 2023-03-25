use std::collections::HashSet;

pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Player {
    pub map_level: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct WantsToMove {
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Health {
    pub current: i32,

    pub max: i32,
}

#[derive(Clone, Debug, PartialEq, Component)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct AmuletOfYala;

#[derive(Clone, Debug, PartialEq, Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct ProvidesDungeonMap;

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Damage(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Weapon {
    pub equipped: bool,
}

#[derive(Clone, Debug, PartialEq, Resource)]
pub struct MapInfo {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Component, Resource)]
pub struct Position(pub Point);
