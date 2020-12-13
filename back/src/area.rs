use std::collections::HashMap;
use itertools::Itertools;
use serde::Serialize;

use crate::config::{Config, ConfigArea};


/// All areas declared into the configuration file are stored in this structure, made available
/// through a state.
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

/// From a vec of areas, computes a cache key to cache `/ratios` endpoint results.
/// For any identical set of areas, the output should be exactly the same; that's
/// why areas ids are sorted.
#[inline(always)]
pub fn cache_key_for_vec_areas(areas: &Vec<Area>) -> String {
    areas.iter()
        .map(|area| area.id.clone())
        .sorted()
        .intersperse(String::from(","))
        .collect()
}

/// Represents an area where players can access chests or other containers. Prism results
/// will be filtered in these areas only, because we don't want to get the ratio of players
/// from the whole maps.
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

    /// Generates an SQL WHERE clause to filter for this area, assuming that the `prism_worlds`
    /// table is aliased as `w`, and the `prism_data` one, as `d`.
    pub fn as_sql(&self) -> String {
        format!(
            "w.world = '{}' AND d.x > {} AND d.y > {} AND d.z > {} AND d.x < {} AND d.y < {} AND d.z < {}",
            self.world,
            self.low_corner[0], self.low_corner[1], self.low_corner[2],
            self.high_corner[0], self.high_corner[1], self.high_corner[2]
        )
    }
}
