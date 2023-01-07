/// ApplySettings reads the 'config.json' file in the application root.
pub fn ApplySettings(cfg: &mut Settings) -> Result<bool, io::Error> {
   let filepath: &str = "config.json";
   let mut data: String = String::new();

   if !fs::metadata(filepath).is_ok() {
      let mut file: File = File::create(filepath).expect("couldn't create file");
      data = json::to_string_pretty(cfg).expect("couldn't serialize json");

      if let Err(e) = file.write_all(data.as_bytes()) {
         return Err(e);
      }

      // `Ok(false)` indicates file did not exist but was successfully created.
      return Ok(false);
   }

   let mut file: File = File::open(filepath).expect("couldn't open file");
   file.read_to_string(&mut data).expect("couldn't read config file");

   *cfg = json::from_str(data.as_str()).expect("could not deserialize json");

   // `Ok(true)` indicates a successful operation.
   return Ok(true);
}

#[inline]
pub fn DefaultSettings() -> Settings {
   return Settings{
      Addr: "".to_string(),
      Db: DbSettings{ Addr: "".to_string(), State: "".to_string() },
      Auth0: Auth0Settings{
         Audience: "".to_string(),
         Domain: "".to_string(),
         Identifier: "".to_string(),
         Issuer: "".to_string(),
         Secret: "".to_string()
      }
   };
}

pub use self::config::*;

use std::{
   io::{
      self,
      prelude::{Read,Write},
   },
   fs::{
      self,
      File,
   },
};

mod blog;
mod config;
