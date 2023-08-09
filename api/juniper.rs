/// Simple wrapper around an incoming GraphQL request
///
/// See the `http` module for more information. This type can be constructed
/// automatically from both GET and POST routes by implementing the `FromForm`
/// and `FromData` traits.
#[derive(Debug, PartialEq)]
pub struct GraphQLRequest<S = DefaultScalarValue>(GraphQLBatchRequest<S>)
where
    S: ScalarValue;

impl<S: ScalarValue> AsRef<GraphQLBatchRequest<S>> for GraphQLRequest<S> {
    fn as_ref(&self) -> &GraphQLBatchRequest<S> {
        &self.0
    }
}

impl<S: ScalarValue> AsMut<GraphQLBatchRequest<S>> for GraphQLRequest<S> {
    fn as_mut(&mut self) -> &mut GraphQLBatchRequest<S> {
        &mut self.0
    }
}

/// Simple wrapper around the result of executing a GraphQL query
pub struct GraphQLResponse(pub Status, pub String);

/// Generate an HTML page containing GraphiQL
pub fn GraphiqlSource(
    graphql_endpoint_url: &str,
    subscriptions_endpoint_url: Option<&str>,
) -> content::RawHtml<String> {
    content::RawHtml(juniper::http::graphiql::graphiql_source(
        graphql_endpoint_url,
        subscriptions_endpoint_url,
    ))
}

/// Generate an HTML page containing GraphQL Playground
pub fn PlaygroundSource(
    graphql_endpoint_url: &str,
    subscriptions_endpoint_url: Option<&str>,
) -> content::RawHtml<String> {
    content::RawHtml(juniper::http::playground::playground_source(
        graphql_endpoint_url,
        subscriptions_endpoint_url,
    ))
}

impl<S> GraphQLRequest<S>
where
    S: ScalarValue,
{
    /// Synchronously execute an incoming GraphQL query.
    pub fn ExecuteSync<CtxT, QueryT, MutationT, SubscriptionT>(
        &self,
        root_node: &RootNode<QueryT, MutationT, SubscriptionT, S>,
        context: &CtxT,
    ) -> GraphQLResponse
    where
        QueryT: GraphQLType<S, Context = CtxT>,
        MutationT: GraphQLType<S, Context = CtxT>,
        SubscriptionT: GraphQLType<S, Context = CtxT>,
    {
        let response = self.0.execute_sync(root_node, context);
        let status = if response.is_ok() {
            Status::Ok
        } else {
            Status::BadRequest
        };
        let json = serde_json::to_string(&response).unwrap();

        GraphQLResponse(status, json)
    }

    /// Asynchronously execute an incoming GraphQL query.
    pub async fn Execute<CtxT, QueryT, MutationT, SubscriptionT>(
        &self,
        root_node: &RootNode<'_, QueryT, MutationT, SubscriptionT, S>,
        context: &CtxT,
    ) -> GraphQLResponse
    where
        QueryT: GraphQLTypeAsync<S, Context = CtxT>,
        QueryT::TypeInfo: Sync,
        MutationT: GraphQLTypeAsync<S, Context = CtxT>,
        MutationT::TypeInfo: Sync,
        SubscriptionT: GraphQLSubscriptionType<S, Context = CtxT>,
        SubscriptionT::TypeInfo: Sync,
        CtxT: Sync,
        S: Send + Sync,
    {
        let response = self.0.execute(root_node, context).await;
        let status = if response.is_ok() {
            Status::Ok
        } else {
            Status::BadRequest
        };
        let json = serde_json::to_string(&response).unwrap();

        GraphQLResponse(status, json)
    }

    /// Returns the operation names associated with this request.
    ///
    /// For batch requests there will be multiple names.
    pub fn OperationNames(&self) -> Vec<Option<&str>> {
        self.0.operation_names()
    }
}

impl GraphQLResponse {
    /// Constructs an error response outside of the normal execution flow
    ///
    /// # Examples
    ///
    /// ```
    /// # use rocket::http::CookieJar;
    /// # use rocket::form::Form;
    /// # use rocket::response::content;
    /// # use rocket::State;
    /// #
    /// # use juniper::tests::fixtures::starwars::schema::{Database, Query};
    /// # use juniper::{EmptyMutation, EmptySubscription, FieldError, RootNode, Value};
    /// #
    /// # type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;
    /// #
    /// #[rocket::get("/graphql?<request..>")]
    /// fn get_graphql_handler(
    ///     cookies: &CookieJar,
    ///     context: &State<Database>,
    ///     request: GraphQLRequest,
    ///     schema: &State<Schema>,
    /// ) -> juniper_rocket::GraphQLResponse {
    ///     if cookies.get("user_id").is_none() {
    ///         let err = FieldError::new("User is not logged in", Value::null());
    ///         return GraphQLResponse::Error(err);
    ///     }
    ///
    ///     request.ExecuteSync(&*schema, &*context)
    /// }
    /// ```
    pub fn Error(error: FieldError) -> Self {
        let response = http::GraphQLResponse::error(error);
        let json = serde_json::to_string(&response).unwrap();
        GraphQLResponse(Status::BadRequest, json)
    }

    /// Constructs a custom response outside of the normal execution flow
    ///
    /// This is intended for highly customized integrations and should only
    /// be used as a last resort. For normal juniper use, use the response
    /// from GraphQLRequest::ExecuteSync(..).
    pub fn Custom(status: Status, response: serde_json::Value) -> Self {
        let json = serde_json::to_string(&response).unwrap();
        GraphQLResponse(status, json)
    }
}

