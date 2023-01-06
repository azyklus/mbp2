//! My Blog/Portfolio app v2
//!
//! `mbp2` is my second iteration of my portfolio and blog hosting application.
//! In this iteration, I use Rust instead of Go and make use of the Rocket server
//! to handle routing and file serving while handing off the frontend of the app
//! to TypeScript and the React library.
#![allow(non_snake_case)]
#![feature(decl_macro)]

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
   // Load dotenv file.
   dotenv().ok();

   let cfg: api::Settings = api::DefaultSettings();

   // Load our configuration into a Figment.
   let fig: Figment = Figment::new()
      .merge(Serialized::defaults(Config::default()))
      .merge(Json::file("config.json").nested())
      .merge(Env::prefixed("MBP2_").global())
      .select(Profile::from_env_or("APP_PROFILE", "default"));

   let mut workDir: String = std::env::var("WORK_DIR").expect("could not load var");
   workDir.push_str("/web");

   // Our mounted base routes.
   let nfc: Catcher = Catcher::new(404, NotFoundHandler);
   let index = Route::new(Get, "/", controllers::home::Index);

   // Our mounted API routes.
   let _config = controllers::api::ReadConfig;

   // Set up our Rocket runtime.
   let rt = rocket::custom(fig)
      .attach(AdHoc::config::<api::Settings>())
      .mount("/", vec![index])
      .mount("/api", rocket::routes![controllers::api::ReadConfig])
      .register("/", vec![nfc]);

   // Check state and launch Rocket.
   assert!(rt.state::<String>().is_none());
   if let Err(e) = rt.ignite().await {
      return Err(e);
   }

   return Ok(());
}

use {
   controllers::{MainController, NotFoundHandler},
   dotenv::dotenv,
   figment::{
      Figment,
      Profile,
      providers::{
         Format,
         Json,
         Serialized,
         Env
      }
   },
   mbp2::api,
   rocket::{
      fairing::AdHoc,
      http::Method::*,
      Catcher,
      Config,
      Route,
   },
};

mod controllers;

extern crate dotenv;
extern crate figment;
#[macro_use]
extern crate lazy_static;
extern crate mbp2;
extern crate rocket;
extern crate rocket_contrib;
extern crate tokio;
