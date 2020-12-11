use std::collections::HashMap;
use serde::Serialize;

use crate::config::{Config, ConfigArea};

#[derive(Serialize, Debug, Clone)]
pub(crate) struct Areas {
    pub areas: HashMap<String, Area>
}

impl From<Config> for Areas {
    fn from(config: Config) -> Self {
        Self {
            areas: config.areas.into_iter().map(|(id, area)| (id.clone(), Area::from(id.clone(), area))).collect()
        }
    }
}

impl From<Areas> for Vec<Area> {
    fn from(areas: Areas) -> Self {
        areas.areas.iter().map(|(_, area)| area.clone()).collect()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Area {
    pub id: String,
    pub name: String,
    pub world: String,
    pub low_corner: Vec<i64>,
    pub high_corner: Vec<i64>
}

impl Area {
    pub fn from(id: String, config: ConfigArea) -> Self {
        Area {
            id,
            name: config.name,
            world: config.world.replace("'", "\\'"),
            low_corner: vec![
                config.pos1[0].min(config.pos2[0]),
                config.pos1[1].min(config.pos2[1]),
                config.pos1[2].min(config.pos2[2]),
            ],
            high_corner: vec![
                config.pos1[0].max(config.pos2[0]),
                config.pos1[1].max(config.pos2[1]),
                config.pos1[2].max(config.pos2[2]),
            ]
        }
    }

    pub fn as_sql(&self) -> String {
        format!(
            "w.world = '{}' AND d.x > {} AND d.y > {} AND d.z > {} AND d.x < {} AND d.y < {} AND d.z < {}",
            self.world,
            self.low_corner[0], self.low_corner[1], self.low_corner[2],
            self.high_corner[0], self.high_corner[1], self.high_corner[2]
        )
    }
}