pub struct GraphQLContext<'f, S: ScalarValue> {
    opts: Options,
    query: Option<String>,
    operation_name: Option<String>,
    variables: Option<InputValue<S>>,
    errors: Errors<'f>,
}

impl<'f, S: ScalarValue> GraphQLContext<'f, S> {
    fn Query(&mut self, value: String) {
        if self.query.is_some() {
            let error = Error::from(ErrorKind::Duplicate).with_name("query");

            self.errors.push(error)
        } else {
            self.query = Some(value);
        }
    }

    fn OperationName(&mut self, value: String) {
        if self.operation_name.is_some() {
            let error = Error::from(ErrorKind::Duplicate).with_name("operation_name");

            self.errors.push(error)
        } else {
            self.operation_name = Some(value);
        }
    }

    fn Variables(&mut self, value: String) {
        if self.variables.is_some() {
            let error = Error::from(ErrorKind::Duplicate).with_name("variables");

            self.errors.push(error)
        } else {
            let parse_result = serde_json::from_str::<InputValue<S>>(&value);

            match parse_result {
                Ok(variables) => self.variables = Some(variables),
                Err(e) => {
                    let error = Error::from(ErrorKind::Validation(Cow::Owned(e.to_string())))
                        .with_name("variables");

                    self.errors.push(error);
                }
            }
        }
    }
}

#[rocket::async_trait]
impl<'f, S> FromForm<'f> for GraphQLRequest<S>
where
    S: ScalarValue + Send,
{
    type Context = GraphQLContext<'f, S>;

    fn init(opts: Options) -> Self::Context {
        GraphQLContext {
            opts,
            query: None,
            operation_name: None,
            variables: None,
            errors: Errors::new(),
        }
    }

    fn push_value(ctx: &mut Self::Context, field: ValueField<'f>) {
        match field.name.key().map(|key| key.as_str()) {
            Some("query") => ctx.Query(field.value.into()),
            Some("operation_name") => ctx.OperationName(field.value.into()),
            Some("variables") => ctx.Variables(field.value.into()),
            Some(key) => {
                if ctx.opts.strict {
                    let error = Error::from(ErrorKind::Unknown).with_name(key);

                    ctx.errors.push(error)
                }
            }
            None => {
                if ctx.opts.strict {
                    let error = Error::from(ErrorKind::Unexpected);

                    ctx.errors.push(error)
                }
            }
        }
    }

    async fn push_data(ctx: &mut Self::Context, field: DataField<'f, '_>) {
        if ctx.opts.strict {
            let error = Error::from(ErrorKind::Unexpected).with_name(field.name);

            ctx.errors.push(error)
        }
    }

    fn finalize(mut ctx: Self::Context) -> rocket::form::Result<'f, Self> {
        if ctx.query.is_none() {
            let error = Error::from(ErrorKind::Missing).with_name("query");

            ctx.errors.push(error)
        }

        match ctx.errors.is_empty() {
            true => Ok(GraphQLRequest(GraphQLBatchRequest::Single(
                http::GraphQLRequest::new(ctx.query.unwrap(), ctx.operation_name, ctx.variables),
            ))),
            false => Err(ctx.errors),
        }
    }
}

const BODY_LIMIT: u64 = 1024 * 100;

#[rocket::async_trait]
impl<'r, S> FromData<'r> for GraphQLRequest<S>
where
    S: ScalarValue,
{
    type Error = String;

    async fn from_data(
        req: &'r Request<'_>,
        data: Data<'r>,
    ) -> data::Outcome<'r, Self, Self::Error> {
        use rocket::tokio::io::AsyncReadExt as _;

        let content_type = req
            .content_type()
            .map(|ct| (ct.top().as_str(), ct.sub().as_str()));
        let is_json = match content_type {
            Some(("application", "json")) => true,
            Some(("application", "graphql")) => false,
            _ => return Box::pin(async move { Forward((data, Status::Ok)) }).await,
        };

        Box::pin(async move {
            let limit = req
                .limits()
                .get("graphql")
                .unwrap_or_else(|| BODY_LIMIT.bytes());
            let mut reader = data.open(limit);
            let mut body = String::new();
            if let Err(e) = reader.read_to_string(&mut body).await {
                return Failure((Status::InternalServerError, format!("{e:?}")));
            }

            Success(GraphQLRequest(if is_json {
                match serde_json::from_str(&body) {
                    Ok(req) => req,
                    Err(e) => return Failure((Status::BadRequest, e.to_string())),
                }
            } else {
                GraphQLBatchRequest::Single(http::GraphQLRequest::new(body, None, None))
            }))
        })
        .await
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for GraphQLResponse {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        let GraphQLResponse(status, body) = self;

        Response::build()
            .header(ContentType::new("application", "json"))
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

use std::{borrow::Cow, io::Cursor};

use rocket::{
    data::{self, FromData, ToByteUnit},
    form::{error::ErrorKind, DataField, Error, Errors, FromForm, Options, ValueField},
    http::{ContentType, Status},
    outcome::Outcome::{Failure, Forward, Success},
    response::{self, content, Responder, Response},
    Data, Request,
};

use juniper::{
    http::{self, GraphQLBatchRequest},
    DefaultScalarValue, FieldError, GraphQLSubscriptionType, GraphQLType, GraphQLTypeAsync,
    InputValue, RootNode, ScalarValue,
};
