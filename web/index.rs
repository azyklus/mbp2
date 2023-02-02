#![allow(non_snake_case)]

pub fn SwitchFn(routes: Route) -> Html {
   match routes {
      Route::Home => html! { <Home /> },
      Route::NotFound => html! { <NotFound /> },
      Route::Author { id } => html! { <Author id={id} /> },
      Route::AuthorIndex => html! { <AuthorIndex /> },
      Route::PostIndex => html! { <BlogPostIndex /> },
      Route::Post { id } => html!{ <BlogPost id={id} /> },
   }
}

pub fn main() {
   wasm_logger::init(Config::new(Level::Trace));
   Renderer::<App>::new().render();
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
   #[at("/posts/:id")]
   Post{ id: u128 },
   #[at("/posts")]
   PostIndex,
   #[at("/authors/:id")]
   Author{ id: u128 },
   #[at("/authors")]
   AuthorIndex,
   #[at("/")]
   Home,
   #[not_found]
   #[at("/404")]
   NotFound,
}

pub enum Msg {
   ToggleNav,
}

use {
   self::{
      components::App,
      pages::*,
   },
   log::Level,
   router::prelude::*,
   wasm_logger::Config,
   yew::{prelude::*, Renderer},
};

mod components;
mod pages;

extern crate gloo;
extern crate log;
extern crate pulldown_cmark as cmark;
extern crate reqwasm;
#[macro_use]
extern crate serde;
extern crate wasm_logger;
extern crate web_sys as web;
extern crate ybc;
extern crate yew;
extern crate yew_router as router;
