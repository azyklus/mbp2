pub struct Main{}

impl Component for Main {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> Self {
      return Main{};
   }

   fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Msg::ToggleNav => return true,
      }
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{
      }
   }
}

use {
   crate::Msg,
   yew::prelude::*,
};
