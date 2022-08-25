# xcb-wm

The long lost Rust implementation of the
[icccm](https://tronche.com/gui/x/icccm/) and
[ewmh](https://specifications.freedesktop.org/wm-spec/wm-spec-1.5.html)
extensions for the X Window System protocol.

xcb-wm provides type safe and Rust-native abstractions. It simplifies the usage
of window manager extensions to X11. 

xcb-wm sits on top of [rust-xcb](https://github.com/rust-x-bindings/rust-xcb)
similar to how [libxcb-wm](https://gitlab.freedesktop.org/xorg/lib/libxcb-wm)
sits on top of [libxcb](https://gitlab.freedesktop.org/xorg/lib/libxcb). If you
are already using rust-xcb you are also familiar with xcb-wm. The public APIs
and general usage are intentionally close.

xcb-wm works with rust-xcb 1.x and later.

## Usage
Add this to your Cargo.toml:

```toml
[dependencies]
xcb-wm = "0.4.0"
```

Each request is either a `Get*`, a `Set*` or a `Send*` struct. `Get*` structs
can be used to get ewmh or iccm properties. `Set*` structs can be used to set
properties. `Send*` structs can be used to send client messages. You can read up
on the protocol definitions for more details but in general every property has a
corresponding `Get*` request. `Set*` requests are mostly useful _before_ a
window is mapped. `Send*` requests for everything else.

Each request can be sent either checked or unchecked. This is typesafe by
special cookies for each of them. You get the request cookie by calling
`send_request`/`send_request_unchecked`.

You can retrieve a reply and wrap it into a high level and meaningful Rust
struct by calling `wait_for_reply`/`wait_for_reply_unchecked` on the cookie. For
requests that don't have a reply (i.e. `Set*` and `Send*` requests) you can use
`check_request` to check for errors.

## Examples

Get the names of available desktops:

``` rust
use xcb;
use xcb_wm::ewmh;

// Create a `rust-xcb` connection
let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;

// Wrap the connection in an `xcb-wm::ewmh` connection for ewmh extensions.
// 
// Note that this does not take ownership of the `rust-xcb` connection
// so you can continue to use other xcb functionality with the same
// connection.
let ewmh_con = ewmh::Connection::connect(&xcb_con);

// Create a request for the _NET_DESKTOP_NAMES property
let request = ewmh::proto::GetDesktopNames;
let cookie = ewmh_con.send_request(&request);

// Get a `GetDesktopNamesReply` reply
//
// Replies are automatically de-serialized into meaningful Rust structs. You
// take full ownership of the reply struct.
let reply = ewmh_con.wait_for_reply(cookie).unwrap();

// All replies implement `Debug` so you can also print them
println!("{:?}", reply);
```
