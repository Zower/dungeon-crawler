use std::{collections::HashMap, mem};

#[derive(Debug)]
pub struct ConvarStore {
    pub convars: HashMap<String, Var>,
}

impl ConvarStore {
    /// Initialize a variable into the store
    pub fn add(&mut self, key: &str, value: Var) -> Option<Var> {
        self.convars.insert(String::from(key), value)
    }

    pub fn get(&self, key: &str) -> Option<&Var> {
        self.convars.get(key)
    }

    /// Updates to the new value. None is returned if the key doesnt exist, or if the type is not the same as previous.
    pub fn set(&mut self, key: &str, new_value: Var) -> Option<&Var> {
        let cv = self.get_mut(key);
        match cv {
            Some(v) => {
                if mem::discriminant(v) == mem::discriminant(&new_value) {
                    *v = new_value;
                    Some(v)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Updates to the new value. None is returned if the key doesn't exist.
    pub fn change(&mut self, key: &str, new_value: Var) -> Option<&Var> {
        let cv = self.get_mut(key);
        match cv {
            Some(v) => {
                *v = new_value;
                Some(v)
            }
            _ => None,
        }
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut Var> {
        self.convars.get_mut(key)
    }
}

impl ConvarStore {
    pub fn new() -> ConvarStore {
        ConvarStore {
            convars: HashMap::new(),
        }
    }

    /// Attempts to parse a string into a String and Var
    pub fn parse(string: String) -> Result<(String, Var), NotParseableError> {
        let tokens: Vec<&str> = string.split(' ').collect();
        if tokens.len() >= 2 {
            let key = String::from(*tokens.get(0).unwrap());
            let value = String::from(*tokens.get(1).unwrap());

            if let Ok(toggleable) = value.parse::<i32>() {
                return Ok((key, Var::Toggleable(toggleable)));
            }

            if let Ok(settable) = value.parse::<f64>() {
                return Ok((key, Var::Settable(settable)));
            }

            return Ok((key, Var::String(value)));
        } else {
            return Err(NotParseableError(String::from(
                "the string doesn't contain at least a key and a value",
            )));
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Var {
    Toggleable(i32),
    Settable(f64),
    String(String),
}

#[derive(Debug)]
pub struct NotParseableError(String);

impl std::fmt::Display for NotParseableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "String is not parseable: {}", self.0)
    }
}

impl std::error::Error for NotParseableError {}
