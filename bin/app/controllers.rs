async fn getIndex() -> Result<NamedFile, NotFound<String>> {
   NamedFile::open("index.html")
      .await
      .map_err(|e| NotFound(e.to_string()))
}

#[rocket::get("/<path..>")]
pub async fn DistFiles(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
   let path = PathBuf::from("../../web/dist").join(path);
   match NamedFile::open(path).await {
      Ok(f) => Ok(f),
      Err(_) => getIndex().await,
   }
}

#[rocket::get("/")]
pub async fn Index() -> Result<NamedFile, NotFound<String>> {
   NamedFile::open("/index.html").await.map_err(|e| NotFound(e.to_string()))
}

pub fn NotFoundHandler<'r>(_: Status, req: &'r Request) -> catcher::BoxFuture<'r> {
   let responder = Custom(Status::NotFound, format!("Couldn't find: {}", req.uri()));
   Box::pin(async move { responder.respond_to(req) })
}

/*
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
*/

use rocket::{catcher, Data, Request, Route};
use rocket::fs::NamedFile;
use rocket::http::{Method::*, Status};
use rocket::outcome::{IntoOutcome, try_outcome};
use rocket::response::{Responder, status::{Custom, NotFound}};
use rocket::route::{self, Handler, Outcome};
use std::path::PathBuf;

pub mod api;
pub mod blog;
pub mod home;
