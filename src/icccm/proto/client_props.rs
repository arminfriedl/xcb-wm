//! Client Properties
//!
//! see: <https://www.x.org/releases/X11R7.7/doc/xorg-docs/icccm/icccm.html#Client_Properties>

#![allow(dead_code)]

use xcb::{Xid, XidNew};

use crate::icccm::traits::*;
use crate::icccm::Connection;

use paste::paste; // Needed for macros

// WM_NAME, TEXT
// {{{
#[derive(Debug)]
pub struct GetWmNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let x = reply.value::<u8>().into();
        GetWmNameReply {
            // TODO Compound string
            name: String::from_utf8(x).unwrap(),
        }
    }
}

icccm_get_property! {
    request=GetWmName{
        window: client,
        property: ATOM_WM_NAME,
        xtype: ATOM_ANY
    },
    reply=GetWmNameReply
}

pub struct SetWmName<T: xcb::x::PropEl> {
    window: xcb::x::Window,
    encoding: xcb::x::Atom,
    data: Vec<T>,
}

impl<T: xcb::x::PropEl> SetWmName<T> {
    pub fn new(window: xcb::x::Window, encoding: xcb::x::Atom, name: Vec<T>) -> SetWmName<T> {
        SetWmName {
            window: window,
            encoding: encoding,
            data: name,
        }
    }
}

icccm_set_text_property! {
    request=SetWmName{
        property: ATOM_WM_NAME
    }
}
// }}}

// WM_ICON_NAME, TEXT
// {{{
#[derive(Debug)]
pub struct GetWmIconNameReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmIconNameReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let x = reply.value::<u8>().into();
        GetWmIconNameReply {
            // TODO Compound string
            name: String::from_utf8(x).unwrap(),
        }
    }
}

icccm_get_property! {
    request=GetWmIconName{
        window: client,
        property: ATOM_WM_ICON_NAME,
        xtype: ATOM_ANY
    },
    reply=GetWmIconNameReply
}

pub struct SetWmIconName<T: xcb::x::PropEl> {
    window: xcb::x::Window,
    encoding: xcb::x::Atom,
    data: Vec<T>,
}

impl<T: xcb::x::PropEl> SetWmIconName<T> {
    pub fn new(window: xcb::x::Window, encoding: xcb::x::Atom, name: Vec<T>) -> SetWmIconName<T> {
        SetWmIconName {
            window: window,
            encoding: encoding,
            data: name,
        }
    }
}

icccm_set_text_property! {
    request=SetWmIconName{
        property: ATOM_WM_ICON_NAME
    }
}
// }}}

// WM_COLORMAP_WINDOWS, WINDOW[]/32
// {{{
#[derive(Debug)]
pub struct GetWmColorMapWindowsReply {
    pub windows: Vec<xcb::x::Window>,
}

impl From<xcb::x::GetPropertyReply> for GetWmColorMapWindowsReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmColorMapWindowsReply {
            windows: reply.value().into(),
        }
    }
}

icccm_get_property! {
    request=GetWmColorMapWindows {
        window: client,
        property: WM_COLORMAP_WINDOWS,
        xtype: ATOM_WINDOW
    },
    reply=GetWmIconNameReply
}

pub struct SetWmColorMapWindows {
    window: xcb::x::Window,
    data: Vec<xcb::x::Window>,
}

impl SetWmColorMapWindows {
    pub fn new(
        window: xcb::x::Window,
        colormap_windows: Vec<xcb::x::Window>,
    ) -> SetWmColorMapWindows {
        SetWmColorMapWindows {
            window: window,
            data: colormap_windows,
        }
    }
}

icccm_set_property! {
    request=SetWmColorMapWindows{
        property: con.WM_COLORMAP_WINDOWS,
        xtype: ATOM_WINDOW
    }
}
// }}}

// WM_CLIENT_MACHINE, TEXT
// {{{
#[derive(Debug)]
pub struct GetWmClientMachineReply {
    pub name: String,
}

impl From<xcb::x::GetPropertyReply> for GetWmClientMachineReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let x = reply.value::<u8>().into();
        GetWmClientMachineReply {
            // TODO Compound string
            name: String::from_utf8(x).unwrap(),
        }
    }
}

icccm_get_property! {
    request=GetWmClientMachine{
        window: client,
        property: ATOM_WM_CLIENT_MACHINE,
        xtype: ATOM_ANY
    },
    reply=GetWmClientMachineReply
}

pub struct SetWmClientMachine<T: xcb::x::PropEl> {
    window: xcb::x::Window,
    encoding: xcb::x::Atom,
    data: Vec<T>,
}

impl<T: xcb::x::PropEl> SetWmClientMachine<T> {
    pub fn new(
        window: xcb::x::Window,
        encoding: xcb::x::Atom,
        name: Vec<T>,
    ) -> SetWmClientMachine<T> {
        SetWmClientMachine {
            window: window,
            encoding: encoding,
            data: name,
        }
    }
}

icccm_set_text_property! {
    request=SetWmClientMachine{
        property: ATOM_WM_CLIENT_MACHINE
    }
}
// }}}

// WM_TRANSIENT_FOR, WINDOW/32
// {{{
#[derive(Debug)]
pub struct GetWmTransientForReply {
    pub window: xcb::x::Window,
}

impl From<xcb::x::GetPropertyReply> for GetWmTransientForReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmTransientForReply {
            window: unsafe { xcb::x::Window::new(reply.value::<u32>()[0]) },
        }
    }
}

icccm_get_property! {
    request=GetWmTransientFor{
        window: client,
        property: ATOM_WM_TRANSIENT_FOR,
        xtype: ATOM_WINDOW
    },
    reply=GetWmTransientForReply
}

pub struct SetWmTransientFor {
    window: xcb::x::Window,
    data: Vec<xcb::x::Window>,
}

impl SetWmTransientFor {
    // TODO better name for second window
    pub fn new(window: xcb::x::Window, window2: xcb::x::Window) -> SetWmTransientFor {
        SetWmTransientFor {
            window: window,
            data: vec![window2],
        }
    }
}

icccm_set_property! {
    request=SetWmTransientFor{
        property: ATOM_WM_CLIENT_MACHINE,
        xtype: ATOM_WINDOW
    }
}
// }}}
