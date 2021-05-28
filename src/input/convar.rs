//! Console variables

use bevy::prelude::*;

/// Plugin that adds the ConvarChange event to the system.
pub struct ConvarPlugin;

impl Plugin for ConvarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ConvarChange>();
    }
}
/// Convars are entirely controlled by events. There is no source of 'truth' for a variable.
/// if an event is missed for whatever reason, it will stay unupdated until a new event tells it to update.
pub struct ConvarChange(pub Convar);

/// All the possible convars, with different values
#[derive(Debug)]
pub enum Convar {
    UiFps(Toggled),
}

/// The toggled value, for a convar that can be either on or off.
#[derive(Debug, PartialEq, Eq)]
pub enum Toggled {
    On,
    Off,
}

impl Convar {
    /// Attempts to parse a string into a Convar
    pub fn parse(string: String) -> Result<Convar, NotParseableError> {
        let tokens: Vec<&str> = string.split(' ').collect();
        if tokens.len() >= 2 {
            let key = *tokens.get(0).unwrap();
            let value = String::from(*tokens.get(1).unwrap());

            match key {
                "ui_fps" => {
                    if value == "1" {
                        return Ok(Convar::UiFps(Toggled::On));
                    }
                    return Ok(Convar::UiFps(Toggled::Off));
                }
                _ => {
                    return Err(NotParseableError(String::from(
                        "No known convar with that key",
                    )))
                }
            }
        } else {
            return Err(NotParseableError(String::from(
                "the string doesn't contain at least a key and a value",
            )));
        }
    }
}

#[derive(Debug)]
pub struct NotParseableError(String);

impl std::fmt::Display for NotParseableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "String is not parseable: {}", self.0)
    }
}

impl std::error::Error for NotParseableError {}
