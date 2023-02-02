pub struct BlogLayout{}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlogProps {
   #[prop_or_default]
   pub children: Children,
}

impl Component for BlogLayout {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> Self {
      return BlogLayout{};
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{}
   }
}

use crate::Msg;
use yew::prelude::*;
