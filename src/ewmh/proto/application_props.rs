//! Application Window Properties
//!
//! see: <https://specifications.freedesktop.org/wm-spec/wm-spec-1.5.html#idm45446104426048>

use xcb::{Xid, XidNew};

use crate::ewmh::connection::Connection;
use crate::ewmh::proto_traits::{EwmhCookie, EwmhCookieUnchecked, EwmhRequest, EwmhRequestData};

// _NET_WM_NAME, UTF8_STRING
// {{{
ewmh_get_window_request! {
    GetWmName,
    _NET_WM_NAME,
    UTF8_STRING,
    GetWmNameCookie,
    GetWmNameCookieUnchecked,
    GetWmNameReply
}

#[derive(Debug)]
pub struct GetWmNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let mut buf = vec![];

        for b in reply.value::<u8>() {
            if *b != 0x00 {
                buf.push(*b)
            }
        }

        GetWmNameReply {
            name: String::from_utf8(buf).unwrap(),
        }
    }
}
// }}}

// _NET_WM_VISIBLE_NAME, UTF8_STRING
// {{{
ewmh_get_window_request! {
    GetWmVisibleName,
    _NET_WM_VISIBLE_NAME,
    UTF8_STRING,
    GetWmVisibleNameCooke,
    GetWmVisibleNameCookieUnchecked,
    GetWmVisibleNameReply
}

#[derive(Debug)]
pub struct GetWmVisibleNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmVisibleNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let mut buf = vec![];

        for b in reply.value::<u8>() {
            if *b != 0x00 {
                buf.push(*b)
            }
        }

        GetWmVisibleNameReply {
            name: String::from_utf8(buf).unwrap(),
        }
    }
}
// }}}

// _NET_WM_ICON_NAME, UTF8_STRING
// {{{
ewmh_get_window_request! {
    GetWmIconName,
    _NET_WM_ICON_NAME,
    UTF8_STRING,
    GetWmIconNameCooke,
    GetWmIconNameCookieUnchecked,
    GetWmIconNameReply
}

#[derive(Debug)]
pub struct GetWmIconNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmIconNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let mut buf = vec![];

        for b in reply.value::<u8>() {
            if *b != 0x00 {
                buf.push(*b)
            }
        }

        GetWmIconNameReply {
            name: String::from_utf8(buf).unwrap(),
        }
    }
}
// }}}

// _NET_WM_VISIBLE_ICON_NAME, UTF8_STRING
// {{{
ewmh_get_window_request! {
    GetWmVisibleIconName,
    _NET_WM_VISIBLE_ICON_NAME,
    UTF8_STRING,
    GetWmVisibleIconNameCooke,
    GetWmVisibleIconNameCookieUnchecked,
    GetWmVisibleIconNameReply
}

#[derive(Debug)]
pub struct GetWmVisibleIconNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmVisibleIconNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let mut buf = vec![];

        for b in reply.value::<u8>() {
            if *b != 0x00 {
                buf.push(*b)
            }
        }

        GetWmVisibleIconNameReply {
            name: String::from_utf8(buf).unwrap(),
        }
    }
}
// }}}

// _NET_WM_DESKTOP, CARDINAL/32
// {{{
ewmh_get_window_request! {
    GetWmDesktop,
    _NET_WM_DESKTOP,
    CARDINAL,
    GetWmDesktopCooke,
    GetWmDesktopCookieUnchecked,
    GetWmDesktopReply
}

#[derive(Debug)]
pub struct GetWmDesktopReply {
    pub desktop: u32,
}

impl From<xcb::x::GetPropertyReply> for GetWmDesktopReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmDesktopReply {
            desktop: reply.value::<u32>()[0],
        }
    }
}
// }}}

// _NET_WM_WINDOW_TYPE, ATOM[]/32
// {{{

pub struct SetWmWindowType {
    window: xcb::x::Window,
    data: Vec<xcb::x::Atom>,
}

impl SetWmWindowType {
    pub fn new(window: xcb::x::Window, types: Vec<xcb::x::Atom>) -> SetWmWindowType {
        SetWmWindowType {
            window,
            data: types,
        }
    }
}

ewmh_set_window_request! {
    SetWmWindowType,
    _NET_WM_WINDOW_TYPE,
    { xcb::x::ATOM_ATOM }
}

ewmh_get_window_request! {
    GetWmWindowType,
    _NET_WM_WINDOW_TYPE,
    ATOM,
    GetWmWindowTypeCooke,
    GetWmWindowTypeCookieUnchecked,
    GetWmWindowTypeReply
}

#[derive(Debug)]
pub struct GetWmWindowTypeReply {
    pub types: Vec<xcb::x::Atom>,
}

impl From<xcb::x::GetPropertyReply> for GetWmWindowTypeReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmWindowTypeReply {
            types: reply.value::<xcb::x::Atom>().into(),
        }
    }
}

// }}}

// _NET_WM_STATE, ATOM[]/32
// {{{

pub struct SetWmState {
    window: xcb::x::Window,
    data: Vec<xcb::x::Atom>,
}

impl SetWmState {
    pub fn new(window: xcb::x::Window, types: Vec<xcb::x::Atom>) -> SetWmState {
        SetWmState {
            window,
            data: types,
        }
    }
}

ewmh_set_window_request! {
    SetWmState,
    _NET_WM_STATE,
    { xcb::x::ATOM_ATOM }
}

ewmh_get_window_request! {
    GetWmState,
    _NET_WM_STATE,
    ATOM,
    GetWmStateCooke,
    GetWmStateCookieUnchecked,
    GetWmStateReply
}

#[derive(Debug)]
pub struct GetWmStateReply {
    pub types: Vec<xcb::x::Atom>,
}

impl From<xcb::x::GetPropertyReply> for GetWmStateReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmStateReply {
            types: reply.value::<xcb::x::Atom>().into(),
        }
    }
}

// }}}
