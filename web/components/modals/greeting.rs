pub struct LaunchModal{}

impl Component for LaunchModal {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> LaunchModal {
      return LaunchModal{};
   }

   fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool { true }

   fn view(&self, _: &Context<Self>) -> Html {
      let classes: Classes = classes!("jumbotron", "text-center");

      html! {
         <Container classes={classes}>
            <Container classes={classes!("hero", "has-background")}>
               <Container classes={classes!("background")}>
                  <h1>{ "Other Skies Blog" }</h1>
                  <Container classes={classes!("row")}>
                     <span>
                        <a rel="me" href="https://mas.to/@zub">
                           { "@zub@mas.to" }
                        </a>
                     </span>
                  </Container>
                  <Container>
                     <p>
                        { "Please sign in for access to donation perks." }
                     </p>
                     <div class="container" style="width:25%;">
                        <a href="/" class={classes!("btn", "btn-primary", "btn-lg", "btn-login", "btn-block")}>
                           { "Sign In" }
                        </a>
                     </div>
                     <p>
                        { "Please consider becoming a Patron today if you are not already!" }
                     </p>
                  </Container>
               </Container>
            </Container>
            <iframe id="kofiframe"
                    src="https://ko-fi.com/azyklus/?hidefeed=true&widget=true&embed=true&preview=true"
                    style={"border: 8px, width:80%, padding: 4px, background:'#f9f9f9'"}
                    height="720"
                    title="azyklus">
            </iframe>
         </Container>
      }
   }
}

use {crate::Msg, ybc::*, yew::prelude::*};
