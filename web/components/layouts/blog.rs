pub struct BlogLayout{}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlogProps {
   #[prop_or_default]
   pub children: Children,
}

impl Component for BlogLayout {
   type Message = Msg;
   type Properties = BlogProps;

   fn create(_: &Context<Self>) -> Self {
      return BlogLayout{};
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      html!{
         <div>
            { ctx.props().children.clone() }
         </div>
      }
   }
}

use crate::Msg;
use yew::prelude::*;
