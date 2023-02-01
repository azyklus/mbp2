pub struct ContentHeader{}

#[derive(Debug, PartialEq, Properties)]
pub struct ContentHeaderProps {
   #[prop_or_default]
   pub children: Children,
   name: String,
   title: String,
   #[prop_or_default]
   summary: String,
   #[prop_or_default]
   tags: String,
   #[prop_or_default]
   links: Vec<String>,
   logo: String,
}

impl Component for ContentHeader {
   type Message = Msg;
   type Properties = ContentHeaderProps;

   fn create(_: &Context<Self>) -> Self {
      return ContentHeader{};
   }

    fn view(&self, ctx: &Context<Self>) -> Html {
      // TODO: Implement content header component.
      html!{
         <Container classes={classes!("col-xs-8", "col-xs-offset-2", "jumbotron", "text-center")}>
            { ctx.props().children.clone() }
         </Container>
      }
   }
}

use {
   crate::Msg,
   ybc::*,
   yew::prelude::*,
};
