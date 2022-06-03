//! Client Properties
//!
//! see: <https://www.x.org/releases/X11R7.7/doc/xorg-docs/icccm/icccm.html#Client_Properties>

#![allow(dead_code)]

use bitflags::bitflags;
use std::convert::TryInto;
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

// WM_NORMAL_HINTS,
// {{{

bitflags! {
    struct WmSizeHintsFlags: u32 {
        const Empty       = 0b0000_0000_0000;
        const USPosition  = 0b0000_0000_0001;
        const USSize      = 0b0000_0000_0010;
        const PPosition   = 0b0000_0000_0100;
        const PSize       = 0b0000_0000_1000;
        const PMinSize    = 0b0000_0001_0000;
        const PMaxSize    = 0b0000_0010_0000;
        const PResizeInc  = 0b0000_0100_0000;
        const PAspect     = 0b0000_1000_0000;
        const PBaseSize   = 0b0001_0000_0000;
        const PWinGravity = 0b0010_0000_0000;
    }
}

#[derive(Debug)]
pub struct WmSizeHints {
    flags: WmSizeHintsFlags,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    min_width: u32,
    min_height: u32,
    max_width: u32,
    max_height: u32,
    width_inc: u32,
    height_inc: u32,
    min_aspect: (u32, u32),
    max_aspect: (u32, u32),
    base_width: u32,
    base_height: u32,
    win_gravity: xcb::x::Gravity,
}

impl Default for WmSizeHints {
    fn default() -> Self {
        WmSizeHints {
            flags: WmSizeHintsFlags::Empty,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            width_inc: 0,
            height_inc: 0,
            min_aspect: (0, 0),
            max_aspect: (0, 0),
            base_width: 0,
            base_height: 0,
            win_gravity: xcb::x::Gravity::NorthWest,
        }
    }
}

impl WmSizeHints {
    pub fn as_data(&mut self) -> Vec<u32> {
        vec![
            self.flags.bits,
            self.x,
            self.y,
            self.width,
            self.height,
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
            self.width_inc,
            self.height_inc,
            self.min_aspect.0,
            self.min_aspect.1,
            self.max_aspect.0,
            self.max_aspect.1,
            self.base_width,
            self.base_height,
            self.win_gravity as u32,
        ]
    }

    pub fn from_reply(reply: xcb::x::GetPropertyReply) -> WmSizeHints {
        let packed_vals = reply.value::<u32>();
        WmSizeHints {
            flags: WmSizeHintsFlags::from_bits_truncate(packed_vals[0]),
            x: packed_vals[1],
            y: packed_vals[2],
            width: packed_vals[3],
            height: packed_vals[4],
            min_width: packed_vals[5],
            min_height: packed_vals[6],
            max_width: packed_vals[7],
            max_height: packed_vals[8],
            width_inc: packed_vals[9],
            height_inc: packed_vals[10],
            min_aspect: (packed_vals[11], packed_vals[12]),
            max_aspect: (packed_vals[13], packed_vals[14]),
            base_width: packed_vals[15],
            base_height: packed_vals[16],
            win_gravity: unsafe { std::mem::transmute(packed_vals[17]) },
        }
    }

    pub fn position(&mut self, user_specified: bool, x: u32, y: u32) {
        // reset
        self.flags &= !(WmSizeHintsFlags::USPosition | WmSizeHintsFlags::PPosition);

        if user_specified {
            self.flags |= WmSizeHintsFlags::USPosition
        } else {
            self.flags |= WmSizeHintsFlags::PPosition
        }

        self.x = x;
        self.y = y;
    }

    pub fn size(&mut self, user_specified: bool, width: u32, height: u32) {
        // reset
        self.flags &= !(WmSizeHintsFlags::USSize | WmSizeHintsFlags::PSize);

        if user_specified {
            self.flags |= WmSizeHintsFlags::USSize
        } else {
            self.flags |= WmSizeHintsFlags::PSize
        }

        self.width = width;
        self.height = height;
    }

    pub fn min_size(&mut self, min_width: u32, min_height: u32) {
        self.flags |= WmSizeHintsFlags::PMinSize;

        self.min_width = min_width;
        self.min_height = min_height;
    }

    pub fn max_size(&mut self, max_width: u32, max_height: u32) {
        self.flags |= WmSizeHintsFlags::PMaxSize;

        self.max_width = max_width;
        self.max_height = max_height;
    }

    pub fn resize_inc(&mut self, width_inc: u32, height_inc: u32) {
        self.flags |= WmSizeHintsFlags::PResizeInc;

        self.width_inc = width_inc;
        self.height_inc = height_inc;
    }

    pub fn aspect(&mut self, min_aspect: (u32, u32), max_aspect: (u32, u32)) {
        self.flags |= WmSizeHintsFlags::PAspect;

        self.min_aspect = min_aspect;
        self.max_aspect = max_aspect;
    }

    pub fn base_size(&mut self, base_width: u32, base_height: u32) {
        self.flags |= WmSizeHintsFlags::PBaseSize;

        self.base_width = base_width;
        self.base_height = base_height;
    }

    pub fn win_gravity(&mut self, win_gravity: xcb::x::Gravity) {
        self.flags |= WmSizeHintsFlags::PWinGravity;

        self.win_gravity = win_gravity;
    }
}

#[derive(Debug)]
pub struct GetWmNormalHintsReply {
    pub size_hints: WmSizeHints,
}

impl From<xcb::x::GetPropertyReply> for GetWmNormalHintsReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWmNormalHintsReply {
            size_hints: WmSizeHints::from_reply(reply),
        }
    }
}

icccm_get_property! {
    request=GetWmNormalHints{
        window: client,
        property: ATOM_WM_NORMAL_HINTS,
        xtype: ATOM_WM_SIZE_HINTS
    },
    reply=GetWmNormalHintsReply
}

pub struct SetWmNormalHints {
    window: xcb::x::Window,
    data: Vec<u32>,
}

impl SetWmNormalHints {
    pub fn new(window: xcb::x::Window, size_hints: &mut WmSizeHints) -> SetWmNormalHints {
        SetWmNormalHints {
            window: window,
            data: size_hints.as_data(),
        }
    }
}

icccm_set_hint_property! {
    request=SetWmNormalHints{
        property: ATOM_WM_NORMAL_HINTS,
        xtype: ATOM_WM_SIZE_HINTS
    }
}
// }}}
