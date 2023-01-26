pub struct Main {}

impl Component for Main {
   type Message = Msg;
   type Properties = ();

   fn create(ctx: &Context<Self>) -> Self {
      todo!()
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      html!{}
   }
}

use crate::Msg;
use yew::prelude::*;
