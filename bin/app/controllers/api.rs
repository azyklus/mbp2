#[rocket::get("/config")]
pub fn ReadConfig(rocket: &Config, app: &State<api::Settings>) -> String {
   return format!("{:#?}\n{:#?}", app, rocket);
}

use mbp2::api;
use rocket::{Config, State};
