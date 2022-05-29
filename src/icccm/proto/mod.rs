//! Request and response definitions for the icccm protocol
//!
//! # Example
//! ```
//! let connection = xcb_wm::icccm::Connection();
//! let request = xcb_wm::ewmh::proto::GetSupported;
//! let cookie = connection.send_request(request);
//! let response = connection.wait_for_reply(cookie);
//! ```

// mod application_props;
// #[allow(unused)]
// mod root_props;
//
// pub use application_props::*;
// pub use root_props::*;
#[macro_use]
mod macros;

mod client_props;
pub use client_props::*;
