//! Console, press F1 to toggle
//! The console is currently not finished, e.g. movement is still registered even if the console is open
use bevy::{ecs::system::Resource, prelude::*};
use bevy_console::{
    reply, reply_failed, AddConsoleCommand, CommandArgs, CommandHelp, CommandInfo, CommandName,
    ConsoleCommand, ConsoleConfiguration, ConsoleOpen, PrintConsoleLine, ToggleConsoleKey,
};
use leafwing_input_manager::prelude::InputMap;
use std::{iter::empty, str::FromStr};
use strum::EnumString;

use crate::{components::Player, core::MovementAction};
// debatable that this should be in ui

/// The plugin representing the Console UI element
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleConfiguration {
            keys: vec![
                ToggleConsoleKey::KeyCode(KeyCode::Escape),
                ToggleConsoleKey::KeyCode(KeyCode::Grave),
                ToggleConsoleKey::KeyCode(KeyCode::F1),
            ],
            history_size: 100,
            scrollback_size: 2000,
            ..Default::default()
        })
        .insert_resource(ConsoleOpen { open: true })
        .add_plugin(bevy_console::ConsolePlugin)
        .add_startup_system(welcome);

        app.add_console_command::<BindCommand, _, _>(bind_command);
        app.add_console_command::<LogCommand, _, _>(log_command);
        app.add_console_command::<CloseCommand, _, _>(close_command);
    }
}
static WELCOME_MESSAGE: &str = r#"
Welcome to game.
This console is your pause screen. Press F1/esc/` to open/close the console.
Here is a list of available commands. Type help to see it again, or help <command> for information on a specific command. 
"#;

fn welcome(mut writer: EventWriter<PrintConsoleLine>, config: Res<ConsoleConfiguration>) {
    writer.send(PrintConsoleLine {
        line: WELCOME_MESSAGE.into(),
    });
    // reply!(help, "Available commands:");
    let longest_command_name = config
        .commands
        .keys()
        .map(|name| name.len())
        .max()
        .unwrap_or(0);
    for (name, cmd) in &config.commands {
        let mut line = format!("  {name}{}", " ".repeat(longest_command_name - name.len()));
        if let Some(CommandInfo {
            description: Some(description),
            ..
        }) = cmd
        {
            line.push_str(&format!(" - {description}"));
        }
        writer.send(PrintConsoleLine { line: line.into() });
    }
    writer.send(PrintConsoleLine { line: "".into() });
}

/// Rebinds a key
#[derive(ConsoleCommand)]
#[console_command(name = "bind")]
struct BindCommand {
    /// Key to press
    key: String,
    /// Action that is taken
    action: String,
}

fn bind_command(
    mut bind: ConsoleCommand<BindCommand>,
    mut input_query: Query<&mut InputMap<MovementAction>, With<Player>>,
) {
    if let Some(BindCommand { key, action }) = bind.take() {
        let action = match MovementAction::from_str(&action) {
            Ok(action) => action,
            Err(_) => {
                reply_failed!(bind, "No such action: '{action}'");
                return;
            }
        };

        let key: KeyCode = match CustomBindInput::from_str(&key) {
            Ok(key) => key.into(),
            Err(_) => {
                reply_failed!(bind, "No such key: '{key}'");
                return;
            }
        };

        let mut input = input_query.single_mut();
        let conflict_action = input.iter().find_map(|(inputs, action)| {
            if inputs.contains(&key.into()) {
                return Some(action);
            }
            None
        });
        if let Some(action) = conflict_action {
            input.remove(action, key);
        }

        input.insert(key, action);
        bind.ok();
    }
}

/// Prints given arguments to the console
#[derive(ConsoleCommand)]
#[console_command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i16>,
}

fn log_command(mut log: ConsoleCommand<LogCommand>) {
    if let Some(LogCommand { msg, num }) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }
    }
}

/// Closes console, unpausing the game
#[derive(ConsoleCommand)]
#[console_command(name = "close")]
struct CloseCommand;

fn close_command(mut close: ConsoleCommand<CloseCommand>, mut console_open: ResMut<ConsoleOpen>) {
    if close.take().is_some() {
        console_open.open = false;
    }
}

pub trait AddConvar {
    // fn add_command<C: CommandHelp + CommandName + 'static, Params>(
    //     &mut self,
    //     system: impl IntoSystemDescriptor<Params>,
    // ) -> &mut Self;

    fn add_convar<C: Resource + CommandHelp + CommandName + CommandArgs + 'static>(
        &mut self,
        init: C,
    ) -> &mut Self;

    fn init_convar<C: Default + Resource + CommandHelp + CommandName + CommandArgs + 'static>(
        &mut self,
    ) -> &mut Self {
        self.add_convar(C::default())
    }
}

impl AddConvar for App {
    // fn add_command<C: CommandHelp + CommandName + 'static, Params>(
    //     &mut self,
    //     system: impl IntoSystemDescriptor<Params>,
    // ) -> &mut Self {
    //     self.add_console_command::<C, _, _>(system);
    //     self
    // }

    fn add_convar<C: Resource + CommandHelp + CommandName + CommandArgs + 'static>(
        &mut self,
        init: C,
    ) -> &mut Self {
        self.add_console_command::<C, _, _>(convar_resource::<C>)
            .insert_resource(init);

        self
    }
}

fn convar_resource<C: Resource + CommandName + CommandArgs + CommandHelp>(
    mut command: ConsoleCommand<C>,
    mut res: ResMut<C>,
) {
    if let Some(t) = command.take() {
        *res = t;
    }
}

#[derive(Copy, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
// I dont want this either
enum CustomBindInput {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Snapshot,
    Scroll,
    Pause,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    Back,
    Return,
    Space,

    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    NumpadAdd,
    Apostrophe,
    Apps,
    Asterisk,
    Plus,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    NumpadDecimal,
    NumpadDivide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    NumpadMultiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    Oem102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    NumpadSubtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

impl From<CustomBindInput> for KeyCode {
    fn from(c: CustomBindInput) -> Self {
        match c {
            CustomBindInput::Key1 => KeyCode::Key1,
            CustomBindInput::Key2 => KeyCode::Key2,
            CustomBindInput::Key3 => KeyCode::Key3,
            CustomBindInput::Key4 => KeyCode::Key4,
            CustomBindInput::Key5 => KeyCode::Key5,
            CustomBindInput::Key6 => KeyCode::Key6,
            CustomBindInput::Key7 => KeyCode::Key7,
            CustomBindInput::Key8 => KeyCode::Key8,
            CustomBindInput::Key9 => KeyCode::Key9,
            CustomBindInput::Key0 => KeyCode::Key0,
            CustomBindInput::A => KeyCode::A,
            CustomBindInput::B => KeyCode::B,
            CustomBindInput::C => KeyCode::C,
            CustomBindInput::D => KeyCode::D,
            CustomBindInput::E => KeyCode::E,
            CustomBindInput::F => KeyCode::F,
            CustomBindInput::G => KeyCode::G,
            CustomBindInput::H => KeyCode::H,
            CustomBindInput::I => KeyCode::I,
            CustomBindInput::J => KeyCode::J,
            CustomBindInput::K => KeyCode::K,
            CustomBindInput::L => KeyCode::L,
            CustomBindInput::M => KeyCode::M,
            CustomBindInput::N => KeyCode::N,
            CustomBindInput::O => KeyCode::O,
            CustomBindInput::P => KeyCode::P,
            CustomBindInput::Q => KeyCode::Q,
            CustomBindInput::R => KeyCode::R,
            CustomBindInput::S => KeyCode::S,
            CustomBindInput::T => KeyCode::T,
            CustomBindInput::U => KeyCode::U,
            CustomBindInput::V => KeyCode::V,
            CustomBindInput::W => KeyCode::W,
            CustomBindInput::X => KeyCode::X,
            CustomBindInput::Y => KeyCode::Y,
            CustomBindInput::Z => KeyCode::Z,
            CustomBindInput::Escape => KeyCode::Escape,
            CustomBindInput::F1 => KeyCode::F1,
            CustomBindInput::F2 => KeyCode::F2,
            CustomBindInput::F3 => KeyCode::F3,
            CustomBindInput::F4 => KeyCode::F4,
            CustomBindInput::F5 => KeyCode::F5,
            CustomBindInput::F6 => KeyCode::F6,
            CustomBindInput::F7 => KeyCode::F7,
            CustomBindInput::F8 => KeyCode::F8,
            CustomBindInput::F9 => KeyCode::F9,
            CustomBindInput::F10 => KeyCode::F10,
            CustomBindInput::F11 => KeyCode::F11,
            CustomBindInput::F12 => KeyCode::F12,
            CustomBindInput::F13 => KeyCode::F13,
            CustomBindInput::F14 => KeyCode::F14,
            CustomBindInput::F15 => KeyCode::F15,
            CustomBindInput::F16 => KeyCode::F16,
            CustomBindInput::F17 => KeyCode::F17,
            CustomBindInput::F18 => KeyCode::F18,
            CustomBindInput::F19 => KeyCode::F19,
            CustomBindInput::F20 => KeyCode::F20,
            CustomBindInput::F21 => KeyCode::F21,
            CustomBindInput::F22 => KeyCode::F22,
            CustomBindInput::F23 => KeyCode::F23,
            CustomBindInput::F24 => KeyCode::F24,
            CustomBindInput::Snapshot => KeyCode::Snapshot,
            CustomBindInput::Scroll => KeyCode::Scroll,
            CustomBindInput::Pause => KeyCode::Pause,
            CustomBindInput::Insert => KeyCode::Insert,
            CustomBindInput::Home => KeyCode::Home,
            CustomBindInput::Delete => KeyCode::Delete,
            CustomBindInput::End => KeyCode::End,
            CustomBindInput::PageDown => KeyCode::PageDown,
            CustomBindInput::PageUp => KeyCode::PageUp,
            CustomBindInput::Left => KeyCode::Left,
            CustomBindInput::Up => KeyCode::Up,
            CustomBindInput::Right => KeyCode::Right,
            CustomBindInput::Down => KeyCode::Down,
            CustomBindInput::Back => KeyCode::Back,
            CustomBindInput::Return => KeyCode::Return,
            CustomBindInput::Space => KeyCode::Space,
            CustomBindInput::Compose => KeyCode::Compose,
            CustomBindInput::Caret => KeyCode::Caret,
            CustomBindInput::Numlock => KeyCode::Numlock,
            CustomBindInput::Numpad0 => KeyCode::Numpad0,
            CustomBindInput::Numpad1 => KeyCode::Numpad1,
            CustomBindInput::Numpad2 => KeyCode::Numpad2,
            CustomBindInput::Numpad3 => KeyCode::Numpad3,
            CustomBindInput::Numpad4 => KeyCode::Numpad4,
            CustomBindInput::Numpad5 => KeyCode::Numpad5,
            CustomBindInput::Numpad6 => KeyCode::Numpad6,
            CustomBindInput::Numpad7 => KeyCode::Numpad7,
            CustomBindInput::Numpad8 => KeyCode::Numpad8,
            CustomBindInput::Numpad9 => KeyCode::Numpad9,
            CustomBindInput::AbntC1 => KeyCode::AbntC1,
            CustomBindInput::AbntC2 => KeyCode::AbntC2,
            CustomBindInput::NumpadAdd => KeyCode::NumpadAdd,
            CustomBindInput::Apostrophe => KeyCode::Apostrophe,
            CustomBindInput::Apps => KeyCode::Apps,
            CustomBindInput::Asterisk => KeyCode::Asterisk,
            CustomBindInput::Plus => KeyCode::Plus,
            CustomBindInput::At => KeyCode::At,
            CustomBindInput::Ax => KeyCode::Ax,
            CustomBindInput::Backslash => KeyCode::Backslash,
            CustomBindInput::Calculator => KeyCode::Calculator,
            CustomBindInput::Capital => KeyCode::Capital,
            CustomBindInput::Colon => KeyCode::Colon,
            CustomBindInput::Comma => KeyCode::Comma,
            CustomBindInput::Convert => KeyCode::Convert,
            CustomBindInput::NumpadDecimal => KeyCode::NumpadDecimal,
            CustomBindInput::NumpadDivide => KeyCode::NumpadDivide,
            CustomBindInput::Equals => KeyCode::Equals,
            CustomBindInput::Grave => KeyCode::Grave,
            CustomBindInput::Kana => KeyCode::Kana,
            CustomBindInput::Kanji => KeyCode::Kanji,
            CustomBindInput::LAlt => KeyCode::LAlt,
            CustomBindInput::LBracket => KeyCode::LBracket,
            CustomBindInput::LControl => KeyCode::LControl,
            CustomBindInput::LShift => KeyCode::LShift,
            CustomBindInput::LWin => KeyCode::LWin,
            CustomBindInput::Mail => KeyCode::Mail,
            CustomBindInput::MediaSelect => KeyCode::MediaSelect,
            CustomBindInput::MediaStop => KeyCode::MediaStop,
            CustomBindInput::Minus => KeyCode::Minus,
            CustomBindInput::NumpadMultiply => KeyCode::NumpadMultiply,
            CustomBindInput::Mute => KeyCode::Mute,
            CustomBindInput::MyComputer => KeyCode::MyComputer,
            CustomBindInput::NavigateForward => KeyCode::NavigateForward,
            CustomBindInput::NavigateBackward => KeyCode::NavigateBackward,
            CustomBindInput::NextTrack => KeyCode::NextTrack,
            CustomBindInput::NoConvert => KeyCode::NoConvert,
            CustomBindInput::NumpadComma => KeyCode::NumpadComma,
            CustomBindInput::NumpadEnter => KeyCode::NumpadEnter,
            CustomBindInput::NumpadEquals => KeyCode::NumpadEquals,
            CustomBindInput::Oem102 => KeyCode::Oem102,
            CustomBindInput::Period => KeyCode::Period,
            CustomBindInput::PlayPause => KeyCode::PlayPause,
            CustomBindInput::Power => KeyCode::Power,
            CustomBindInput::PrevTrack => KeyCode::PrevTrack,
            CustomBindInput::RAlt => KeyCode::RAlt,
            CustomBindInput::RBracket => KeyCode::RBracket,
            CustomBindInput::RControl => KeyCode::RControl,
            CustomBindInput::RShift => KeyCode::RShift,
            CustomBindInput::RWin => KeyCode::RWin,
            CustomBindInput::Semicolon => KeyCode::Semicolon,
            CustomBindInput::Slash => KeyCode::Slash,
            CustomBindInput::Sleep => KeyCode::Slash,
            CustomBindInput::Stop => KeyCode::Stop,
            CustomBindInput::NumpadSubtract => KeyCode::NumpadSubtract,
            CustomBindInput::Sysrq => KeyCode::Sysrq,
            CustomBindInput::Tab => KeyCode::Tab,
            CustomBindInput::Underline => KeyCode::Underline,
            CustomBindInput::Unlabeled => KeyCode::Unlabeled,
            CustomBindInput::VolumeDown => KeyCode::VolumeDown,
            CustomBindInput::VolumeUp => KeyCode::VolumeUp,
            CustomBindInput::Wake => KeyCode::Wake,
            CustomBindInput::WebBack => KeyCode::WebBack,
            CustomBindInput::WebFavorites => KeyCode::WebFavorites,
            CustomBindInput::WebForward => KeyCode::WebForward,
            CustomBindInput::WebHome => KeyCode::WebHome,
            CustomBindInput::WebRefresh => KeyCode::WebRefresh,
            CustomBindInput::WebSearch => KeyCode::WebSearch,
            CustomBindInput::WebStop => KeyCode::WebStop,
            CustomBindInput::Yen => KeyCode::Yen,
            CustomBindInput::Copy => KeyCode::Copy,
            CustomBindInput::Paste => KeyCode::Paste,
            CustomBindInput::Cut => KeyCode::Cut,
        }
    }
}
