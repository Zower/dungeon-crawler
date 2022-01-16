use bevy::{core::FixedTimestep, prelude::*};

use crate::{level::Point, logic::Direction, Level};

use rand::Rng;

use super::Health;

/// Just a small test, adds enemies ever so often and moves them around.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage(
            "fixed_enemy_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.36).with_label("movement"))
                .with_system(move_enemies),
        )
        .insert_resource(SpawnEnemyTimer(Timer::from_seconds(10f32, true)))
        .insert_resource(DamageEnemyTimer(Timer::from_seconds(10f32, true)))
        .add_system(spawn_enemies)
        .add_system(damage_enemies);
    }
}

#[derive(Component, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Enemy;

#[derive(Copy, Clone, Debug, Component)]
pub struct LastDirection(Direction);

pub struct SpawnEnemyTimer(Timer);

pub struct DamageEnemyTimer(Timer);

impl Enemy {
    fn next_direction(curr: Direction) -> Direction {
        match curr {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            _ => Direction::Up,
        }
    }
}

fn spawn_enemies(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    levels: Res<Level>,
    mut timer: ResMut<SpawnEnemyTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let map = levels.get_current();

        let pos_x = rand::thread_rng().gen_range(1..map.size.width - 1);
        let pos_y = rand::thread_rng().gen_range(1..map.size.height - 1);

        let pos = Vec2::new(pos_x as f32, pos_y as f32);

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("chars/blob.png"),
                transform: Transform {
                    translation: Vec3::from((pos, 1.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Point::new(pos_x, pos_y))
            .insert(LastDirection(Direction::Still))
            .insert(Health(100));
    }
}

fn damage_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies_query: Query<(Entity, &mut Health), With<Enemy>>,
    mut timer: ResMut<DamageEnemyTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (entity, mut enemy_hp) in enemies_query.iter_mut() {
            debug!(
                "Damaging entity: {entity:?}, with current hp: {enemy_hp:?}, new hp: {:?}",
                enemy_hp.0 - 50
            );

            enemy_hp.0 -= 50;
            if enemy_hp.0 <= 0 {
                debug!("Killing entity: {entity:?}");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn move_enemies(
    maps: Res<Level>,
    mut enemies_query: Query<(&mut LastDirection, &mut Transform, &mut Point), With<Enemy>>,
) {
    // Stuff that's happening here no good, me fix later :D
    for (mut last_direction, mut transform, mut point) in enemies_query.iter_mut() {
        let new_direction = Enemy::next_direction(last_direction.0);
        last_direction.0 = new_direction;
        let map = maps.get_current();
        let next_tile = map
            .get_neighbour(map.get_tile(*point).unwrap(), new_direction)
            .unwrap();

        let new_transform = next_tile.screen_position();

        transform.translation = Vec3::new(new_transform.x as f32, new_transform.y as f32, 1.0);

        *point = next_tile.position;
    }
}
