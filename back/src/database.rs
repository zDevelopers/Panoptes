use cached::proc_macro::cached;
use itertools::Itertools;
use mysql::prelude::*;
use mysql::{Conn, Error};
use serde::Serialize;
use uuid::Uuid;

use crate::area::{Area, cache_key_for_vec_areas};
use crate::params::Uuids;
use crate::locales::MinecraftLocale;
use std::sync::Arc;


#[derive(Serialize, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub uuid: Uuid
}

#[cached(
    size=128, time=60,
    result = true,
    key = "String",
    convert = r#"{ filter.clone() }"#
)]
pub fn query_recent_players(c: &mut Conn, filter: String) -> Result<Vec<Player>, Error> {
    let stmt = c.prep("
        SELECT
            p.player AS name,
            HEX(p.player_uuid) AS uuid,
            COALESCE((
               SELECT epoch
               FROM prism_data d
               WHERE d.player_id = p.player_id
               ORDER BY epoch DESC
               LIMIT 1
            ), 0) AS last_action
        FROM prism_players p
        WHERE
            p.player LIKE :player
            -- We try to exclude non-player entries
            AND NOT p.player LIKE '%:%'
            AND NOT p.player LIKE '% %'
            AND p.player NOT IN (
                'Piston', 'custom', 'zombie', 'skeleton', 'Lava',
                'dispenser', 'beehive', 'Environment', 'suffocation',
                'mount', 'spawner', 'fall', 'water', 'player', 'cramming',
                'breeding', 'creeper', 'guardian', 'drowning', 'drowned',
                'fire', 'piglin', 'unknown', 'default', 'tnt', 'witch',
                'villager', 'patrol', 'lightning', 'shulker', 'pillager',
                'dryout', 'egg', 'wither_skeleton', 'ocelot', 'fireball',
                'infection', 'player_unleash', 'holder_gone', 'blaze',
                'enderman', 'spectral_arrow', 'piglin_brute', 'hoglin',
                'strider', 'vex', 'vindicator', 'raid', 'wolf', 'stray',
                'husk', 'distance', 'turtle', 'wither', 'zombie_villager',
                'wandering_trader', 'arrow', 'cured', 'void', 'trap',
                'jockey', 'spider', 'snowman', 'starvation', 'sheep', 'cow',
                'trader_llama', 'fox', 'magma_cube', 'horse', 'projectile',
                'rabbit', 'parrot', 'donkey', 'cat', 'skeleton_horse',
                'chicken', 'zombified_piglin', 'evoker', 'ravager', 'ghast',
                'endermite'
            )
        ORDER BY last_action DESC
        LIMIT 20
    ")?;
    let player = format!("%{}%", filter);

    c.exec_map(
        stmt, params! { player },
        |(name, uuid, _): (String, String, u64)| Player {
            name,
            uuid: Uuid::parse_str(uuid.as_str()).unwrap_or(Uuid::nil())
        }
    )
}


#[derive(Serialize, Debug, Clone)]
pub struct Ratios {
    pub global: i64,
    pub detail: Vec<Ratio>
}

#[derive(Serialize, Debug, Clone)]
pub struct Ratio {
    pub id: String,
    pub display_name: String,
    pub ratio: i64
}

#[cached(
    size=128, time=600,
    result = true,
    key = "String",
    convert = r#"{ format!("{}{}{:?}", cache_key_for_vec_areas(&areas), players, (*locale).file) }"#
)]
pub fn query_ratios(c: &mut Conn, areas: Vec<Area>, players: Uuids, locale: Arc<MinecraftLocale>) -> Result<Ratios, Error> {
    let areas_where_clause: String = areas
        .iter()
        .map(|a| format!("({})", a.as_sql()))
        .intersperse(String::from(" OR "))
        .collect();
    let players_where_clause = players.as_sql();
    let sql = format!(
        "
        SELECT material, SUM(amount_diff) AS ratio
        FROM (
             SELECT
                    b.material AS material,
                    IF(action = 'item-insert', 1, -1) * JSON_EXTRACT(e.data, '$.amt') AS amount_diff
            FROM prism_data d
            LEFT JOIN prism_actions a ON a.action_id = d.action_id
            LEFT JOIN prism_players p ON p.player_id = d.player_id
            LEFT JOIN prism_worlds w ON w.world_id = d.world_id
            LEFT JOIN prism_id_map b ON b.block_id = d.block_id
            LEFT JOIN prism_data_extra e ON e.data_id = d.id
            WHERE a.action IN ('item-insert', 'item-remove')
                AND ({})
                AND ({})
        ) history
        GROUP BY material
        ORDER BY ratio;
        ",
        areas_where_clause,
        players_where_clause
    );

    let mut ratios: Vec<Ratio> = c.query_map(
        sql,
        |(material, ratio): (String, i64)| Ratio {
            id: if material.contains(":") { material.clone() } else { format!("minecraft:{}", material) },
            display_name: locale.translate(material),
            ratio
        }
    )?.into_iter().collect();

    ratios.sort_by_key(|ratio| -ratio.ratio);

    Ok(Ratios {
        global: ratios.iter().map(|ratio| ratio.ratio).sum(),
        detail: ratios
    })
}
