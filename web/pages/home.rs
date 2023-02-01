pub struct Home{}

impl Component for Home {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> Self {
      return Home{};
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{
         <Main>
         </Main>
      }
   }
}

use crate::Msg;
use crate::components::layouts::Main;
use yew::prelude::*;
