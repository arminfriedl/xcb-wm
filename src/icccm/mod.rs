//! Request and response definitions for the icccm protocol
//!
//! see: <https://www.x.org/releases/X11R7.7/doc/xorg-docs/icccm/icccm.html>

mod traits;

mod atoms;

mod connection;
pub use connection::Connection;

pub mod proto;
