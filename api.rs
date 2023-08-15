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

#[inline(always)]
pub fn DefaultSettings() -> Settings {
   return Settings{
      Addr: String::new(),
      MaxSocketPayloadSize: 0,
      SocketHostOverride: None,
      Client: DefaultClientConfig(),
      Db: DbSettings{
         Addr: String::new(),
         State: String::new(),
         WriteConcern: String::new(),
         Username: String::new(),
         Password: String::new(),
         RetryReads: false,
         RetryWrites: false,
         LocalThreshold: 15,
      },
      Auth0: Auth0Settings{
         Audience: String::new(),
         Domain: String::new(),
         Identifier: String::new(),
         Issuer: String::new(),
         Secret: String::new(),
      }
   };
}

/// DefaultClientConfig creates a default [`ClientConfig`] instance.
///
/// [`ClientConfig`]: self::config::ClientConfig
#[inline(always)]
pub fn DefaultClientConfig() -> ClientConfig {
   return ClientConfig{
      Name: String::new(),
      Summary: String::new(),
      Tags: vec![],
      NSFW: false,
      Logo: String::new(),
      Version: String::new(),
      AppearanceVariables: hash_map!{},
      CustomStyles: String::new(),
      SocialHandles: vec![],
      Federation: Federation{
         Enabled: true,
         Account: String::new(),
         FollowerCount: 0,
      },
      Notifications: Notifications{
         Browser: Browser{
            Enabled: false,
            PublicKey: String::new(),
         },
      },
   };
}

pub use self::blog::*;
pub use self::config::*;
pub use self::graphql::*;

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
mod graphql;
