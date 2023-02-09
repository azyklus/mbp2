pub struct ContentHeader{}

#[derive(Debug, PartialEq, Properties)]
pub struct ContentHeaderProps {
   pub name: String,
   pub logo: String,
   #[prop_or_default]
   pub children: Children,
   #[prop_or_default]
   pub classes: Classes,
   #[prop_or_default]
   pub summary: String,
   #[prop_or_default]
   pub tags: String,
   #[prop_or_default]
   pub links: Vec<String>,
}

impl Component for ContentHeader {
   type Message = Msg;
   type Properties = ContentHeaderProps;

   fn create(_: &Context<Self>) -> Self {
      return ContentHeader{};
   }

    fn view(&self, ctx: &Context<Self>) -> Html {
      // TODO: Implement content header component.
      return html!(
         <Container classes={ctx.props().classes.clone()}>
            <Container classes={classes!("col-xs-8", "col-xs-offset-2", "jumbotron", "text-center")}>
               { ctx.props().children.clone() }
            </Container>
         </Container>
      );
   }
}

use {
   crate::Msg,
   ybc::*,
   yew::prelude::*,
};
