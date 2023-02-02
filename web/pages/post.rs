pub struct BlogPost {}

pub enum BlogPostMsg{}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlogPostProps {
   #[prop_or_default]
   pub title: String,
   pub id: u128,
}

impl Component for BlogPost {
   type Message = BlogPostMsg;
   type Properties = BlogPostProps;

   fn create(_: &Context<Self>) -> Self {
      return BlogPost {};
   }

   fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
      true
   }

   fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
      true
   }

   fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

   fn prepare_state(&self) -> Option<String> {
      None
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      let BlogPostProps { title, id } = ctx.props();

      html! {
         <Container>
            <Blog />
         </Container>
      }
   }
}

// INDEX PAGE //

pub struct BlogPostIndex {}

impl Component for BlogPostIndex {
   type Message = BlogPostMsg;
   type Properties = ();

   fn create(ctx: &Context<Self>) -> Self {
      return BlogPostIndex {};
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      html! {}
   }
}

use {
   crate::components::layouts::Blog,
   ybc::*, yew::prelude::*
};
