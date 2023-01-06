pub fn NotFoundHandler<'r>(_: Status, req: &'r Request) -> catcher::BoxFuture<'r> {
   let responder = Custom(Status::NotFound, format!("Couldn't find: {}", req.uri()));
   Box::pin(async move { responder.respond_to(req) })
}

#[derive(Clone)]
pub struct MainController {
   data: &'static str,
}

impl MainController {
   fn routes(data: &'static str) -> Vec<Route> {
      vec![Route::new(Get, "/<id>", Self { data })]
   }
}

#[rocket::async_trait]
impl Handler for MainController {
   async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> route::Outcome<'r> {
      let self_data = self.data;
      let id = req.param::<&str>(0)
            .and_then(Result::ok)
            .or_forward(data);

      return Outcome::from(req, format!("{} - {}", self_data, try_outcome!(id)));
   }
}

use rocket::{catcher, Data, Request, Route};
use rocket::http::{Method::*, Status};
use rocket::outcome::{IntoOutcome, try_outcome};
use rocket::response::{Responder, status::Custom};
use rocket::route::{self, Handler, Outcome};

pub mod api;
pub mod home;
