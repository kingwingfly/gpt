//! Tools for interacting with the OpenAI chatGPT API.
//! The config includes the endpoint and api_key, which can be saved to the keyring.
//! Usage example is [here](https://github.com/kingwingfly/gpt).
//! Having an openAI mock server, stream the chat.
//! The keyring will also work on mock mode. (Needs `mock` feature enabled)

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
