//! Mappings of objects received from the API, with optional helper methods for
//! ease of use.
//!
//! Models can optionally have additional helper methods compiled, by enabling
//! the `model` feature.
//!
//! Normally you can import models through the sub-modules:
//!
//! ```rust,no_run
//! use serenity::model::channel::{ChannelType, GuildChannel, Message};
//! use serenity::model::id::{ChannelId, GuildId};
//! use serenity::model::user::User;
//! ```
//!
//! This can get a bit tedious - especially with a large number of imports - so
//! this can be simplified by simply glob importing everything from the prelude:
//!
//! ```rust,no_run
//! use serenity::model::prelude::*;
//! ```

#[macro_use]
mod utils;

pub mod application;
pub mod channel;
pub mod error;
pub mod event;
pub mod gateway;
pub mod guild;
pub mod id;
pub mod invite;
pub mod misc;
pub mod permissions;
pub mod prelude;
pub mod user;
pub mod voice;
pub mod webhook;

pub use self::error::Error as ModelError;
pub use self::permissions::Permissions;

use client::Client;
use internal::prelude::*;
use self::utils::*;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::result::Result as StdResult;

#[cfg(feature = "utils")]
use utils::Colour;

type WrappedClient = Option<Rc<Client>>;
