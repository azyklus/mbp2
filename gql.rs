#[cfg(feature = "async-gql")]
mod __async_gql {
   // TODO: Implement Async-graphql Actix integration.
}

#[cfg(feature = "juniper-gql")]
mod __juniper {
   pub async fn GqlHandler<QueryT, MutationT, SubscriptionT, CtxT, S>(
      schema: &juniper::RootNode<'static, QueryT, MutationT, SubscriptionT, S>,
      context: &CtxT,
      req: HttpRequest,
      payload: Payload,
   ) -> Result<HttpResponse, Error>
   where
      QueryT: juniper::GraphQLTypeAsync<S, Context = CtxT>,
      QueryT::TypeInfo: Sync,
      MutationT: juniper::GraphQLTypeAsync<S, Context = CtxT>,
      MutationT::TypeInfo: Sync,
      SubscriptionT: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
      SubscriptionT::TypeInfo: Sync,
      CtxT: Sync,
      S: ScalarValue + Send + Sync,
   {
      match *req.method() {
         Method::POST => PostGqlHandler(schema, context, req, payload).await,
         Method::GET => GetGqlHandler(schema, context, req).await,
         _ => Err(actix_web::error::UrlGenerationError::ResourceNotFound.into()),
      }
   }

   pub async fn GetGqlHandler<QueryT, MutationT, SubscriptionT, CtxT, S>(
      schema: &juniper::RootNode<'static, QueryT, MutationT, SubscriptionT, S>,
      context: &CtxT,
      req: HttpRequest,
   ) -> Result<HttpResponse, Error>
   where
      QueryT: juniper::GraphQLTypeAsync<S, Context = CtxT>,
      QueryT::TypeInfo: Sync,
      MutationT: juniper::GraphQLTypeAsync<S, Context = CtxT>,
      MutationT::TypeInfo: Sync,
      SubscriptionT: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
      SubscriptionT::TypeInfo: Sync,
      CtxT: Sync,
      S: ScalarValue + Send + Sync,
   {
      let getReq = web::Query::<GetGqlRequest>::from_query(req.query_string())?;
      let req = GraphQLRequest::from(getReq.into_inner());
      let gqlResponse = req.execute(schema, context).await;
      let bodyResponse = serde_json::to_string(&gqlResponse)?;
      let mut response = match gqlResponse.is_ok() {
         true => HttpResponse::Ok(),
         false => HttpResponse::BadRequest(),
      };

      Ok(response.content_type("application/json").body(bodyResponse))
   }

   pub async fn PostGqlHandler<QueryT, MutationT, SubscriptionT, CtxT, S>(
      schema: &juniper::RootNode<'static, QueryT, MutationT, SubscriptionT, S>,
      context: &CtxT,
      req: HttpRequest,
      payload: Payload,
   ) -> Result<HttpResponse, Error>
   where
      QueryT: juniper::GraphQLTypeAsync<S, Context = CtxT>,
      QueryT::TypeInfo: Sync,
      MutationT: juniper::GraphQLTypeAsync<S, Context = CtxT>,
      MutationT::TypeInfo: Sync,
      SubscriptionT: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
      SubscriptionT::TypeInfo: Sync,
      CtxT: Sync,
      S: ScalarValue + Send + Sync,
   {
      let req = match req.content_type() {
         "application/json" => {
            let body = String::from_request(&req, &mut payload.into_inner()).await?;
            serde_json::from_str::<GraphQLBatchRequest<S>>(&body).map_err(JsonPayloadError::Deserialize)
         }
         "application/graphql" => {
            let body = String::from_request(&req, &mut payload.into_inner()).await?;
            Ok(GraphQLBatchRequest::Single(GraphQLRequest::new(body, None, None)))
         }
         _ => Err(JsonPayloadError::ContentType),
      }?;
      let gqlBatchResponse = req.execute(schema, context).await;
      let gqlResponse = serde_json::to_string(&gqlBatchResponse)?;
      let mut response = match gqlBatchResponse.is_ok() {
         true => HttpResponse::Ok(),
         false => HttpResponse::BadRequest(),
      };
      Ok(response.content_type("application/json").body(gqlResponse))
   }

   use {
      crate::api::*,
      actix_web::{
         error::JsonPayloadError,
         http::Method,
         web::{self, Payload},
         Error,
         FromRequest, HttpRequest,
         HttpResponse, HttpMessage,
      },
      juniper::{http::{GraphQLRequest, GraphQLBatchRequest}, ScalarValue},
   };
}

#[cfg(feature = "async-gql")]
pub use self::__async_gql::*;
#[cfg(feature = "juniper-gql")]
pub use self::__juniper::*;
