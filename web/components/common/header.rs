pub struct ContentHeader{}

#[derive(Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct ContentHeaderProps {
   name: String,
   title: String,
   summary: String,
   tags: String,
   links: Vec<String>,
   logo: String,
}

impl Component for ContentHeader {
   type Message = Msg;
   type Properties = ContentHeaderProps;

   fn create(_: &Context<Self>) -> Self {
      return ContentHeader{};
   }

    fn view(&self, _: &Context<Self>) -> Html {
      // TODO: Implement content header component.
      html!{}
   }
}

use {
   crate::Msg,
   yew::prelude::*,
};
