#[cfg(feature = "async-gql")]
mod __graphql {
   /// A batch request which can be extracted from a request's body.
   ///
   /// # Examples
   ///
   /// ```ignore
   /// #[rocket::post("/graphql", data = "<request>", format = "application/json", rank = 1)]
   /// async fn graphql_request(schema: State<'_, ExampleSchema>, request: BatchRequest) -> Response {
   ///     request.execute(&schema).await
   /// }
   /// ```
   #[derive(Debug)]
   pub struct GraphQLBatchRequest(pub async_graphql::BatchRequest);

   impl GraphQLBatchRequest {
      /// Shortcut method to execute the request on the executor.
      pub async fn Execute<E>(self, executor: &E) -> GraphQLResponse
      where
         E: Executor,
      {
         GraphQLResponse(executor.execute_batch(self.0).await)
      }
   }

   #[rocket::async_trait]
   impl<'r> FromData<'r> for GraphQLBatchRequest {
      type Error = ParseRequestError;

      async fn from_data(req: &'r rocket::Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
         let opts: MultipartOptions = req.rocket().state().copied().unwrap_or_default();

         let request = async_graphql::http::receive_batch_body(
            req.headers().get_one("Content-Type"),
            data.open(req.limits().get("graphql").unwrap_or_else(|| 128.kibibytes())).compat(),
            opts,
         )
         .await;

         match request {
            Ok(request) => data::Outcome::Success(Self(request)),
            Err(e) => data::Outcome::Failure((
               match e {
                  ParseRequestError::PayloadTooLarge => Status::PayloadTooLarge,
                  _ => Status::BadRequest,
               },
               e,
            )),
         }
      }
   }

   /// A GraphQL request which can be extracted from the request's body.
   ///
   /// # Examples
   ///
   /// ```ignore
   /// #[rocket::post("/graphql", data = "<request>", format = "application/json", rank = 2)]
   /// async fn graphql_request(schema: State<'_, ExampleSchema>, request: Request) -> Result<Response, Status> {
   ///     request.Execute(&schema).await
   /// }
   /// ```
   #[derive(Debug)]
   pub struct GraphQLRequest(pub async_graphql::Request);

   impl GraphQLRequest {
      /// Shortcut method to execute the request on the schema.
      pub async fn Execute<E>(self, executor: &E) -> GraphQLResponse
      where
         E: Executor,
      {
         GraphQLResponse(executor.execute(self.0).await.into())
      }

      /// Insert some data for this request.
      #[must_use]
      pub fn Data<D: Any + Send + Sync>(mut self, data: D) -> Self {
         self.0.data.insert(data);
         self
      }
   }

   impl From<GraphQLQuery> for GraphQLRequest {
      fn from(query: GraphQLQuery) -> Self {
         let mut request = async_graphql::Request::new(query.Query);

         if let Some(operation_name) = query.OperationName {
            request = request.operation_name(operation_name);
         }

         if let Some(variables) = query.Variables {
            let value = serde_json::from_str(&variables).unwrap_or_default();
            let variables = async_graphql::Variables::from_json(value);
            request = request.variables(variables);
         }

         GraphQLRequest(request)
      }
   }

   /// A GraphQL request which can be extracted from a query string.
   ///
   /// # Examples
   ///
   /// ```ignore
   /// #[rocket::get("/graphql?<query..>")]
   /// async fn graphql_query(schema: State<'_, ExampleSchema>, query: Query) -> Result<Response, Status> {
   ///     query.execute(&schema).await
   /// }
   /// ```
   #[derive(FromForm, Debug)]
   pub struct GraphQLQuery {
      #[field(name = "query")]
      pub Query: String,
      #[field(name = "operationName")]
      pub OperationName: Option<String>,
      #[field(name = "variables")]
      pub Variables: Option<String>,
   }

   impl GraphQLQuery {
      /// Shortcut method to execute the request on the schema.
      pub async fn Execute<E>(self, executor: &E) -> GraphQLResponse
      where
         E: Executor,
      {
         let request: GraphQLRequest = self.into();
         request.Execute(executor).await
      }
   }

   #[rocket::async_trait]
   impl<'r> FromData<'r> for GraphQLRequest {
      type Error = ParseRequestError;

      async fn from_data(req: &'r rocket::Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
         GraphQLBatchRequest::from_data(req, data)
            .await
            .and_then(|request| match request.0.into_single() {
               Ok(single) => data::Outcome::Success(Self(single)),
               Err(e) => data::Outcome::Failure((Status::BadRequest, e)),
            })
      }
   }

   /// Wrapper around `async-graphql::Response` that is a Rocket responder so it
   /// can be returned from a routing function in Rocket.
   ///
   /// It contains a `BatchResponse` but since a response is a type of batch
   /// response it works for both.
   #[derive(Debug)]
   pub struct GraphQLResponse(pub async_graphql::BatchResponse);

