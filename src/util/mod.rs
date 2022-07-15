//! Utilties.
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bevy::{
    log::Level,
    math::Vec2,
    prelude::{Plugin, SystemStage},
};
use bevy_ecs_tilemap::TilePos;

mod consts;
mod debug;
mod queries;

pub use consts::*;
pub use debug::*;
use iyes_loopless::prelude::FixedTimestepStage;
pub use queries::*;
use tracing_subscriber::{
    filter::LevelFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    EnvFilter, Layer, Registry,
};

use self::{resources::LogWriter, systems::map_logs};

pub fn tile_from_trans(translation: &Vec2) -> TilePos {
    TilePos(
        (translation.x / TILE_SIZE) as u32,
        (translation.y / TILE_SIZE) as u32,
    )
}

pub fn trans_from_tile(pos: &TilePos) -> Vec2 {
    Vec2::new(
        (pos.0 as f32 * TILE_SIZE) + HALF_TILE_SIZE,
        (pos.1 as f32 * TILE_SIZE) + HALF_TILE_SIZE,
    )
}

pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let log_writer = LogWriter {
            buffer: Arc::new(Mutex::new(Vec::new())),
        };

        let cloned = log_writer.clone();

        let console_log = tracing_subscriber::fmt::layer()
            .pretty()
            .compact()
            .with_ansi(false)
            .with_line_number(cfg!(debug_assertions))
            .with_target(cfg!(debug_assertions))
            .with_file(cfg!(debug_assertions))
            .without_time()
            .with_writer(cloned)
            .with_filter(
                EnvFilter::try_new(format!(
                    "{},wgpu=error,bevy_ecs_tilemap=error",
                    if cfg!(debug_assertions) {
                        Level::INFO
                    } else {
                        Level::WARN
                    }
                ))
                .unwrap(),
            );

        let subscriber = Registry::default().with(console_log);

        let error_log = tracing_subscriber::fmt::layer().with_filter(LevelFilter::ERROR);

        subscriber.with(error_log).init();

        app.insert_resource(log_writer).add_stage(
            "log",
            FixedTimestepStage::new(Duration::from_millis(100))
                .with_stage(SystemStage::parallel().with_system(map_logs)),
        );
    }
}

pub mod systems {
    use bevy::{prelude::*, render::render_resource::TextureUsages};
    use bevy_console::{
        egui::{text::LayoutJob, Color32, FontId, TextFormat},
        PrintConsoleLine,
    };

    use super::resources::LogWriter;

    // Required by ecs_tilemap. Idk why TBH.
    pub fn set_texture_filters_to_nearest(
        mut texture_events: EventReader<AssetEvent<Image>>,
        mut textures: ResMut<Assets<Image>>,
    ) {
        // quick and dirty, run this for all textures anytime a texture is created.
        for event in texture_events.iter() {
            match event {
                AssetEvent::Created { handle } => {
                    if let Some(mut texture) = textures.get_mut(handle) {
                        texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                            | TextureUsages::COPY_SRC
                            | TextureUsages::COPY_DST;
                    }
                }
                _ => (),
            }
        }
    }

    pub(super) fn map_logs(t: Res<LogWriter>, mut writer: EventWriter<PrintConsoleLine>) {
        // for line in t.buffer.lock().unwrap().drain(..) {
        let mut lock = t.buffer.lock().unwrap();
        // let mut temp = vec![];

        let temp = lock.drain(..);

        // for x in 0..lock.len().saturating_sub(1) {
        //     println!("Test {x}");
        //     temp.push(lock.remove(x));
        // }
        writer.send_batch(temp.map(|line| {
            let line = line.trim_start();
            let (level, rest) = line.split_at(line.find(" ").unwrap());

            let mut space = ": ".to_string();

            space.push_str(rest);

            let mut layout = LayoutJob::default();
            layout.append(
                level,
                0.,
                TextFormat {
                    color: match level {
                        "INFO" => Color32::LIGHT_BLUE,
                        "WARN" => Color32::YELLOW,
                        "ERROR" => Color32::LIGHT_RED,
                        _ => panic!("Unknown log level"),
                    },
                    font_id: FontId {
                        size: 13.,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            layout.append(&space, 0., Default::default());

            PrintConsoleLine {
                line: layout.into(),
            }
        }));
        // }
    }
}

pub mod resources {
    use std::{
        io::Write,
        sync::{Arc, Mutex},
    };

    use tracing_subscriber::fmt::MakeWriter;

    pub(super) struct VecWriter<'a>(&'a Mutex<Vec<String>>);

    impl<'a> Write for VecWriter<'a> {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            self.0
                .lock()
                .unwrap()
                .push(String::from_utf8_lossy(_buf).into());
            Ok(_buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub(super) struct LogWriter {
        pub buffer: Arc<Mutex<Vec<String>>>,
    }

    impl<'a> MakeWriter<'a> for LogWriter {
        type Writer = VecWriter<'a>;
        fn make_writer(&'a self) -> Self::Writer {
            VecWriter(&self.buffer)
        }
    }
}
