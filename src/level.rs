use bevy::prelude::*;

const TILE_SIZE: f32 = 32.0;
pub struct Level;

struct LevelSize {
    length: i32,
    height: i32,
}

impl Plugin for Level {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LevelSize {
            length: 10,
            height: 10,
        })
        .add_startup_system(setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
    size: Res<LevelSize>,
    //windows: Res<Windows>,
) {
    //let texture_tile = asset_server.load("tiles/floor.png");

    //let window = windows.get_primary().unwrap();

    let mut posx = -100.0;
    let mut posy = 200.0;

    let mut level_batch_sprite = Vec::new();
    for _ in 0..size.length {
        for _ in 0..size.height {
            level_batch_sprite.push(SpriteBundle {
                sprite: Sprite::new(Vec2::new(TILE_SIZE, TILE_SIZE)),
                //material: materials.add(texture_tile.clone().into()),
                material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                transform: Transform {
                    translation: Vec3::new(posx, posy, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
            posx += TILE_SIZE + 2.0;
        }
        posx = -100.0;
        posy -= TILE_SIZE + 2.0;
    }

    commands.spawn_batch(level_batch_sprite);
}
