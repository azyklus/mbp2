#![allow(non_snake_case)]

/// Reads the configuration file and marshals the JSON
/// into the [`api::Config`] struct.
///
/// [`api::Config`]: crate::api::config::Config;
pub fn read_config() -> api::Settings {
   let cfg: api::Settings = api::Settings {
      Addr: "".to_string(),
      Db: api::DbSettings { Addr: "".to_string(), State: "".to_string() },
      A0: api::Auth0Settings {
         Audience: "".to_string(),
         Domain: "".to_string(),
         Id: "".to_string(),
         Issuer: "".to_string(),
         Secret: "".to_string()
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
extern crate serde_json;