   impl From<async_graphql::BatchResponse> for GraphQLResponse {
      fn from(batch: async_graphql::BatchResponse) -> Self {
         Self(batch)
      }
   }

   impl From<async_graphql::Response> for GraphQLResponse {
      fn from(res: async_graphql::Response) -> Self {
         Self(res.into())
      }
   }

   impl<'r> Responder<'r, 'static> for GraphQLResponse {
      fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
         let body = serde_json::to_string(&self.0).unwrap();

         let mut response = rocket::Response::new();
         response.set_header(ContentType::new("application", "json"));

         if self.0.is_ok() {
            if let Some(cache_control) = self.0.cache_control().value() {
               response.set_header(Header::new("cache-control", cache_control));
            }
         }

         for (name, value) in self.0.http_headers_iter() {
            if let Ok(value) = value.to_str() {
               response.adjoin_header(Header::new(name.as_str().to_string(), value.to_string()));
            }
         }

         response.set_sized_body(body.len(), Cursor::new(body));

         Ok(response)
      }
   }

   use {
      async_graphql::{http::MultipartOptions, Executor, ParseRequestError},
      core::any::Any,
      rocket::{
         data::{self, Data, FromData, ToByteUnit},
         form::FromForm,
         http::{ContentType, Header, Status},
         response::{self, Responder},
      },
      std::io::Cursor,
      tokio_util::compat::TokioAsyncReadCompatExt,
   };
}

#[cfg(feature = "juniper-gql")]
mod __juniper {
   #[derive(Clone, Debug, Deserialize, PartialEq)]
   pub struct GetGqlRequest {
      query: String,
      #[serde(rename = "operationName")]
      opName: Option<String>,
      #[serde(rename = "variables")]
      vars: Option<String>,
   }

   impl<S> From<GetGqlRequest> for GraphQLRequest<S>
   where
      S: ScalarValue,
   {
      fn from(getReq: GetGqlRequest) -> Self {
         let GetGqlRequest { query, opName, vars } = getReq;
         let vars = vars.map(|s| serde_json::from_str(&s).unwrap());
         return Self::new(query, opName, vars);
      }
   }

   use juniper::{http::GraphQLRequest, ScalarValue};

   #[cfg(feature="gql-subs")]
   pub use self::subscriptions::*;

   #[cfg(feature = "gql-subs")]
   mod subscriptions {
      pub type ConnectionSplitSink<QueryT, MutationT, SubscriptionT, CtxT, S, I> =
         Arc<Mutex<SplitSink<Connection<ArcSchema<QueryT, MutationT, SubscriptionT, CtxT, S>, I>, Message>>>;

      pub type ConnectionSplitStream<QueryT, MutationT, SubscriptionT, CtxT, S, I> =
         Arc<Mutex<SplitStream<Connection<ArcSchema<QueryT, MutationT, SubscriptionT, CtxT, S>, I>>>>;

      pub struct SubscriptionActor<QueryT, MutationT, SubscriptionT, CtxT, S, I>
      where
         QueryT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         QueryT::TypeInfo: Send + Sync,
         MutationT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         MutationT::TypeInfo: Send + Sync,
         SubscriptionT: GraphQLSubscriptionType<S, Context = CtxT> + Send + 'static,
         SubscriptionT::TypeInfo: Send + Sync,
         CtxT: Unpin + Send + Sync + 'static,
         S: ScalarValue + Send + Sync + 'static,
         I: Init<S, CtxT> + Send,
      {
         gqlTx: ConnectionSplitSink<QueryT, MutationT, SubscriptionT, CtxT, S, I>,
         gqlRx: ConnectionSplitStream<QueryT, MutationT, SubscriptionT, CtxT, S, I>,
      }

      impl<QueryT, MutationT, SubscriptionT, CtxT, S, I> StreamHandler<Result<ws::Message, ws::ProtocolError>>
         for SubscriptionActor<QueryT, MutationT, SubscriptionT, CtxT, S, I>
      where
         QueryT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         QueryT::TypeInfo: Send + Sync,
         MutationT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         MutationT::TypeInfo: Send + Sync,
         SubscriptionT: GraphQLSubscriptionType<S, Context = CtxT> + Send + 'static,
         SubscriptionT::TypeInfo: Send + Sync,
         CtxT: Unpin + Send + Sync + 'static,
         S: ScalarValue + Send + Sync + 'static,
         I: Init<S, CtxT> + Send,
      {
         fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
            let msg = msg.map(|r| Message(r));

            match msg {
               Ok(msg) => {
                  let tx = self.gqlTx.clone();

                  async move {
                     let mut tx = tx.lock().unwrap();
                     tx.send(msg).await.expect("Infallible: this should not happen");
                  }
                  .into_actor(self)
                  .wait(ctx);
               }
               Err(_) => {
                  // TODO: trace
                  // ignore the message if there's a transport error
               }
            }
         }
      }

