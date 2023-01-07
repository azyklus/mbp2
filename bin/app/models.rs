fn wowHelper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

#[doc="Customize Handlebars model handler."]
pub fn Customise(hbs: &mut Handlebars) {
   // TODO: Add more registers as needed.
   hbs.register_helper("wow", Box::new(wowHelper));
}

pub mod home;

use tmpl::handlebars::{
   Context,
   Handlebars,
   Helper,
   HelperResult,
   RenderContext,
   JsonRender,
   Output,
};
