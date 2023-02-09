#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageContent {
   #[serde(rename="data")]
   pub Data: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkupType {
   Markdown,
   Html,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct MarkupProps {
   pub id: String,
   #[prop_or_default]
   pub classes: Classes,
}

pub struct Markup {
   pub Inner: FetchState<PageContent>,
}

impl Markup {
   fn renderMarkup(&self) -> String {
      let Self { Inner: md } = self;
      let mut options: Options = Options::empty();
      options.insert(Options::ENABLE_STRIKETHROUGH);
      options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

      return match md {
         FetchState::Success(mu) => {
            let mu = mu.Data.clone();
            let parser: Parser = Parser::new_ext(&mu[..], options);
            let mut htmlOut: String = String::new();

            html::push_html(&mut htmlOut, parser);
            htmlOut
         },
         _ => "Failed to load...".to_string(),
      };
   }
}

impl Component for Markup {
   type Message = FetchStateMsg<PageContent>;
   type Properties = MarkupProps;

   fn create(_: &Context<Self>) -> Self {
      return Self{
         Inner: FetchState::NotFetching,
      };
   }

   fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
      ctx.link().send_message(FetchStateMsg::GetData);
      return true;
   }

   fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
      let uri: String = format!("/api/entry/{}", ctx.props().id);

      return match msg {
         FetchStateMsg::SetDataFetchState(state) => {
            self.Inner = state;
            true
         },
         FetchStateMsg::GetData => {
            ctx.link().send_future(async move {
               match Request::get(uri.as_str()).send().await {
                  Ok(m) => match m.json().await {
                     Ok(m) => FetchStateMsg::SetDataFetchState(FetchState::Success(m)),
                     Err(e) => FetchStateMsg::SetDataFetchState(FetchState::Failed(FetchError {
                        Err: e.to_string(),
                     }))
                  },
                  Err(e) => FetchStateMsg::SetDataFetchState(FetchState::Failed(FetchError {
                     Err: e.to_string(),
                  })),
               }
            });

            ctx.link().send_message(FetchStateMsg::SetDataFetchState(FetchState::Fetching));

            false
         },
      };
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      if matches!(&self.Inner, &FetchState::NotFetching) {
         ctx.link().send_message(FetchStateMsg::GetData);
      }

      let div = web::window()
         .unwrap()
         .document()
         .unwrap()
         .create_element("div")
         .unwrap();

      div.set_inner_html(&self.renderMarkup());
      let node: Node = Node::from(div);

      return VNode::VRef(node);
   }
}

use {
   crate::utils::*,
   cmark::{html, Options, Parser},
   reqwasm::http::Request,
   yew::{
      prelude::*,
      virtual_dom::VNode,
   },
   web::Node,
};
