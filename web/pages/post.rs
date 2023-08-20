pub struct BlogPost {
   inner: Option<BlogPostEx>,
}

#[non_exhaustive]
pub enum BlogPostMsg {
   AddFavourite,
   AddComment(usize, String),
   RetrievePost,
   RmFavourite,
   RmComment(usize),
   TickShare,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlogPostProps {
   pub id: String,
}

impl Component for BlogPost {
   type Message = BlogPostMsg;
   type Properties = BlogPostProps;

   fn create(ctx: &Context<Self>) -> Self {
      ctx.link().send_message(BlogPostMsg::RetrievePost);

      return BlogPost{
         inner: None,
      };
   }

   fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
      return match msg {
         BlogPostMsg::RetrievePost => {
            let query = PostQuery::new(ctx.props().id.clone());
            let json = serde_json::to_string_pretty(&query).unwrap();
            let url = Url::parse("0.0.0.0/api/graphql").unwrap();
            let res = Client::new()
               .get(url)
               .body(json)
               .send();

            let jsonValue: BlogPostEx = serde_json::from_value(res.unwrap().json().unwrap()).unwrap();
            self.inner = Some(jsonValue);

            true
         }
         _ => {
            false
         }
      }
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      let post = self.inner.clone().unwrap();
      let header1 = html!(
         <div class={classes!("justify-content-center")}>
            <h1>{ "Chapter" } { post.body.headers[0].chapter }</h1>
            <p>{ post.body.headers[0].tagline.clone() }</p>
         </div>
      );

      let mut sections: Vec<Html> = vec![];
      for sect in post.body.sections.iter() {
         sections.push(html!(
            <Container>
               <div class={classes!("justify-content-left")}>
                  <h4>{ sect.title.clone() }</h4>
                  <p></p>
               </div>
            </Container>
         ));
      }

      return html!(
         <Container>
            <Blog>
               { header1 }

               <Container>
                  { for sections.iter() }
               </Container>
            </Blog>
         </Container>
      );
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
      return html!(
         <Container>
            <Blog>

            </Blog>
         </Container>
      );
   }
}

use {
   crate::components::layouts::Blog,
   crate::types::{BlogPost as BlogPostEx, PostQuery},
   reqwest::{Url, blocking::Client},
   ybc::*,
   yew::prelude::*,
};
