pub struct Main {
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MainProps {
   #[prop_or_default]
   pub children: Children,
}

impl Component for Main {
   type Message = Msg;
   type Properties = MainProps;

   fn create(_: &Context<Self>) -> Self {
      return Main{};
   }

   fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Msg::ToggleNav => return true,
      }
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      html!{
         <Container>
            <Container classes={classes!("justify-content-center", "m-5")}>
               { ctx.props().children.clone() }
            </Container>
         </Container>
      }
   }
}

use {
   crate::Msg,
   ybc::*,
   yew::prelude::*,
};
