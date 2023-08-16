pub struct ErrStateModal {}

pub enum ErrStateMsg {}

#[derive(Debug, PartialEq, Properties)]
pub struct ErrStateProps {
   pub Title: String,
   pub Msg: String,
}

impl Component for ErrStateModal {
   type Message = ErrStateMsg;
   type Properties = ErrStateProps;

   fn create(_: &Context<Self>) -> Self {
      return ErrStateModal {};
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      let ErrStateProps { Title: title, Msg: message } = ctx.props();

      let body: Html = html! {
         <Container>
            <h3>{ "An error occurred:" } { title.clone() }</h3>
            <p>{ message }</p>
         </Container>
      };

      let footer: Html = html! {
         <Container>
            <Link<Route> classes={classes!("btn", "btn-primary", "btn-md", "btn-login", "btn-block")} to={Route::Home}>
               { "Go home" }
            </Link<Route>>
         </Container>
      };

      html! {
         <Container classes={classes!("col-xs-8", "col-xs-offset-2", "jumbotron", "justify-content-center")}>
            <ModalCard id="modal" title={title.clone()} body={body} footer={footer} />
         </Container>
      }
   }
}

use {crate::Route, router::prelude::*, ybc::*, yew::prelude::*};
