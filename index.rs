//! MBP2: A utility library for federated apps.
//!
//! This library includes various tools for building federated applications.
//! Building the Fediverse is an important undertaking, which is why I built
//! this library to support my blogging network application and others.
//!
//! The [`crypto`] module contains small functions to randomly generate secret keys
//! to decrypt them using a provided cipher.
//!
//! The [`gql`] module contains functions for building dynamic GraphQL schemas and
//! setting up GraphQL API servers.
//!
//! The [`service`] module (currently under construction) is a planned API that will allow
//! construction of proper Service APIs, much like those within the `tower` ecosystem.
//!
//! Have a look around for yourself to determine whether this utility library fits your
//! needs, and I hope that it proves as useful for you as it does for me and my projects.
//! ~Az
//!
//! You may contact the author of this project on Mastodon, Discord, or on Matrix:
//! - https://elk.zone/mas.to/@zub
//! - @azyklus
//! - azyklus:mozilla.org
#![doc=include_str!("./README.md")]
#![doc(html_root_url="https://mbp2.blog/src/foundation/latest")]
#![allow(non_snake_case)]

/// Contains API definitions for GraphQL types.
pub mod api;

/// A placeholder module for asset-bundling functionality.
pub mod assets;

/// This module contains cryptography functionality, such as
/// key generation for secrets, JSON web token utilities, and
/// other useful helpers.
#[cfg(feature="crypto")]
pub mod crypto;

/// GraphQL-specific functions live here, such as GET and POST requests,
/// Subscription helpers, and contextual helpers.
#[cfg(any(feature="async-gql", feature="juniper-gql"))]
pub mod gql;

/// Models for custom API definitions.
pub mod models;

/// A service API, similar to that of the `tower` ecosystem.
/// Currently a placeholder for a future implementation.
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
