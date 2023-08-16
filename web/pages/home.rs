pub const HOME_STYLES: &str = include_str!("./home.page.less");

pub struct Home {}

impl Component for Home {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> Self {
      return Home {};
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html! {
         <Main>
            <LaunchModal />
         </Main>
      }
   }
}

use {
   crate::{
      components::{layouts::Main, modals::LaunchModal},
      Msg,
   },
   yew::prelude::*,
};
