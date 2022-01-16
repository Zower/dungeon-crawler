//! Console, press F1 to toggle
//! The console is currently not finished, e.g. movement is still registered even if the console is open
use bevy::prelude::*;

use crate::input::{Convar, ConvarChange};

/// The plugin representing the Console UI element
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(update_text)
            .add_system(toggle_console);
    }
}

// Component held by the TextBundle to identify the right text.
#[derive(Debug, Component)]
struct ConsoleText;

/// System that checks if the user pressed F1, and toggles the visibility accordingly.
fn toggle_console(
    keyboard_input: Res<Input<KeyCode>>,
    mut console_text_query: Query<&mut Visibility, With<ConsoleText>>,
) {
    let mut visible = console_text_query.single_mut();
    if keyboard_input.just_pressed(KeyCode::F1) {
        visible.is_visible = !visible.is_visible
    }
}

/// System that types what the user is writing into the second part of the console text, and attempts to set the ConVar when enter is pressed
fn update_text(
    // Checking for literal keyboard keys, using for Enter, Backspace, etc. Trying to type * with this would just yield Lshift Key8
    key_pressed: Res<Input<KeyCode>>,
    mut convar_changed: EventWriter<ConvarChange>,
    // Checking for characters, used to read into the text, allows for modifiers etc.
    mut char_inputs: EventReader<ReceivedCharacter>,
    mut console_text_query: Query<(&mut Text, &mut Visibility), With<ConsoleText>>,
) {
    let (mut text, mut visible) = console_text_query.single_mut();
    // Check if we're reading input right now
    if visible.is_visible {
        let prompt = &mut text.sections[1].value;

        for pressed_key in key_pressed.get_just_pressed() {
            match pressed_key {
                KeyCode::Return => {
                    if let Ok(new_value) = Convar::parse(prompt.clone()) {
                        convar_changed.send(ConvarChange(new_value));
                    }
                    prompt.clear();
                    visible.is_visible = false;
                }

                KeyCode::Escape => {
                    prompt.clear();
                    visible.is_visible = false;
                }
                KeyCode::Back => {
                    prompt.pop();
                }
                _ => {
                    for pressed_char in char_inputs.iter() {
                        *prompt = format!("{}{}", prompt, pressed_char.char);
                    }
                }
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Command: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::GRAY,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(ConsoleText);
}
