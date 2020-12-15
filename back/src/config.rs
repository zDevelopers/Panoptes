use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct AreasConfig {
    pub areas: HashMap<String, ConfigArea>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigArea {
    pub name: String,
    pub world: String,
    pub pos1: Vec<i64>,
    pub pos2: Vec<i64>,
}


#[derive(Serialize, Deserialize)]
pub struct CorsConfig {
    pub cors: String
}

impl Default for CorsConfig {
    fn default() -> CorsConfig {
        CorsConfig {
            cors: String::from("*")
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct TranslationsConfig {
    pub minecraft_translations: Option<TranslationsConfigInner>
}

#[derive(Serialize, Deserialize)]
pub struct TranslationsConfigInner {
    pub directory: PathBuf,
    pub default_locale: String
}
