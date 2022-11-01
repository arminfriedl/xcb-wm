//! Application Window Properties
//!
//! see: <https://specifications.freedesktop.org/wm-spec/wm-spec-1.5.html#idm45446104426048>

#![allow(dead_code)]

use xcb::Xid;

use crate::ewmh::proto::util::{strings_to_x_buffer, x_buffer_to_strings};
use crate::ewmh::traits::*;
use crate::ewmh::Connection;

use paste::paste; // Needed for macros

// _NET_WM_NAME, UTF8_STRING
// {{{
#[derive(Debug)]
pub struct GetWmNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmNameReply {
            name: x_buffer_to_strings(reply.value::<u8>())[0].to_owned(),
        }
    }
}

ewmh_get_property! {
    request=GetWmName{
        window: client,
        property: _NET_WM_NAME,
        xtype: UTF8_STRING
    },
    reply=GetWmNameReply
}

pub struct SetWmName {
    window: xcb::x::Window,
    data: Vec<u8>,
}

impl SetWmName {
    pub fn new(window: xcb::x::Window, name: &str) -> SetWmName {
        SetWmName {
            window,
            data: strings_to_x_buffer(vec![name]),
        }
    }
}

ewmh_set_property! {
    request=SetWmName{
        window: client,
        property: _NET_WM_NAME,
        xtype: UTF8_STRING
    }
}

// }}}

// _NET_WM_VISIBLE_NAME, UTF8_STRING
// {{{
#[derive(Debug)]
pub struct GetWmVisibleNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmVisibleNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmVisibleNameReply {
            name: x_buffer_to_strings(reply.value::<u8>())[0].to_owned(),
        }
    }
}

ewmh_get_property! {
    request=GetWmVisibleName{
        window: client,
        property: _NET_WM_VISIBLE_NAME,
        xtype: UTF8_STRING
    },
    reply=GetWmVisibleNameReply
}
// }}}

// _NET_WM_ICON_NAME, UTF8_STRING
// {{{
#[derive(Debug)]
pub struct GetWmIconNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmIconNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmIconNameReply {
            name: x_buffer_to_strings(reply.value::<u8>())[0].to_owned(),
        }
    }
}

ewmh_get_property! {
    request=GetWmIconName{
        window: client,
        property: _NET_WM_ICON_NAME,
        xtype: UTF8_STRING
    },
    reply=GetWmIconNameReply
}
// }}}

// _NET_WM_VISIBLE_ICON_NAME, UTF8_STRING
// {{{
#[derive(Debug)]
pub struct GetWmVisibleIconNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmVisibleIconNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmVisibleIconNameReply {
            name: x_buffer_to_strings(reply.value::<u8>())[0].to_owned(),
        }
    }
}

ewmh_get_property! {
    request=GetWmVisibleIconName{
        window: client,
        property: _NET_WM_VISIBLE_ICON_NAME,
        xtype: UTF8_STRING
    },
    reply=GetWmVisibleIconNameReply
}
// }}}

// _NET_WM_DESKTOP, CARDINAL/32
// {{{
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

ewmh_get_property! {
    request=GetWmDesktop{
        window: client,
        property: _NET_WM_DESKTOP,
        xtype: ATOM_CARDINAL
    },
    reply=GetWmDesktopReply
}

pub struct SetWmDesktop {
    window: xcb::x::Window,
    data: Vec<u32>,
}

impl SetWmDesktop {
    pub fn new(window: xcb::x::Window, desktop: u32) -> SetWmDesktop {
        SetWmDesktop {
            window,
            data: vec![desktop],
        }
    }
}

ewmh_set_property! {
    request=SetWmDesktop {
        window: client,
        property: _NET_WM_DESKTOP,
        xtype: ATOM_CARDINAL
    }
}

pub struct SendWmDesktop {
    client_message: xcb::x::ClientMessageEvent,
}

impl SendWmDesktop {
    pub fn new(connection: &Connection, desktop: u32, source_indication: u32) -> SendWmDesktop {
        SendWmDesktop {
            client_message: xcb::x::ClientMessageEvent::new(
                connection.con.get_setup().roots().next().unwrap().root(),
                connection.atoms._NET_WM_DESKTOP,
                xcb::x::ClientMessageData::Data32([desktop, source_indication, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=SendWmDesktop{destination: root}
}
// }}}

// _NET_WM_WINDOW_TYPE, ATOM[]/32
// {{{
#[derive(Debug)]
pub struct GetWmWindowTypeReply {
    pub window_types: Vec<xcb::x::Atom>,
}

impl From<xcb::x::GetPropertyReply> for GetWmWindowTypeReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmWindowTypeReply {
            window_types: reply.value::<xcb::x::Atom>().into(),
        }
    }
}

ewmh_get_property! {
    request=GetWmWindowType{
        window: client,
        property: _NET_WM_WINDOW_TYPE,
        xtype: ATOM_ATOM
    },
    reply=GetWmWindowTypeReply
}

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

ewmh_set_property! {
    request=SetWmWindowType{
        window: client,
        property: _NET_WM_WINDOW_TYPE,
        xtype: ATOM_ATOM
    }
}

// }}}

// _NET_WM_STATE, ATOM[]/32
// {{{
#[derive(Debug)]
pub struct GetWmStateReply {
    pub states: Vec<xcb::x::Atom>,
}

impl From<xcb::x::GetPropertyReply> for GetWmStateReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmStateReply {
            states: reply.value::<xcb::x::Atom>().into(),
        }
    }
}

ewmh_get_property! {
    request=GetWmState{
        window: client,
        property: _NET_WM_STATE,
        xtype: ATOM_ATOM
    },
    reply=GetWmStateReply
}

pub struct SendWmState {
    client_message: xcb::x::ClientMessageEvent,
}

impl SendWmState {
    pub fn new(
        connection: &Connection,
        window: xcb::x::Window,
        action: xcb::x::PropMode,
        states: [xcb::x::Atom; 2],
        source_indication: u32,
    ) -> SendWmState {
        let data = [
            unsafe { std::mem::transmute::<_, u32>(action) },
            states[0].resource_id(),
            states[1].resource_id(),
            source_indication,
            0x00,
        ];

        SendWmState {
            client_message: xcb::x::ClientMessageEvent::new(
                window,
                connection.atoms._NET_WM_DESKTOP,
                xcb::x::ClientMessageData::Data32(data),
            ),
        }
    }
}

ewmh_client_message! {
    request=SendWmState{destination: root}
}
// }}}

// _NET_WM_USER_TIME CARDINAL/32
// {{{
#[derive(Debug)]
pub struct GetWmUserTimeReply {
    pub time: u32,
}

impl From<xcb::x::GetPropertyReply> for GetWmUserTimeReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        Self {
            time: reply.value::<u32>()[0],
        }
    }
}

ewmh_get_property! {
    request=GetWmUserTime{
        window: client,
        property: _NET_WM_USER_TIME,
        xtype: ATOM_CARDINAL
    },
    reply=GetWmUserTimeReply
}
// }}}
