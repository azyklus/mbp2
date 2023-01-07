//! My Blog/Portfolio app v2
//!
//! `mbp2` is my second iteration of my portfolio and blog hosting application.
//! In this iteration, I use Rust instead of Go and make use of the Rocket server
//! to handle routing and file serving while handing off the frontend of the app
//! to TypeScript and the React library.
#![allow(non_snake_case)]
#![feature(decl_macro)]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Load dotenv file.
   dotenv().ok();

   // Load configuration file.
   let mut cfg: Settings = DefaultSettings();
   if let Err(e) = ApplySettings(&mut cfg) {
      return Err(e.into());
   }

   // Load our configuration into a Figment.
   let fig: Figment = Figment::new()
      .merge(Toml::file("Rocket.toml").nested())
      .merge(Serialized::defaults(Config::default()))
      .merge(Env::prefixed("ROCKET_").global())
      .select(Profile::from_env_or("APP_PROFILE", "default"));

   let mut workDir: String = std::env::var("WORK_DIR").expect("could not load var");
   workDir.push_str("/web");

   // Our mounted base routes.
   let nfc: Catcher = Catcher::new(404, NotFoundHandler);
   //let index = Route::new(Get, "/", controllers::home::Index);

   // Our mounted API routes.
   let _config = controllers::api::ReadRocketConfig;

   // Set up our Rocket runtime.
   let rt = rocket::custom(fig)
      .attach(AdHoc::config::<Config>())
      .attach(Arc::new(Template::fairing()))
      .mount("/", FileServer::from(workDir))
      .mount("/", rocket::routes![
         Index
      ])
      .mount("/home", rocket::routes![
         home::Home
      ])
      .mount("/api", rocket::routes![
         api::Rocket,
         api::ReadRocketConfig,
      ])
      .register("/", vec![nfc]);

   // Check state and launch Rocket.
   assert!(rt.state::<String>().is_none());
   if let Err(e) = rt.launch().await {
      return Err(e.into());
   }

   return Ok(());
}

use {
   self::controllers::*,
   dotenv::dotenv,
   figment::{
      Figment,
      Profile,
      providers::{
         Env,
         Format,
         Serialized,
         Toml,
      }
   },
   mbp2::api::{ApplySettings, DefaultSettings, Settings},
   rocket::{
      fairing::AdHoc,
      fs::FileServer,
      Catcher,
      Config,
   },
   std::sync::Arc,
   tmpl::Template,
};

mod controllers;
mod models;

extern crate dotenv;
extern crate figment;
#[macro_use]
extern crate lazy_static;
extern crate mbp2;
extern crate rocket;
extern crate rocket_contrib as contrib;
#[macro_use]
extern crate rocket_dyn_templates as tmpl;
#[macro_use]
extern crate serde;
extern crate serde_json as json;
extern crate tokio;
