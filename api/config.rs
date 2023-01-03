#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
   pub Addr: String,
   pub Db: DbSettings,
   pub A0: Auth0Settings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbSettings {
   pub Addr: String,
   pub State: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0Settings {
   pub Audience: String,
   pub Domain: String,
   pub Id: String,
   pub Issuer: String,
   pub Secret: String,
}
