#[rocket::get("/rocket_config")]
pub fn ReadRocketConfig(rocket: &Config, app: &State<Config>) -> String {
   return format!("{:#?}\n{:#?}", app, rocket);
}

#[rocket::get("/api/rocket")]
pub fn Rocket() -> String {
   return "🚀 server v0.5.0-rc.2".to_string();
}

#[doc(hidden)]
pub fn ReadAppConfig() {}

use rocket::{Config, State};
