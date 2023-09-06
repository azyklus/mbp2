#![doc = include_str!("./readme.markdown")]
#![allow(non_snake_case)]

/// NewConfig creates a default [`Settings`] instance.
///
/// [`Settings`]: crate::api::config::Settings;
pub fn NewConfig() -> api::Settings {
   return api::DefaultSettings();
}

pub mod api;
pub mod assets;
pub mod crypto;
pub mod models;
pub mod service;

#[macro_use]
extern crate async_graphql;
extern crate cbc;
#[macro_use]
extern crate common_macros as macros;
extern crate rand;
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate tokio_util;
extern crate ulid;
