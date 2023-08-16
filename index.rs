#![doc = include_str!("./readme.markdown")]
#![allow(non_snake_case)]

/// NewConfig creates a default [`Settings`] instance.
///
/// [`Settings`]: crate::api::config::Settings;
pub fn NewConfig() -> api::Settings {
   return api::DefaultSettings();
}

pub mod assets;
pub mod api;

#[macro_use]
extern crate async_graphql;
#[macro_use]
extern crate common_macros as macros;
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate tokio_util;
extern crate ulid;
