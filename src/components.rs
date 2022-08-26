pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType
}

enum Entities {
    Player,
    Monster
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Goblin;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Orc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ogre;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ettin;
