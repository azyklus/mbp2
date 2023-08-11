pub type GraphqlSchema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<DbContext>>;

#[rocket::get("/rocket_config")]
pub fn ReadRocketConfig(rocket: &Config, app: &State<Config>) -> String {
   return format!("{:#?}\n{:#?}", app, rocket);
}

#[rocket::get("/rocket")]
pub fn Rocket() -> String {
   return "🚀 server v0.5.0-rc.3".to_string();
}

#[rocket::get("/graphql")]
pub fn GraphiQL() -> content::RawHtml<String> {
   api::GraphiqlSource("/api/graphql", None)
}

#[rocket::get("/graphql/playground")]
pub fn GraphqlPlayground() -> content::RawHtml<String> {
   api::PlaygroundSource("/api/graphql/playground", None)
}

#[rocket::get("/graphql?<request>")]
pub fn GetGraphqlHandler(
    context: &State<DbContext>,
    request: api::GraphQLRequest,
    schema: &State<GraphqlSchema>,
) -> api::GraphQLResponse {
    request.ExecuteSync(&*schema, &*context)
}

#[rocket::post("/graphql", data = "<request>")]
pub fn PostGraphqlHandler(
    context: &State<DbContext>,
    request: api::GraphQLRequest,
    schema: &State<GraphqlSchema>,
) -> api::GraphQLResponse {
    request.ExecuteSync(&*schema, &*context)
}

#[doc(hidden)]
pub fn ReadAppConfig() {}

use {
   crate::{MutationRoot, QueryRoot, service::DbContext},
   juniper::{EmptySubscription, RootNode},
   mbp2::api,
   rocket::{response::content, Config, State}
};
