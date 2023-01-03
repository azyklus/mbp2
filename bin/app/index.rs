//! My Portfolio app v2
//!
//! `mp2` is my second iteration of my portfolio and blog hosting application.
//! In this iteration, I use Rust instead of Go and make use of the Rocket server
//! to handle routing and file serving while handing off the frontend of the app
//! to TypeScript and the React library.
#![allow(non_snake_case)]
#![feature(decl_macro)]

#[rocket::get("/")]
fn readConfig(rocket: &Config, app: &State<api::Settings>) -> String {
   return format!("{:#?}\n{:#?}", app, rocket);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
   dotenv().ok();

   let mut workDir: String = std::env::var("WORK_DIR").expect("could not load var");
   workDir.push_str("/static");

   let nfc: Catcher = Catcher::new(404, NotFoundHandler);

   let index = Route::new(Get, "/", controllers::home::Index);

   let rt = rocket::build()
      .attach(AdHoc::config::<api::Settings>())
      .mount("/", vec![index])
      .register("/", vec![nfc]);

   assert!(rt.state::<String>().is_none());
   let rt = rt.ignite().await?;

   return Ok(());
}

mod controllers;
use controllers::{MainController, NotFoundHandler};

use dotenv::dotenv;
use mp2::api;
use rocket::{
   fairing::AdHoc,
   http::Method::*,
   Catcher,
   Config,
   Route,
   State,
};

extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate mp2;
extern crate rocket;
extern crate rocket_contrib;
extern crate tokio;
