//! Request and response definitions for the ewmh protocol
//!
//! # Example
//! ```
//! let connection = xcb_wm::ewmh::Connection();
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

pub(crate) mod util;

mod root_props;
pub use root_props::*;

mod application_props;
pub use application_props::*;
