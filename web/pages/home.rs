pub struct Home{}

impl Component for Home {
   type Message = Msg;
   type Properties = ();

   fn create(_: &Context<Self>) -> Self {
      return Home{};
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{
         <div class={classes!("container")}>
            <div class={classes!("col-xs-8", "col-xs-offset-2", "jumbotron", "text-center")}>
               <h1>{ "Other Skies Blog" }</h1>
               <div class={classes!("container", "row")}>
                  <span>
                     <a rel="me" href="https://mas.to/@zub">
                        { "@zub@mas.to" }
                     </a>
                  </span>
               </div>
               <p>
                  {"Please sign in for access to donation perks."}
               </p>
               <a href="javascript:void(0)" class={classes!("btn", "btn-primary", "btn-lg", "btn-login", "btn-block")}>
                  {"Sign In"}
               </a>
               <p>
                  {"Please consider becoming a Patron today if you are not already!"}
               </p>
               <iframe id="kofiframe"
                       src="https://ko-fi.com/azyklus/?hidefeed=true&widget=true&embed=true&preview=true"
                       style={"border: 8px, width:100%, padding: 4px, background:'#f9f9f9'"}
                       height="720"
                       title="azyklus">
               </iframe>
            </div>

            <Main />
         </div>
      }
   }
}

use crate::Msg;
use crate::components::layouts::Main;
use yew::prelude::*;

