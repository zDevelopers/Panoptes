#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod area;
mod config;
mod output;

use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::State;
use rocket_contrib::databases::mysql;
use rocket_contrib::databases::mysql::prelude::*;
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
    ZCRAFT 1984 API DOCS

        GET /

            Displays this help.

        GET /players

            Returns a list of recently active players, according to Prism's records.
            Add a `filter` query parameter to filter by username.

        GET /ratio/<areas>/<uuids>

            Returns the ratio of the given player(s) in the given area(s).
            `areas` are either `all` or a comma-separated list of areas.
            `uuids` are a comma-separated list of UUIDs.
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
        c.query_map(
            "SELECT player AS name, HEX(player_uuid) as uuid FROM prism_players LIMIT 10",
            |(name, uuid): (String, String)| Player { name, uuid: Uuid::parse_str(uuid.as_str()).unwrap_or(Uuid::nil()) }
        )
    }).await;

    match players {
        Ok(players) => Ok(Json(players)),
        Err(_) => Err(Json(json!({ "error": "Unable to query players" })))
    }
}


#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, areas, players])
        .attach(AdHoc::on_attach("Areas Configuration", |rocket| async {
            let figment: &Figment = rocket.figment();
            let config: Config = figment.extract().expect("Missing or invalid areas configuration");
            let areas: Areas = config.into();

            Ok(rocket.manage(areas))
        }))
        .attach(PrismDatabase::fairing())
}
