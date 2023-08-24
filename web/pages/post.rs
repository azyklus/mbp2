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
            let res = Client::new()
               .get(Url::parse("0.0.0.0/api/graphql").unwrap())
               .body(serde_json::to_string_pretty(&query).unwrap())
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

      return html!(
         <Container>
            <Blog>
               {
                  for post.body.headers.iter().map(|header| {html!(
                     <Container classes={classes!("justify-content-center")}>
                        <h1>{"Chapter "} { header.chapter }</h1>
                        <p>{ header.tagline.clone() }</p>
                     </Container>
                  )})
               }

               <Container>
                  {
                     for post.body.sections.iter().map(|post|{html!(
                        <Container>
                           <div class={classes!("justify-content-left")}>
                              <h4>{ post.title.clone() }</h4>
                              <p>{ for post.paragraphs.iter() }</p>
                           </div>
                        </Container>
                     )})
                  }
               </Container>
            </Blog>
         </Container>
      );
   }
}

/// INDEX PAGE //
#[non_exhaustive]
pub struct BlogPostIndex{}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PostIndexProps {
   pub filter: PostListFilter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PostListFilter {
   All,
   ByAuthor(BlogAuthor),
   ByTag(String),
   FavouritedBy(String),
   Feed,
}

#[function_component(PostIndex)]
pub fn BlogPostIndex(props: &PostIndexProps) -> Html {
   let props = props.clone();
   let currentPage = use_state(|| 0u32);

   let postList = {
      let currentPage = currentPage.clone();
      let filter = props.filter.clone();

      use_async(async move {
         match filter {
            PostListFilter::All => GetAllPosts().await,
            PostListFilter::ByAuthor(author) => GetPostsByAuthor(author).await,
            _ => Err(Error::RequestError)
         }
      })
   };

   {
      let postList = postList.clone();
      use_effect_with_deps(
         move |_| {
            postList.run();
            || ()
         },
         (props.filter.clone(), *currentPage),
      );
   }

   let callback = {
      let currentPage = currentPage.clone();
      use_callback(
         move |page, _| {
            currentPage.set(page);
         },
         (),
      )
   };

   return html!(
      <Container>
         <Blog>

         </Blog>
      </Container>
   );
}

use {
   crate::{
      components::layouts::Blog,
      error::Error,
      types::{ BlogPost as BlogPostEx, BlogAuthor, PostQuery },
      utils::{ GetAllPosts, GetPostsByAuthor },
   },
   reqwest::{ Url, Client },
   ybc::*,
   yew::prelude::*,
   yew_hooks::prelude::*,
};
