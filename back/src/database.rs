use mysql::prelude::*;
use mysql::{Conn, Error};

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub uuid: Uuid
}

pub fn query_recent_players(c: &mut mysql::Conn, filter: String) -> Result<Vec<Player>, Error> {
    let stmt = c.prep("
        SELECT
            p.player AS name,
            HEX(p.player_uuid) AS uuid,
            (
               SELECT epoch
               FROM prism_data d
               WHERE d.player_id = p.player_id
               ORDER BY epoch DESC
               LIMIT 1
            ) AS last_action
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
