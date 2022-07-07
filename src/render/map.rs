use bevy::prelude::*;
use dungeon_crawler_derive::Convar;

use crate::{
    entity::Player,
    input::Convar,
    level::{FieldOfView, Point, TileComponent},
    logic::Cursor,
    Level,
};

#[derive(Debug, Default, Convar)]
pub struct GlobalVision(bool);

pub fn render_map(
    global: Res<GlobalVision>,
    mut level: ResMut<Level>,
    mut player_query: Query<(&Point, &Cursor, &FieldOfView), With<Player>>,
    mut map_sprites_query: Query<(&mut Sprite, &mut Visibility, &TileComponent)>,
) {
    let (player_pos, cursor, player_fov) = player_query.single_mut();
    let map = level.get_current_mut();

    let rect = cursor.circle(2);
    for (mut sprite, mut visibility, pos) in map_sprites_query.iter_mut() {
        if global.0 {
            visibility.is_visible = true;
            sprite.color = Color::WHITE;
        } else {
            if player_fov.tiles.contains(&pos.0) || pos.0 == *player_pos {
                visibility.is_visible = true;
                sprite.color = Color::WHITE;
            } else if map.get_tile(&pos.0).unwrap().revealed {
                sprite.color = Color::GRAY;
            } else {
                visibility.is_visible = false;
            }
        }

        if let Some(rect) = &rect {
            for tile in rect {
                if *tile == pos.0 && visibility.is_visible {
                    // visibility.is_visible = true;
                    sprite.color = Color::GREEN;

                    break;
                }
            }
        }
    }

    map.update_visibility(&player_fov.tiles);
}
