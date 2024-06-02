#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod chat;
pub mod config;
pub mod ctx;
pub mod data;
pub mod error;
#[cfg(feature = "mock")]
pub mod mock;
pub mod model;
pub mod msg;
pub mod req;
