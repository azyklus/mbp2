/// ApplySettings reads the 'config.json' file in the application root.
pub fn ApplySettings(cfg: &mut Settings) -> Result<(), io::Error> {
   let filepath: &str = "config.json";
   if let Err(e) = fs::try_exists(filepath) {
      return Err(e);
   }

   let file: File = File::create(filepath).expect("couldn't create file");
   let json = json::to_string_pretty(cfg).expect("couldn't serialize json");

   if let Err(e) = fs::write(filepath, json) {
      return Err(e);
   }

   return Ok(());
}

#[inline]
pub fn DefaultSettings() -> Settings {
   return Settings{
      addr: "".to_string(),
      db: DbSettings { addr: "".to_string(), state: "".to_string() },
      a0: Auth0Settings {
         audience: "".to_string(),
         domain: "".to_string(),
         id: "".to_string(),
         issuer: "".to_string(),
         secret: "".to_string()
      }
   };
}

pub use self::config::*;
use std::{
   io,
   fs::{
      self,
      File,
   },
};

mod config;
