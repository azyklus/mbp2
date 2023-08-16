pub struct App {}

impl Component for App {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> App {
      return App {};
   }

   fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
      return match msg {
         Msg::ToggleNav => true,
      };
   }

   fn view(&self, _: &Context<Self>) -> Html {
      return html!(
         <BrowserRouter>
            <Navbar title={"OSB"} />

            <main>
               <Switch<Route> render={SwitchFn} />
            </main>

            <Container classes={classes!("footer")}>
               <Container classes={classes!("content", "has-text-centered")}>
                  { "Powered by " }
                  <a href="https://yew.rs">{ "Yew" }</a>
                  { " and " }
                  <a href="https://rocket.rs">{ "Rocket" }</a>
                  { " using " }
                  <a href="https://bulma.io">{ "Bulma" }</a>
               </Container>
            </Container>

            <script src="https://cdn.jsdelivr.net/npm/less"></script>
         </BrowserRouter>
      );
   }
}

use {
   super::common::Navbar,
   crate::{Msg, Route, SwitchFn},
   router::prelude::*,
   ybc::*,
   yew::prelude::*,
};
