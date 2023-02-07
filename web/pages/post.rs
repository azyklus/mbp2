pub struct BlogPost {}

pub enum BlogPostMsg {
   AddFavourite,
   AddComment(usize, String),
   RmFavourite,
   RmComment(usize),
   TickShare,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlogPostProps {
   pub id: u128,
}

impl Component for BlogPost {
   type Message = BlogPostMsg;
   type Properties = BlogPostProps;

   fn create(_: &Context<Self>) -> Self {
      return BlogPost {};
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      let BlogPostProps { id } = ctx.props();

      html! {
         <Container>
            <Blog>
               <Markup id={id.to_string()} />
            </Blog>
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
   crate::components::{
      common::Markup,
      layouts::Blog,
   },
   ybc::*, yew::prelude::*
};
