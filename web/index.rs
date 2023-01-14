pub fn main() {
   wasm_logger::init(Config::new(Level::Trace));
   Renderer::<App>::new().render();
}

use {
   self::components::App,
   log::Level,
   wasm_logger::Config,
   yew::Renderer,
};

mod components;

extern crate log;
extern crate pulldown_cmark as cmark;
extern crate reqwasm;
extern crate serde;
extern crate wasm_logger;
extern crate web_sys as web;
extern crate yew;
