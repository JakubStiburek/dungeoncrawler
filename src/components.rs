pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType
}

pub enum Monsters {
    Goblin,
    Orc,
    Ogre,
    Ettin
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

pub struct Monster  {
    pub kind: Monsters,
    pub glyph: char,
}

impl Monster {
    pub fn new(kind: Monsters) -> Self {
        let glyph = match kind {
            Monsters::Goblin => 'g',
            Monsters::Orc => 'o',
            Monsters::Ogre => 'O',
            Monsters::Ettin => 'E',
        };
        Self {
            kind,
            glyph
        }
    }
}
