#[derive(Debug, Deserialize, Serialize)]
pub struct ClientConfig {
   #[serde(rename = "name")]
   pub Name: String,
   #[serde(rename = "summary")]
   pub Summary: String,
   #[serde(rename = "tags")]
   pub Tags: Vec<String>,
   #[serde(rename = "nsfw")]
   pub NSFW: bool,
   #[serde(rename = "logo")]
   pub Logo: String,
   #[serde(rename = "version")]
   pub Version: String,
   #[serde(rename = "appearanceVariables")]
   pub AppearanceVariables: HashMap<String, String>,
   #[serde(rename = "customStyles")]
   pub CustomStyles: String,
   #[serde(rename = "socialHandles")]
   pub SocialHandles: Vec<SocialHandle>,
   #[serde(rename = "federation")]
   pub Federation: Federation,
   #[serde(rename = "notifications")]
   pub Notifications: Notifications,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Authorisation {
   #[serde(rename = "indieAuthEnabled")]
   pub IndieAuthEnabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Federation {
   #[serde(rename = "enabled")]
   pub Enabled: bool,
   #[serde(rename = "account")]
   pub Account: String,
   #[serde(rename = "followerCount")]
   pub FollowerCount: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Notifications {
   #[serde(rename = "browser")]
   pub Browser: Browser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Browser {
   #[serde(rename = "enabled")]
   pub Enabled: bool,
   #[serde(rename = "publicKey")]
   pub PublicKey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SocialHandle {
   #[serde(rename = "platform")]
   pub Platform: String,
   #[serde(rename = "url")]
   pub Url: String,
   #[serde(rename = "icon")]
   pub Icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
   #[serde(rename = "addr")]
   pub Addr: String,
   #[serde(rename = "maxSocketPayloadSize")]
   pub MaxSocketPayloadSize: usize,
   #[serde(rename = "socketHostOverride")]
   pub SocketHostOverride: Option<String>,
   #[serde(rename = "client")]
   pub Client: ClientConfig,
   #[serde(rename = "db")]
   pub Db: DbSettings,
   #[serde(rename = "a0")]
   pub Auth0: Auth0Settings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbSettings {
   #[serde(rename = "addr")]
   pub Addr: String,
   #[serde(rename = "state")]
   pub State: String,
   #[serde(rename = "w")]
   pub WriteConcern: String,
   #[serde(rename = "username")]
   pub Username: String,
   #[serde(rename = "password")]
   pub Password: String,
   #[serde(rename = "retryReads")]
   pub RetryReads: bool,
   #[serde(rename = "retryWrites")]
   pub RetryWrites: bool,
   #[serde(rename = "localThreshold")]
   pub LocalThreshold: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0Settings {
   #[serde(rename = "audience")]
   pub Audience: String,
   #[serde(rename = "domain")]
   pub Domain: String,
   #[serde(rename = "id")]
   pub Identifier: String,
   #[serde(rename = "issuer")]
   pub Issuer: String,
   #[serde(rename = "secret")]
   pub Secret: String,
}

use std::collections::HashMap;
