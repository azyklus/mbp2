pub struct App {
   navActive: bool,
}

impl Component for App {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> App {
      return App{
         navActive: true,
      };
   }

   fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
      return false;
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{
         <Home />
      }
   }
}

use crate::Msg;
use crate::pages::Home;
use yew::prelude::*;
