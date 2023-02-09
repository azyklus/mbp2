pub struct BlogLayout{}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlogProps {
   #[prop_or_default]
   pub children: Children,
   #[prop_or_default]
   pub classes: Classes,
}

impl Component for BlogLayout {
   type Message = Msg;
   type Properties = BlogProps;

   fn create(_: &Context<Self>) -> Self {
      return BlogLayout{};
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      return html!(
         <Container classes={ctx.props().classes.clone()}>
            <Container classes={classes!("justify-content-center", "m-5")}>
               { ctx.props().children.clone() }
            </Container>
         </Container>
      );
   }
}

use {
   crate::Msg,
   ybc::*, yew::prelude::*,
};