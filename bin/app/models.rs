fn wowHelper(h: &Helper<'_, '_>, _: &Handlebars, _: &Context, _: &mut RenderContext<'_, '_>, out: &mut dyn Output) -> HelperResult {
   if let Some(param) = h.param(0) {
      out.write("<b><i>")?;
      out.write(&param.value().render())?;
      out.write("</b></i>")?;
   }

   Ok(())
}

#[doc = "Customize Handlebars model handler."]
pub fn Customise(hbs: &mut Handlebars) -> Result<(), Box<dyn std::error::Error>> {
   // TODO: Add more registers as needed.
   if let Ok(var) = std::env::var("WORK_DIR") {
      let workDir: PathBuf = Path::new(&var).join("/web");

      if let Err(e) = hbs.register_templates_directory(".html", workDir) {
         return Err(e.into());
      }
   }

   hbs.register_helper("wow", Box::new(wowHelper));

   return Ok(());
}

pub mod home;

use {
   handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext},
   std::path::{Path, PathBuf},
};
