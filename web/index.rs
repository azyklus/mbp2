#![allow(non_snake_case)]

pub fn SwitchFn(routes: Route) -> Html {
   return match routes {
      Route::Home => html!( <Home /> ),
      Route::NotFound => html!( <NotFound /> ),
      Route::Author { id } => html!( <Author id={id} /> ),
      Route::AuthorIndex => html!( <AuthorIndex /> ),
      Route::PostIndex => html!( <PostIndex /> ),
      Route::Post { id } => html!( <BlogPost id={id} /> ),
   };
}

fn main() {
   wasm_logger::init(Config::new(Level::Trace));
   Renderer::<App>::new().render();
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
   #[at("/posts/:id")]
   Post { id: String },
   #[at("/posts")]
   PostIndex,
   #[at("/authors/:id")]
   Author { id: String },
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
   self::{components::App, pages::*},
   log::Level,
   router::prelude::*,
   wasm_logger::Config,
   yew::{prelude::*, Renderer},
};

mod components;
mod error;
mod pages;
mod types;
mod utils;

extern crate chrono;
extern crate dotenv_codegen;
extern crate gloo;
#[macro_use]
extern crate lazy_static;
extern crate log;
extern crate parking_lot;
extern crate pulldown_cmark as cmark;
extern crate reqwasm;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate wasm_logger;
extern crate web_sys as web;
extern crate ybc;
extern crate yew;
extern crate yew_hooks;
extern crate yew_router as router;
