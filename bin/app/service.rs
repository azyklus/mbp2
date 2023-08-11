pub async fn DbSrvCreate(config: DbSettings) -> FieldResult<Client> {
   let mut clientOptions = ClientOptions::parse(format!("mongodb+srv://{}:{}@cluster0.jlm4ztq.mongodb.net/?retryWrites={}&retryReads={}&localThresholdMS={}&w={}",
      config.Username, config.Password, config.RetryWrites, config.RetryReads, config.LocalThreshold, config.WriteConcern))
      .await.expect("failure to parse client options");

   let serverApi = ServerApi::builder().version(ServerApiVersion::V1).build();
   clientOptions.server_api = Some(serverApi);

   let dbUrl: String = env::var("MONGODB_URL").expect("url must be set");
   let client: Client = Client::with_options(clientOptions).expect("failed to build client");

   // Ping the database to ensure a connection.
   client.database("admin")
      .run_command(doc!{ "ping": 1 }, None)
      .await.unwrap();

   return Ok(client);
}

pub struct DbContext {
   pub Db: mongodb::Database
}

impl juniper::Context for DbContext {}

use {
   bson::doc,
   juniper::FieldResult,
   mbp2::api::*,
   mongodb::{Client, options::{ClientOptions, ServerApi, ServerApiVersion}},
   std::env,
};
