use crate::prelude::*;

use crate::{BLACK, ColorPair, Player, Point, to_cp437, WHITE};

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

// pub fn spawn_entity(ecs: &mut World, pos: Point, entity: Entity, color: ColorPair, glyph: char) {
//     ecs.push(
//         (
//             entity,
//             pos,
//             Render {
//                 color,
//                 glyph: to_cp437(glyph)
//             },
//         )
//     );
// }
