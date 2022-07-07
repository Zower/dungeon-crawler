use crate::entity::Health;
use crate::level::Point;
use bevy::app::Events;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;

// use super::cast_spell::SpellCast;
use super::cast_spell::{self, SpellCast};
use super::Cursor;

#[derive(Debug, Clone, Copy)]
pub enum Spell {
    Fireball,
    Heal,
}

impl Spell {
    fn take_action(&self, mut entity: EntityMut) {
        match self {
            Spell::Fireball => {
                if let Some(mut hp) = entity.get_mut::<Health>() {
                    hp.0 -= 10;
                }
            }
            Spell::Heal => {
                if let Some(mut hp) = entity.get_mut::<Health>() {
                    hp.0 += 10;
                }
            }
        }
    }
}

fn recieved_spell(world: &mut World) {
    let entities = world
        .query::<(Entity, &Point)>()
        .iter(&world)
        .map(|(e, p)| (e, *p))
        .collect::<Vec<(Entity, Point)>>();

    let spells_sent = world.get_resource::<Events<SpellCast>>().unwrap();

    let casts = spells_sent
        .get_reader()
        .iter(&spells_sent)
        .map(|s| *s)
        .collect::<Vec<_>>();

    for spell in casts {
        // TODO: Same r everywhere
        let circle = Cursor::draw_circle(&spell.position, 2)
            .into_iter()
            .collect::<Vec<_>>();

        if true {
            let mut entities_effected = vec![];
            for (entity, pos) in entities.iter() {
                if circle.iter().any(|t| *t == *pos) {
                    entities_effected.push(*entity);
                }
            }

            for entity in entities_effected {
                spell.spell.take_action(world.entity_mut(entity));
            }
        }
    }
}

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(recieved_spell.exclusive_system())
            .add_system(cast_spell::cast_spell)
            .add_event::<SpellCast>();
    }
}
