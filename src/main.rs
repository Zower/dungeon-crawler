#![windows_subsystem = "windows"]

use ggez::graphics;
use ggez::input::keyboard;
use ggez::*;

mod player;
struct Game {
    player: player::Player,
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.player.icon.y -= self.player.speed;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            self.player.icon.x -= self.player.speed;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            self.player.icon.y += self.player.speed;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::D) {
            self.player.icon.x += self.player.speed;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 255, 255));
        let text = graphics::Text::new(format!("frames: {}", ggez::timer::fps(ctx)));
        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(5.0, 16.0)),
        )
        .expect("Failed to draw fps");

        let mesh = graphics::MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                self.player.icon,
                graphics::BLACK,
            )
            .build(ctx)?;

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(0.0, 0.0)),
        )
        .expect("Failed to draw");

        graphics::present(ctx).expect("Failed to present");

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: ggez::input::keyboard::KeyCode,
        _keymods: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            ggez::input::keyboard::KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        }
    }
}

fn main() {
    let state = &mut Game {
        player: player::Player {
            icon: graphics::Rect::new(0.0, 0.0, 100.0, 100.0),
            speed: 1.0,
        },
    };

    let ws = conf::WindowSetup {
        title: "My game".to_owned(),
        vsync: false,
        icon: "".to_owned(),
        srgb: true,
        samples: conf::NumSamples::Zero,
    };

    let wm = conf::WindowMode {
        width: 1200.0,
        height: 900.0,
        maximized: false,
        fullscreen_type: conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 600.0,
        max_width: 0.0,
        min_height: 400.0,
        max_height: 0.0,
        resizable: false,
    };

    let c = conf::Conf {
        window_setup: ws,
        window_mode: wm,
        backend: conf::Backend::default(),
        modules: conf::ModuleConf::default(),
    };
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("game", "Eirik Tobiassen")
        .conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
