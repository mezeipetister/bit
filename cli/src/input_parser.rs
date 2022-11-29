use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{Error, Map, Value};

pub fn try_parse<T>(i: &str) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de> + DeserializeOwned,
{
    let p = parse_cmd(i).unwrap();
    serde_json::from_value(Value::from(p))
}

enum ParserMode {
    None,
    Key,
    Value,
}

impl ParserMode {
    // fn is_key(&self) -> bool {
    //     match self {
    //         Self::Key => true,
    //         _ => false,
    //     }
    // }
    // fn is_value(&self) -> bool {
    //     match self {
    //         Self::Value => true,
    //         _ => false,
    //     }
    // }
    fn flip(&mut self) {
        match self {
            ParserMode::None => (),
            ParserMode::Key => *self = ParserMode::Value,
            ParserMode::Value => *self = ParserMode::Key,
        }
    }
    fn flip_to_key(&mut self) {
        *self = ParserMode::Key;
    }
    // fn flip_to_value(&mut self) {
    //     *self = ParserMode::Value;
    // }
}

fn add_k_v(data: &mut Map<String, Value>, k: &mut Option<String>, v: &mut Option<String>) {
    data.insert(
        k.take().unwrap_or_default(),
        Value::from(v.take().unwrap_or_default()),
    );
}

pub fn parse_cmd(raw_cmd: &str) -> Result<Map<String, Value>, String> {
    let mut result = Map::new();
    let mut multi_world: bool = false;
    let mut mode: ParserMode = ParserMode::None;
    let mut key: Option<String> = None;
    let mut value: Option<String> = None;
    for (_pos, ch) in raw_cmd.chars().enumerate() {
        match ch {
            '"' => match multi_world {
                true => {
                    multi_world = false;
                    continue;
                }
                false => {
                    multi_world = true;
                    continue;
                }
            },
            ' ' => match multi_world {
                true => value.as_mut().unwrap().push(ch),
                false => match mode {
                    ParserMode::None => continue,
                    ParserMode::Key => mode.flip(),
                    ParserMode::Value => {
                        add_k_v(&mut result, &mut key, &mut value);
                        mode.flip_to_key();
                    }
                },
            },
            x => match mode {
                ParserMode::None => match x.is_uppercase() {
                    true => {
                        mode.flip_to_key();
                        match &mut key {
                            Some(k) => k.push(ch),
                            None => key = Some(ch.to_string()),
                        }
                    }
                    false => return Err(format!("Key '{}' error. KEY mut be uppercase", ch)),
                },
                ParserMode::Key => match x.is_uppercase() {
                    true => match &mut key {
                        Some(k) => k.push(ch),
                        None => key = Some(ch.to_string()),
                    },
                    false => return Err(format!("Key '{}' error. KEY mut be uppercase", ch)),
                },
                ParserMode::Value => match &mut value {
                    Some(v) => v.push(ch),
                    None => value = Some(ch.to_string()),
                },
            },
        }
    }
    if key.is_some() && value.is_some() {
        add_k_v(&mut result, &mut key, &mut value);
    }
    Ok(result.iter().fold(Map::new(), |mut r, i| {
        r.insert(i.0.to_lowercase(), i.1.to_owned());
        r
    }))
}
