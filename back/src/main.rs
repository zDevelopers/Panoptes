#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate mysql;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod area;
mod config;
mod database;

use figment::{Figment, providers::{Env, Format, Serialized, Toml}};
use mysql::prelude::*;
use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket::State;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;

use crate::area::Areas;
use crate::config::{Config, CorsConfig};
use crate::database::{Player, query_recent_players};


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
    match db.run(|c: &mut mysql::Conn| query_recent_players(c, filter.unwrap_or(String::from("")))).await {
        Ok(players) => Ok(Json(players)),
        Err(_) => Err(Json(json!({ "error": "Unable to query players" })))
    }
}


#[launch]
fn rocket() -> rocket::Rocket {
    let figment = rocket::Config::figment()
        .merge(Serialized::defaults(CorsConfig::default()))
        .merge(Toml::file("Panoptes.toml").nested())
        .merge(Toml::file(Env::var_or("PANOPTES_CONFIG", "../Panoptes.toml")).nested())
        .merge(Env::prefixed("PANOPTES_").global());

    rocket::custom(figment)
        .mount("/", routes![index, areas, players])
        .attach(AdHoc::on_attach("Areas Configuration", |rocket| async {
            let figment: &Figment = rocket.figment();
            let config: Config = match figment.extract() {
                Ok(config) => config,
                Err(e) => {
                    rocket::config::pretty_print_error(e);
                    return Err(rocket);
                }
            };

            let areas: Areas = config.into();

            Ok(rocket.manage(areas))
        }))
        .attach(AdHoc::config::<CorsConfig>())
        .attach(PrismDatabase::fairing())
        .attach(SpaceHelmet::default())
        .attach(AdHoc::on_response("CORS header", |req, res| Box::pin(async move {
            let cors_config = req.guard::<rocket::State<'_, CorsConfig>>().await.expect("CorsConfig state not attached");
            res.set_header(Header::new("Access-Control-Allow-Origin", cors_config.cors.clone()));
        })))
}
