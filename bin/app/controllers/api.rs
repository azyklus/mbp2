#[rocket::get("/rocket_config")]
pub fn ReadRocketConfig(rocket: &Config, app: &State<Config>) -> String {
   return format!("{:#?}\n{:#?}", app, rocket);
}

#[rocket::get("/rocket")]
pub fn Rocket() -> String {
   return "🚀 server v0.5.0-rc.3".to_string();
}

#[rocket::get("/graphql")]
pub fn GraphiQL() -> RawHtml<String> {
   return RawHtml(
      GraphiQLSource::build()
         .endpoint("/api/graphiql")
         .finish()
   );
}

#[rocket::get("/graphql/playground")]
pub fn GraphqlPlayground() -> RawHtml<String> {
   let config = GraphQLPlaygroundConfig::new("/api/graphql/playground");
   return RawHtml(playground_source(config));
}

#[rocket::get("/graphql?<query..>")]
pub async fn GetGraphqlHandler(query: GraphQLQuery, schema: &State<GraphQLSchema>) -> GraphQLResponse {
   query.Execute(schema.inner()).await
}

#[rocket::post("/graphql", data="<request>", format="application/json", rank=1)]
pub async fn PostGraphqlHandler(request: GraphQLRequest, schema: &State<GraphQLSchema>) -> GraphQLResponse {
   request.Execute(schema.inner()).await
}

#[rocket::post("/graphql", data="<request>", format="multipart/form-data", rank=2)]
pub async fn PostGraphqlHandlerMultipart(request: GraphQLRequest, schema: &State<GraphQLSchema>) -> GraphQLResponse {
   request.Execute(schema.inner()).await
}

#[doc(hidden)]
pub fn ReadAppConfig() {}

use {
   crate::service::GraphQLSchema,
   async_graphql::http::{
      GraphiQLSource, GraphQLPlaygroundConfig,
      playground_source,
   },
   mbp2::api::{
      GraphQLRequest, GraphQLResponse, GraphQLQuery
   },
   rocket::{response::content::RawHtml, Config, State}
};
