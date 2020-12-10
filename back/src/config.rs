use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub areas: HashMap<String, ConfigArea>,
}

#[derive(Deserialize)]
pub struct ConfigArea {
    pub name: String,
    pub world: String,
    pub pos1: Vec<i64>,
    pub pos2: Vec<i64>,
}
