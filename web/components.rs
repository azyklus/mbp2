pub enum Msg {}

pub struct App {}

impl Component for App {
   type Message = Msg;
   type Properties = ();

   fn create(_ctx: &Context<Self>) -> App {
      return App{};
   }

   fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
      return false;
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      html!{
         <p>{"Hello world!"}</p>
      }
   }
}

use yew::prelude::*;
