/*
pub fn Index<'r>(req: &'r Request, _data: Data<'r>) -> route::BoxFuture<'r> {
   Box::pin(async move {
      //let mut workDir: String = std::env::var("WORK_DIR").expect("could not load var");
      //workDir.push_str("/web");

      let path = Path::new("web/index.html");//.join("/static");
      if let Some(file) = NamedFile::open(path).await.ok() {
         return route::Outcome::from(req, tokio::fs::read_to_string(file.path()).await.ok());
      }

      let responder = status::Custom(Status::BadRequest, content::RawHtml(r#"<p>Cannot locate html file to load.</p>"#));
      route::Outcome::from(req, responder)
   })
}
*/

#[rocket::get("/")]
pub fn Home() -> Template {
   let model: HomeModel = HomeModel {
      Title: "Other Skies".to_string(),
      Parent: "layouts/home".to_string(),
   };

   return Template::render(
      "index.html",
      context! {
         title: model.Title,
         parent: model.Parent,
      },
   );
}

use crate::models::home::HomeModel;
use tmpl::Template;
