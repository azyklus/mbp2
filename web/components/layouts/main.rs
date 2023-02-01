pub struct Main {
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MainProps {
   #[prop_or_default]
   pub children: Children,
   #[prop_or_default]
   pub classes: Classes,
}

impl Component for Main {
   type Message = Msg;
   type Properties = MainProps;

   fn create(_: &Context<Self>) -> Self {
      return Main{};
   }

   fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Msg::ToggleNav => return true,
      }
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      html!{
         <Container classes={ctx.props().classes.clone()}>
            <div class={classes!("col-xs-8", "col-xs-offset-2", "jumbotron", "text-center")}>
               <h1>{ "Other Skies Blog" }</h1>
               <Container classes={classes!("container", "row")}>
                  <span>
                     <a rel="me" href="https://mas.to/@zub">
                        { "@zub@mas.to" }
                     </a>
                  </span>
               </Container>
               <p>
                  { "Please sign in for access to donation perks." }
               </p>
               <a href="/" class={classes!("btn", "btn-primary", "btn-lg", "btn-login", "btn-block")}>
                  { "Sign In" }
               </a>
               <p>
                  { "Please consider becoming a Patron today if you are not already!" }
               </p>
               <iframe id="kofiframe"
                       src="https://ko-fi.com/azyklus/?hidefeed=true&widget=true&embed=true&preview=true"
                       style={"border: 8px, width:80%, padding: 4px, background:'#f9f9f9'"}
                       height="720"
                       title="azyklus">
               </iframe>
            </div>

            <Container classes={classes!("row")}>
               { ctx.props().children.clone() }
            </Container>
         </Container>
      }
   }
}

use {
   crate::Msg,
   ybc::*,
   yew::prelude::*,
};
