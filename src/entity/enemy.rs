use bevy::{core::FixedTimestep, prelude::*};

use crate::{
    level::{Point, TILE_SIZE},
    logic::Direction,
    Levels,
};

use rand::Rng;

use super::{Health, Position};

/// Just a small test, adds enemies ever so often and moves them around.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage(
            "fixed_enemy_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(0.36).with_label("movement"))
                .with_system(move_enemies.system()),
        )
        .insert_resource(SpawnEnemyTimer(Timer::from_seconds(10f32, true)))
        .insert_resource(DamageEnemyTimer(Timer::from_seconds(10f32, true)))
        .add_system(spawn_enemies.system())
        .add_system(damage_enemies.system());
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Enemy;

#[derive(Copy, Clone, Debug)]
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
    levels: Res<Levels>,
    mut timer: ResMut<SpawnEnemyTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("{:?}", "10 seconds elapsed");
        let level = levels.current();

        let pos_x = rand::thread_rng().gen_range(1..level.size.width - 1);
        let pos_y = rand::thread_rng().gen_range(1..level.size.height - 1);

        let pos = Position(Point { x: pos_x, y: pos_y });

        let texture_char = asset_server.load("chars/blob.png");

        println!("Spawning enemy at {:?}", pos.0);

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture_char.into()),
                transform: Transform {
                    translation: Vec3::new(
                        (pos.0.x * TILE_SIZE) as f32,
                        (pos.0.y * TILE_SIZE) as f32,
                        1.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(pos)
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
            println!(
                "Damaging entity: {:?}, with current hp: {:?}, new hp: {:?}",
                entity,
                enemy_hp.0,
                enemy_hp.0 - 50
            );

            enemy_hp.0 -= 50;
            if enemy_hp.0 <= 0 {
                println!("{:?}", "killing that entity.");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn move_enemies(
    levels: Res<Levels>,
    mut enemies_query: Query<(&mut LastDirection, &mut Transform, &mut Position), With<Enemy>>,
) {
    // Stuff that's happening here no good, me fix later :D
    for (mut last_direction, mut transform, mut position) in enemies_query.iter_mut() {
        let new_direction = Enemy::next_direction(last_direction.0);
        last_direction.0 = new_direction;
        let level = levels.current();
        let next_tile = level
            .get_neighbour(level.get_tile(position.0).unwrap(), new_direction)
            .unwrap();

        let new_transform = next_tile.screen_position();

        transform.translation = Vec3::new(new_transform.0.x as f32, new_transform.0.y as f32, 1.0);

        position.0 = next_tile.position;
    }
}
