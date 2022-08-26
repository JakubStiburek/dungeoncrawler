use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, pos: Point, monster: Monster) {
    let glyph = to_cp437(monster.glyph);
    ecs.push(
        (
            monster,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            }
        )
    );
}
