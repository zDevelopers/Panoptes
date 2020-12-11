#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate mysql;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod area;
mod config;
mod output;

use figment::{Figment, providers::{Format, Toml, Env}};
use mysql::prelude::*;
use rocket::fairing::AdHoc;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;

use crate::area::Areas;
use crate::config::Config;
use crate::output::Player;


#[database("prism")]
struct PrismDatabase(mysql::Conn);


#[get("/")]
fn index() -> &'static str {
    "
    PANOPTÈS API DOCS

        GET /

            Displays this help.

        GET /players

            Returns a list of recently active players, according to Prism's records.
            Add a `filter` query parameter to filter by username.

        GET /ratio/<areas>/<uuids>

            Returns the ratio of the given player(s) in the given area(s).
            `areas` is either `all` or a comma-separated list of areas.
            `uuids` is a comma-separated list of UUIDs.
            Data will be aggregated as a whole from all areas and all players.

            Results are cached. Cache information will be available in the response. To force
            fresh results, add a `fresh` query parameter with any value.

        GET /areas

            Returns a list of available areas."
}

#[get("/areas")]
fn areas(areas: State<Areas>) -> Json<Areas>{
    Json(areas.inner().clone())
}


#[get("/players?<filter>")]
async fn players(filter: Option<String>, db: PrismDatabase) -> Result<Json<Vec<Player>>, Json<JsonValue>> {
    let players = db.run(|c: &mut mysql::Conn| {
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
        let player = format!("%{}%", filter.unwrap_or(String::from("")));
        c.exec_map(
            stmt, params! { player },
            |(name, uuid, _): (String, String, u64)| Player { name, uuid: Uuid::parse_str(uuid.as_str()).unwrap_or(Uuid::nil()) }
        )
    }).await;

    match players {
        Ok(players) => Ok(Json(players)),
        Err(_) => Err(Json(json!({ "error": "Unable to query players" })))
    }
}


#[launch]
fn rocket() -> rocket::Rocket {
    let figment = rocket::Config::figment()
        .merge(Toml::file("Panoptes.toml").nested())
        .merge(Toml::file(Env::var_or("PANOPTES_CONFIG", "../Panoptes.toml")).nested())
        .merge(Env::prefixed("PANOPTES_").global());

    rocket::custom(figment)
        .mount("/", routes![index, areas, players])
        .attach(AdHoc::on_attach("Areas Configuration", |rocket| async {
            let figment: &Figment = rocket.figment();
            let config: Config = figment.extract().expect("Missing or invalid areas configuration");
            let areas: Areas = config.into();

            Ok(rocket.manage(areas))
        }))
        .attach(PrismDatabase::fairing())
}
