#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
   pub addr: String,
   pub db: DbSettings,
   pub a0: Auth0Settings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbSettings {
   pub addr: String,
   pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth0Settings {
   pub audience: String,
   pub domain: String,
   pub id: String,
   pub issuer: String,
   pub secret: String,
}
