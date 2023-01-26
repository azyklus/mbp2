#![allow(non_snake_case)]

pub fn main() {
   wasm_logger::init(Config::new(Level::Trace));
   Renderer::<App>::new().render();
}

pub enum Msg {
   ToggleNav,
}

use {
   self::components::App,
   log::Level,
   wasm_logger::Config,
   yew::Renderer,
};

mod components;
mod pages;

extern crate log;
extern crate pulldown_cmark as cmark;
extern crate reqwasm;
#[macro_use]
extern crate serde;
extern crate wasm_logger;
extern crate web_sys as web;
extern crate yew;
