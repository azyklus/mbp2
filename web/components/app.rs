pub enum Msg {}

pub struct App {}

impl Component for App {
   type Message = Msg;
   type Properties = ();

   fn create(_ctx: &Context<Self>) -> App {
      return App{};
   }

   fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
      return false;
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{
         <Main />
      }
   }
}

use super::layouts::Main;
use yew::prelude::*;
