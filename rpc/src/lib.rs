//! Tendermint RPC definitions and types.
//!
//! ## Client
//!
//! This crate optionally provides access to various levels of client
//! functionality and transports.
//!
//! By only specifying the `client` feature flag, all you get access to is the
//! [`Client`] trait. Additional feature flags for access to different aspects
//! of the client and different transports are as follows:
//!
//! | Features | Description |
//! | -------- | ----------- |
//! | `client`, `transport_http` | Provides [`HttpClient`], which is a basic RPC client that interacts with remote Tendermint nodes via **JSON-RPC over HTTP**. This client does not provide [`Event`] subscription functionality. See the [Tendermint RPC] for more details. |
//! | `client`, `subscription`, `transport_websocket` | Provides [`WebSocketClient`], which provides [`Event`] subscription functionality over a WebSocket connection. See the [`/subscribe` endpoint] in the Tendermint RPC for more details. |
//! | `client`, `transport_mock` | Provides [`MockClient`], which only implements [`Client`] (no subscription functionality) for aiding in testing your application that depends on the Tendermint RPC client. |
//! | `client`, `subscription`, `transport_mock` | In addition to [`MockClient`], this will give you access to [`MockSubscriptionClient`], which helps you simulate subscriptions to events being generated by a Tendermint node. |
//!
//! [`Client`]: trait.Client.html
//! [`HttpClient`]: struct.HttpClient.html
//! [`Event`]: event/struct.Event.html
//! [`WebSocketClient`]: struct.WebSocketClient.html
//! [Tendermint RPC]: https://docs.tendermint.com/master/rpc/
//! [`/subscribe` endpoint]: https://docs.tendermint.com/master/rpc/#/Websocket/subscribe
//! [`MockClient`]: struct.MockClient.html
//! [`MockSubscriptionClient`]: struct.MockSubscriptionClient.html

#[cfg(feature = "client")]
mod client;
#[cfg(all(feature = "client", feature = "transport_http"))]
pub use client::HttpClient;
#[cfg(all(
    feature = "client",
    feature = "subscription",
    feature = "transport_mock"
))]
pub use client::MockSubscriptionClient;
#[cfg(all(
    feature = "client",
    feature = "subscription",
    feature = "transport_websocket"
))]
pub use client::WebSocketClient;
#[cfg(feature = "client")]
pub use client::{Client, ClosableClient};
#[cfg(all(feature = "client", feature = "transport_mock"))]
pub use client::{MockClient, MockRequestMatcher, MockRequestMethodMatcher};
#[cfg(all(feature = "client", feature = "subscription"))]
pub use client::{Subscription, SubscriptionClient, SubscriptionId};

pub mod endpoint;
pub mod error;
pub mod event;
mod id;
mod method;
pub mod request;
pub mod response;
mod result;
mod version;

pub use self::{
    error::Error, id::Id, method::Method, request::Request, response::Response, result::Result,
    version::Version,
};