      /// juniper -> actor
      impl<QueryT, MutationT, SubscriptionT, CtxT, S, I> Actor for SubscriptionActor<QueryT, MutationT, SubscriptionT, CtxT, S, I>
      where
         QueryT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         QueryT::TypeInfo: Send + Sync,
         MutationT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         MutationT::TypeInfo: Send + Sync,
         SubscriptionT: GraphQLSubscriptionType<S, Context = CtxT> + Send + 'static,
         SubscriptionT::TypeInfo: Send + Sync,
         CtxT: Unpin + Send + Sync + 'static,
         S: ScalarValue + Send + Sync + 'static,
         I: Init<S, CtxT> + Send,
      {
         type Context = ws::WebsocketContext<Self>;

         fn started(&mut self, ctx: &mut Self::Context) {
            let stream = self.gqlRx.clone();
            let addr = ctx.address();

            let fut = async move {
               let mut stream = stream.lock().unwrap();
               while let Some(message) = stream.next().await {
                  // sending the message to self so that it can be forwarded back to the client
                  addr.do_send(ServerMessageWrapper { message });
               }
            }
            .into_actor(self);

            // TODO: trace
            ctx.spawn(fut);
         }

         fn stopped(&mut self, _: &mut Self::Context) {
            // TODO: trace
         }
      }

      impl<QueryT, MutationT, SubscriptionT, CtxT, S, I> actix::prelude::Handler<ServerMessageWrapper<S>>
         for SubscriptionActor<QueryT, MutationT, SubscriptionT, CtxT, S, I>
      where
         QueryT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         QueryT::TypeInfo: Send + Sync,
         MutationT: GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
         MutationT::TypeInfo: Send + Sync,
         SubscriptionT: GraphQLSubscriptionType<S, Context = CtxT> + Send + 'static,
         SubscriptionT::TypeInfo: Send + Sync,
         CtxT: Unpin + Send + Sync + 'static,
         S: ScalarValue + Send + Sync + 'static,
         I: Init<S, CtxT> + Send,
      {
         type Result = ();

         fn handle(&mut self, msg: ServerMessageWrapper<S>, ctx: &mut Self::Context) -> Self::Result {
            let msg = serde_json::to_string(&msg.message);
            match msg {
               Ok(msg) => ctx.text(msg),
               Err(e) => {
                  let reason = ws::CloseReason {
                     code: ws::CloseCode::Error,
                     description: Some(format!("error serializing response: {}", e)),
                  };

                  // TODO: trace
                  ctx.close(Some(reason))
               }
            };
            ()
         }
      }

      #[derive(Message)]
      #[rtype(result = "()")]
      pub struct ServerMessageWrapper<S>
      where
         S: ScalarValue + Send + Sync + 'static,
      {
         message: ServerMessage<S>,
      }

      pub struct Message(ws::Message);

      impl<S: ScalarValue> std::convert::TryFrom<Message> for ClientMessage<S> {
         type Error = SubsError;

         fn try_from(msg: Message) -> Result<Self, Self::Error> {
            match msg.0 {
               ws::Message::Text(text) => serde_json::from_slice(text.as_bytes()).map_err(|e| SubsError::Serde(e)),
               ws::Message::Close(_) => Ok(ClientMessage::ConnectionTerminate),
               _ => Err(SubsError::UnexpectedClientMessage),
            }
         }
      }

      /// Errors that can happen while handling client messages
      #[derive(Debug)]
      pub enum SubsError {
         /// Errors that can happen while deserializing client messages
         Serde(serde_json::Error),

         /// Error for unexpected client messages
         UnexpectedClientMessage,
      }

      impl fmt::Display for SubsError {
         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
               Self::Serde(e) => write!(f, "serde error: {}", e),
               Self::UnexpectedClientMessage => {
                  write!(f, "unexpected message received from client")
               }
            }
         }
      }

      impl std::error::Error for SubsError {}

      use {
         actix::{prelude::*, Actor, StreamHandler},
         actix_web_actors::ws,
         juniper::{
            futures::{
               stream::{SplitSink, SplitStream, StreamExt},
               SinkExt,
            },
            GraphQLSubscriptionType, GraphQLTypeAsync, ScalarValue,
         },
         juniper_graphql_ws::{ArcSchema, ClientMessage, Connection, Init, ServerMessage},
         std::{
            fmt,
            sync::{Arc, Mutex},
         },
      };
   }
}

#[cfg(feature = "juniper-gql")]
mod juniper;

#[cfg(feature="juniper-gql")]
pub use self::__juniper::*;
