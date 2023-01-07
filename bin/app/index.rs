//! My Blog/Portfolio app v2
//!
//! `mbp2` is my second iteration of my portfolio and blog hosting application.
//! In this iteration, I use Rust instead of Go and make use of the Rocket server
//! to handle routing and file serving while handing off the frontend of the app
//! to TypeScript and the React library.
#![allow(non_snake_case)]
#![feature(decl_macro, proc_macro_hygiene)]

lazy_static!{
   pub static ref HOME_ROUTES: Vec<Route> = rocket::routes![
      Index, home::Home,
   ];

   pub static ref API_ROUTES: Vec<Route> = rocket::routes![
      api::ReadRocketConfig,
      api::Rocket,
   ];
}

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
   let index: Route = (&HOME_ROUTES).get(0).unwrap().clone();
   let home: Route = (&HOME_ROUTES).get(1).unwrap().clone();

   // Our mounted API routes.
   let apiConfig: Route = (&API_ROUTES).get(0).unwrap().clone();
   let apiRocket: Route = (&API_ROUTES).get(1).unwrap().clone();

   // Set up our Rocket runtime.
   let rt = rocket::custom(fig)
      .attach(AdHoc::config::<Config>())
      .attach(Template::try_custom(|e| {
         models::Customise(&mut e.handlebars);
         Ok(())
      }))
      .mount("/", FileServer::from(workDir))
      .mount("/", vec![index])
      .mount("/api", vec![apiConfig, apiRocket])
      .mount("/home", vec![home])
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
      Route,
   },
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
#[macro_use]
extern crate rocket_dyn_templates as tmpl;
#[macro_use]
extern crate serde;
extern crate serde_json as json;
extern crate tokio;
