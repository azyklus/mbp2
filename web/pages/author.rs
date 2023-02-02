pub struct Author{}

pub enum AuthorMsg{}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AuthorProps {
   #[prop_or_default]
   pub name: String,
   pub id: u128,
}

impl Component for Author {
   type Message = AuthorMsg;
   type Properties = AuthorProps;

   fn create(_: &Context<Self>) -> Self {
      return Author{};
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{}
   }
}

pub struct AuthorIndex{}

impl Component for AuthorIndex {
   type Message = AuthorMsg;
   type Properties = ();

   fn create(_: &Context<Self>) -> Self {
      return AuthorIndex{};
   }

   fn view(&self, _: &Context<Self>) -> Html {
      html!{}
   }
}

use {
   ybc::*, yew::prelude::*
};
