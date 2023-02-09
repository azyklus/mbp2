pub struct NotFound {}

impl Component for NotFound {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> Self {
      return NotFound{};
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      return html!(
         <section class="hero is-danger is-bold is-large">
            <div class="hero-body">
               <div class="container">
                  <h1 class="title">
                     { "Page not found" }
                  </h1>
                  <h2 class="subtitle">
                     { "Page " } { "" } { "page does not seem to exist" }
                  </h2>
               </div>
            </div>
         </section>
      );
   }
}

use crate::Msg;
use yew::prelude::*;
