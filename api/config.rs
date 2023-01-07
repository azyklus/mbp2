#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
   #[serde(rename="addr")]
   pub Addr: String,
   #[serde(rename="db")]
   pub Db: DbSettings,
   #[serde(rename="a0")]
   pub Auth0: Auth0Settings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbSettings {
   #[serde(rename="addr")]
   pub Addr: String,
   #[serde(rename="state")]
   pub State: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0Settings {
   #[serde(rename="audience")]
   pub Audience: String,
   #[serde(rename="domain")]
   pub Domain: String,
   #[serde(rename="id")]
   pub Identifier: String,
   #[serde(rename="issuer")]
   pub Issuer: String,
   #[serde(rename="secret")]
   pub Secret: String,
}
