#![feature(proc_macro_hygiene, decl_macro)]

extern crate cached;
#[macro_use] extern crate mysql;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

mod area;
mod config;
mod database;
mod params;
mod locales;

use figment::{Figment, providers::{Env, Format, Serialized, Toml}};
use itertools::Itertools;
use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket::response::status::BadRequest;
use rocket::State;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::json::{Json, JsonValue};

use rocket::logger::PaintExt;

use crate::area::{Area, Areas};
use crate::config::{AreasConfig, CorsConfig, TranslationsConfig};
use crate::database::{Player, Ratios, query_recent_players, query_ratios};
use crate::params::{AreasIds, Uuids};
use crate::locales::{MinecraftLocales, Locale};
use rocket::yansi::Paint;
use std::sync::Arc;


type Result<T> = std::result::Result<T, BadRequest<Json<JsonValue>>>;


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

            Results are cached for one minute.

        GET /ratios?areas=<areas>&players=<players>&locale=<locale>

            Returns the ratio of the given player(s) in the given area(s).
            - `areas` is a comma-separated list of areas. If missing, all areas are searched.
            - `players` is a comma-separated list of UUIDs.
            - `locale` is the locale to use for the display names (e.g. “ja_jp” or “ru_ru”). If
               missing, the app's default locale will be used.

            Data will be aggregated as a whole from all areas and all players.
            Results are cached for ten minutes.

        GET /areas

            Returns a list of available areas."
}


#[get("/areas")]
fn areas(areas: State<Areas>) -> Json<Vec<Area>>{
    Json(areas.inner().clone().into())
}


#[get("/players?<filter>")]
async fn players(filter: Option<String>, db: PrismDatabase) -> Result<Json<Vec<Player>>> {
    match db.run(|c: &mut mysql::Conn| query_recent_players(c, filter.unwrap_or(String::from("")))).await {
        Ok(players) => Ok(Json(players)),
        Err(_) => Err(BadRequest(Some(Json(json!({ "error": "Unable to query players" })))))
    }
}


#[get("/ratios?<areas>&<players>")]
async fn ratios(areas: AreasIds, players: Uuids, areas_state: State<'_, Areas>, locale: Locale, db: PrismDatabase) -> Result<Json<Ratios>> {
    let areas: Vec<Area> = areas_state.filter(areas).areas.iter().map(|(_, a)| a.clone()).collect();
    match areas.len() {
        0 => Err(BadRequest(Some(Json(json!({ "error": "There are no areas matching your request." }))))),
        _ => match db.run(move |c: &mut mysql::Conn| query_ratios(c, areas, players, Arc::clone(&*locale))).await {
            Ok(ratios) => Ok(Json(ratios)),
            Err(_) => Err(BadRequest(Some(Json(json!({ "error": "Unable to query ratios" })))))
        }
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
        .mount("/", routes![index, areas, players, ratios])
        .attach(AdHoc::on_attach("Areas Configuration", |rocket| async {
            let figment: &Figment = rocket.figment();
            let config: AreasConfig = match figment.extract() {
                Ok(config) => config,
                Err(e) => {
                    rocket::config::pretty_print_error(e);
                    return Err(rocket);
                }
            };

            let areas: Areas = config.into();

            Ok(rocket.manage(areas))
        }))
        .attach(AdHoc::on_attach("Translations Configuration", |rocket| async {
            let figment: &Figment = rocket.figment();
            let config: TranslationsConfig = match figment.extract() {
                Ok(config) => config,
                Err(e) => {
                    rocket::config::pretty_print_error(e);
                    return Err(rocket);
                }
            };

            if let Some(config) = config.minecraft_translations {
                let locales: MinecraftLocales = config.into();

                println!("{}{}{}", Paint::blue(Paint::emoji("🌍 ")), Paint::magenta("Translations"), Paint::blue(":"));
                println!("    {} {} {}", Paint::default("=>").bold(), Paint::blue("loaded:"), Paint::default(locales.locales.keys().cloned().intersperse(String::from(", ")).collect::<String>()).bold());
                println!("    {} {} {}", Paint::default("=>").bold(), Paint::blue("default:"), Paint::default(&locales.default_locale).bold());

                return Ok(rocket.manage(locales));
            }

            Ok(rocket.manage(MinecraftLocales::empty()))
        }))
        .attach(AdHoc::config::<CorsConfig>())
        .attach(PrismDatabase::fairing())
        .attach(SpaceHelmet::default())
        .attach(AdHoc::on_response("CORS", |req, res| Box::pin(async move {
            let cors_config = req.guard::<rocket::State<'_, CorsConfig>>().await.expect("CorsConfig state not attached");
            res.set_header(Header::new("Access-Control-Allow-Origin", cors_config.cors.clone()));
        })))
}
