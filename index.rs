#![doc(html_root_url="https://mbp2.blog/src/foundation/latest")]
#![allow(non_snake_case)]

pub mod api;
pub mod assets;
#[cfg(feature="crypto")]
pub mod crypto;
#[cfg(any(feature="async-gql", feature="juniper-gql"))]
pub mod gql;
pub mod models;
pub mod service;

#[cfg(all(feature="juniper-gql", feature="gql-subs"))]
extern crate actix;
#[cfg(all(feature="juniper-gql", feature="gql-subs"))]
extern crate actix_rt;
#[macro_export]
#[cfg(feature="juniper-gql")]
extern crate actix_web;
#[cfg(all(feature="juniper-gql", feature="gql-subs"))]
extern crate actix_web_actors;
#[cfg(feature="crypto")]
extern crate aes;
#[macro_use]
#[cfg(feature="async-gql")]
extern crate async_graphql;
#[cfg(feature="crypto")]
extern crate cbc;
#[macro_use]
extern crate common_macros as macros;
#[cfg(feature="crypto")]
extern crate hex;
#[cfg(feature="juniper-gql")]
extern crate juniper;
#[cfg(all(feature="juniper-gql", feature="gql-subs"))]
extern crate juniper_graphql_ws;
#[cfg(feature="crypto")]
extern crate rand;
#[cfg(feature="async-gql")]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate tokio_util;
extern crate ulid;
