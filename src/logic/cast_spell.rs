use bevy::prelude::*;

use crate::{input::CurrentMousePosition, level::Point};

use super::Spell;

#[derive(Debug, Clone, Copy)]
pub struct SpellCast {
    pub position: Point,
    pub spell: Spell,
}

pub fn cast_spell(
    // should take inventory, only fireball for now
    input: Res<Input<KeyCode>>,
    mut writer: EventWriter<SpellCast>,
    hovered: Res<CurrentMousePosition>,
) {
    if let Some(hovered) = hovered.position() {
        if input.just_pressed(KeyCode::Key1) {
            writer.send(SpellCast {
                position: hovered,
                spell: Spell::Fireball,
            })
        } else if input.just_pressed(KeyCode::Key2) {
            writer.send(SpellCast {
                position: hovered,
                spell: Spell::Heal,
            })
        }
    }
}
