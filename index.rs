#![allow(non_snake_case)]
//#![feature(fs_try_exists)]
// The above feature decl breaks compilation, and I have yet to determine why.
// I will uncomment the decl when I fix the problem.
// -Az

/// NewConfig creates a default [`Settings`] instance.
///
/// [`Settings`]: crate::api::config::Settings;
pub fn NewConfig() -> api::Settings {
   return api::DefaultSettings();
}

pub mod assets;
pub mod api;

#[macro_use]
extern crate common_macros as macros;
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate ulid;
