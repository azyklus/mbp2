pub fn Index<'r>(req: &'r Request, _data: Data<'r>) -> route::BoxFuture<'r> {
   Box::pin(async move {
      let mut workDir: String = std::env::var("WORK_DIR").expect("could not load var");
      workDir.push_str("/static");

      let mut path: PathBuf = Path::new(&workDir).join("/web");
      if path.is_dir() {
         path.push("/index.html");
      }

      if let Some(file) = NamedFile::open(path).await.ok() {
         return route::Outcome::from(req, tokio::fs::read_to_string(file.path()).await.ok());
      }

      let responder = status::Custom(Status::BadRequest, content::RawHtml(r#"<p>Cannot locate html file to load.</p>"#));
      route::Outcome::from(req, responder)
   })
}

use rocket::{
   http::Status,
   response::{content, status},
   route,
   fs::NamedFile,
   Data,
   Request,
};

use std::path::{PathBuf, Path};
