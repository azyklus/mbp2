#![doc = include_str!("../../readme.markdown")]
#![warn(missing_docs)]
#![allow(non_snake_case)]

lazy_static! {
   /// Root and /home routes.
   pub static ref HOME_ROUTES: Vec<Route> = rocket::routes![
      DistFiles, Index, home::Home,
   ];

   /// API level routes.
   pub static ref API_ROUTES: Vec<Route> = rocket::routes![
      api::ReadRocketConfig,
      api::Rocket,
      api::GraphiQL,
      api::GetGraphqlHandler,
      api::PostGraphqlHandler,
      api::PostGraphqlHandlerMultipart,
      api::GraphqlPlayground,
   ];
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Load dotenv file.
   let Some(vp) = dotenv().ok() else { unreachable!() };

   // Load configuration file.
   let mut cfg: Settings = DefaultSettings();
   if let Err(e) = ApplySettings(&mut cfg) {
      return Err(e.into());
   }

   // Load our configuration into a Figment.
   let fig: Figment = Figment::new()
      .merge(Toml::file("Rocket.toml").nested())
      .merge(Serialized::defaults(Config::default()))
      .merge(Env::prefixed("ROCKET_").global())
      .select(Profile::from_env_or("APP_PROFILE", "default"));

   let mut workDir: String = std::env::var("WORK_DIR").expect("could not load var");
   workDir.push_str("/web/dist");

   // Our mounted base routes.
   let nfc: Catcher = Catcher::new(404, NotFoundHandler);
   //let index = Route::new(Get, "/", controllers::home::Index);
   let dist: Route = (&HOME_ROUTES).get(0).unwrap().clone();
   let index: Route = (&HOME_ROUTES).get(1).unwrap().clone();
   let home: Route = (&HOME_ROUTES).get(2).unwrap().clone();

   // Our mounted API routes.
   let apiConfig: Route = (&API_ROUTES).get(0).unwrap().clone();
   let apiRocket: Route = (&API_ROUTES).get(1).unwrap().clone();
   let apiGraphql: Route = (&API_ROUTES).get(2).unwrap().clone();
   let apiGraphqlGet: Route = (&API_ROUTES).get(3).unwrap().clone();
   let apiGraphqlPost: Route = (&API_ROUTES).get(4).unwrap().clone();
   let apiGraphqlPostMultipart: Route = (&API_ROUTES).get(5).unwrap().clone();
   let apiGraphqlPlayground: Route = (&API_ROUTES).get(6).unwrap().clone();

   let dbClient: Client = DbConnCreate(cfg.Db).await.expect("db connection failed");
   // Ping the database to ensure a connection.
   if let Ok(_) = dbClient.database("otherskies").run_command(doc! { "ping": 1 }, None).await {
      println!("Database connection success");
   }

   let graphqlSchema: GraphQLSchema = GraphQLSchema::build(QueryRoot, MutationRoot, EmptySubscription)
      .data(dbClient.database("otherskies").clone())
      .finish();

   let rt = rocket::custom(fig)
      .attach(AdHoc::config::<Config>())
      .attach(Template::try_custom(|e| {
         let _ = models::Customise(&mut e.handlebars).unwrap();
         Ok(())
      }))
      .manage(dbClient.database("otherskies").clone())
      .manage(graphqlSchema)
      //.mount("/", vec![index])
      .mount("/", FileServer::from(workDir))
      .mount(
         "/api",
         vec![
            apiConfig,
            apiRocket,
            apiGraphql,
            apiGraphqlGet,
            apiGraphqlPost,
            apiGraphqlPostMultipart,
            apiGraphqlPlayground,
         ],
      )
      .mount("/home", vec![home]);
   //.register("/", vec![nfc]);

   // Check state and launch Rocket.
   assert!(rt.state::<String>().is_none());
   if let Err(e) = rt.launch().await {
      return Err(e.into());
   }

   return Ok(());
}

use {
   self::{
      controllers::*,
      service::{DbConnCreate, GraphQLSchema, MutationRoot, QueryRoot},
   },
   async_graphql::{EmptySubscription, Schema},
   bson::doc,
   dotenv::dotenv,
   figment::{
      providers::{Env, Format, Serialized, Toml},
      Figment, Profile,
   },
   mbp2::api::{ApplySettings, DefaultSettings, Settings},
   mongodb::Client,
   rocket::{fairing::AdHoc, fs::FileServer, Catcher, Config, Route},
   tmpl::Template,
};

mod controllers;
mod models;
mod service;

#[macro_use]
extern crate async_graphql;
extern crate bson;
extern crate chrono;
extern crate dotenv;
extern crate figment;
#[macro_use]
extern crate lazy_static;
extern crate mbp2;
extern crate mongodb;
extern crate rocket;
#[macro_use]
extern crate rocket_dyn_templates as tmpl;
#[macro_use]
extern crate serde;
extern crate serde_json as json;
extern crate tokio;
extern crate ulid;
