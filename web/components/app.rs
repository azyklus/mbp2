pub struct App{}

impl Component for App {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> App {
      return App{};
   }

   fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Msg::ToggleNav => {}
      }

      return false;
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{
         <BrowserRouter>
            <Navbar title={"OSB"} />

            <main>
               <Switch<Route> render={SwitchFn} />
            </main>
            <footer class="footer">
               <div class="content has-text-centered">
                  { "Powered by " }
                  <a href="https://yew.rs">{ "Yew" }</a>
                  { " using " }
                  <a href="https://bulma.io">{ "Bulma" }</a>
               </div>
            </footer>
            <script src="https://cdn.jsdelivr.net/npm/less"></script>
         </BrowserRouter>
      }
   }
}

use {
   super::common::Navbar,
   crate::{
      Msg,
      Route,
      SwitchFn,
   },
   router::prelude::*,
   yew::prelude::*,
};
