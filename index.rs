#![allow(non_snake_case)]
#![feature(fs_try_exists)]

/// Reads the configuration file and marshals the JSON
/// into the [`api::Config`] struct.
///
/// [`api::Config`]: crate::api::config::Config;
pub fn read_config() -> api::Settings {
   let cfg: api::Settings = api::Settings {
      addr: "".to_string(),
      db: api::DbSettings { addr: "".to_string(), state: "".to_string() },
      a0: api::Auth0Settings {
         audience: "".to_string(),
         domain: "".to_string(),
         id: "".to_string(),
         issuer: "".to_string(),
         secret: "".to_string()
      }
   };

   return cfg;
}

pub mod assets;
pub mod api;

extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
