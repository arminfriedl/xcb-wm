//! see: <https://specifications.freedesktop.org/wm-spec/1.5/ar01s03.html#idm45539547193552>

#[macro_use]
mod traits;

mod atoms;
mod connection;

pub use atoms::Atoms;
pub use connection::Connection;

pub mod proto;
pub use proto::*;
