pub async fn DbSrvConnect() -> FieldResult<Client> {
   let dbUrl: String = env::var("MONGODB_URL").expect("url must be set");
   let client: Client = Client::with_uri_str(dbUrl).await.unwrap();

   return Ok(client);
}

pub struct DbContext {
   pub Db: mongodb::Database
}

impl juniper::Context for DbContext {}

use {
   juniper::FieldResult,
   mbp2::api::*,
   mongodb::Client,
   std::env,
};
