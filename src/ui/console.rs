use bevy::prelude::*;

use crate::input::ConvarStore;

/// The plugin representing the Console UI element
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update_text.system())
            .add_system(toggle_console.system());
    }
}

// Component held by the TextBundle to identify the right text.
struct ConsoleText;

/// System that checks if the user pressed F2, and toggles the visibility accordingly.
fn toggle_console(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visible, With<ConsoleText>>,
) {
    if let Ok(mut visible) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::F1) {
            visible.is_visible = !visible.is_visible
        }
    } else {
        warn!("ConsoleText UI element not found");
    }
}

/// System that types what the user is writing into the second part of the console text, and attempts to set the ConVar when enter is pressed
fn update_text(
    mut store: ResMut<ConvarStore>,
    // Checking for literal keyboard keys, using for Enter, Backspace, etc. Trying to type * with this would just yield Lshift Key8
    key_pressed: Res<Input<KeyCode>>,
    // Checking for characters, used to read into the text, allows for modifiers etc.
    mut char_inputs: EventReader<ReceivedCharacter>,
    mut query: Query<(&mut Text, &mut Visible), With<ConsoleText>>,
) {
    if let Ok((mut text, mut visible)) = query.single_mut() {
        // Check if its time to update the FPS
        if visible.is_visible {
            let prompt = &mut text.sections[1].value;

            for pressed_key in key_pressed.get_just_pressed() {
                match pressed_key {
                    KeyCode::Return => {
                        let (key, value) = ConvarStore::parse(prompt.clone()).unwrap();
                        store.set(key.as_str(), value);
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
    } else {
        warn!("FPSText UI element not found")
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
            visible: Visible {
                is_transparent: false,
                is_visible: false,
            },
            ..Default::default()
        })
        .insert(ConsoleText);
}
