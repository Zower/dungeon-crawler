//! Console variables

use std::str::FromStr;

use bevy::prelude::*;

/// Plugin that adds the ConvarChange event to the system.
pub struct ConvarPlugin;

impl Plugin for ConvarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConvarTextSubmit>();
    }
}

pub trait Convar {
    type Item: IntoConvar;

    fn change(&mut self, item: Self::Item);
    fn command_name(&self) -> &'static str;
}

pub trait IntoConvar: Sized {
    fn into_convar(s: &str) -> Option<Self>;
}

impl<T> IntoConvar for T
where
    T: FromStr + NotConvarManual,
{
    fn into_convar(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}

fn process_convar_text_change<T: 'static + Convar + Send + Sync>(
    mut res: ResMut<T>,
    mut convar_text: EventReader<ConvarTextSubmit>,
) {
    for text in convar_text.iter() {
        let mut split = text.0.split(" ");
        if split.next() == Some(res.command_name()) {
            if let Some(next) = split.next() {
                let value = T::Item::into_convar(next);
                match value {
                    Some(value) => res.change(value),
                    None => info!("Wrong convar!"),
                }
            }
        }
    }
}

pub struct ConvarTextSubmit(pub String);

pub trait AddConvar {
    fn add_convar_default<T: 'static + Convar + Default + Send + Sync>(&mut self) -> &mut Self;
    fn add_convar<T: 'static + Convar + Send + Sync>(&mut self, t: T) -> &mut Self;
}

impl AddConvar for App {
    fn add_convar_default<T: 'static + Convar + Default + Send + Sync>(&mut self) -> &mut Self {
        self.insert_resource(T::default())
            .add_system(process_convar_text_change::<T>);
        self
    }

    fn add_convar<T: 'static + Convar + Send + Sync>(&mut self, t: T) -> &mut Self {
        self.insert_resource(t)
            .add_system(process_convar_text_change::<T>);
        self
    }
}

pub trait NotConvarManual {}

impl NotConvarManual for i32 {}
impl NotConvarManual for f32 {}
impl NotConvarManual for String {}
impl NotConvarManual for &str {}

impl IntoConvar for bool {
    fn into_convar(s: &str) -> Option<Self> {
        match Self::from_str(s) {
            Ok(val) => Some(val),
            _ => match s.parse::<i32>() {
                Ok(val) if val == 1 => Some(true),
                Ok(_) => Some(false),
                _ => None,
            },
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
