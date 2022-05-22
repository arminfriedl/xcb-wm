//! Root Window Properties (and Related Messages)
//!
//! see: <https://specifications.freedesktop.org/wm-spec/1.5/ar01s03.html#idm45539547193552>

use xcb::{Xid, XidNew};

use crate::ewmh::proto::util::{strings_to_x_buffer, x_buffer_to_strings};
use crate::ewmh::traits::*;
use crate::ewmh::Connection;

use paste::paste; // Needed for macros

// _NET_SUPPORTED, ATOM[]/32
// {{{
#[derive(Debug)]
pub struct GetSupportedReply {
    pub atoms: Vec<xcb::x::Atom>,
}

impl From<xcb::x::GetPropertyReply> for GetSupportedReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetSupportedReply {
            atoms: reply.value().into(),
        }
    }
}

ewmh_get_property! {
    request=GetSupported{
        window: root,
        property: _NET_SUPPORTED,
        xtype: ATOM_ATOM
    },
    reply=GetSupportedReply
}
// }}}

// _NET_CLIENT_LIST, WINDOW[]/32
// {{{
#[derive(Debug)]
pub struct GetClientListReply {
    pub clients: Vec<xcb::x::Window>,
}

impl From<xcb::x::GetPropertyReply> for GetClientListReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetClientListReply {
            clients: reply.value().into(),
        }
    }
}

ewmh_get_property! {
    request=GetClientList{
        window: root,
        property: _NET_CLIENT_LIST,
        xtype: ATOM_WINDOW
    },
    reply=GetClientListReply
}
// }}}

// _NET_CLIENT_LIST_STACKING, WINDOW[]/32
// {{{
#[derive(Debug)]
pub struct GetClientListStackingReply {
    pub clients: Vec<xcb::x::Window>,
}

impl From<xcb::x::GetPropertyReply> for GetClientListStackingReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetClientListStackingReply {
            clients: reply.value().into(),
        }
    }
}

ewmh_get_property! {
    request=GetClientListStacking{
        window: root,
        property: _NET_CLIENT_LIST_STACKING,
        xtype: ATOM_WINDOW
    },
    reply=GetClientListStackingReply
}
// }}}

// _NET_NUMBER_OF_DESKTOPS, CARDINAL/32
// {{{
#[derive(Debug)]
pub struct GetNumberOfDesktopsReply {
    pub desktops: u32,
}

impl From<xcb::x::GetPropertyReply> for GetNumberOfDesktopsReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNumberOfDesktopsReply {
            desktops: reply.value::<u32>()[0],
        }
    }
}

ewmh_get_property! {
    request=GetNumberOfDesktops{
        window: root,
        property: _NET_NUMBER_OF_DESKTOPS,
        xtype: ATOM_CARDINAL
    },
    reply=GetNumberOfDesktopsReply
}

pub struct SetNumberOfDesktops {
    client_message: xcb::x::ClientMessageEvent,
}

impl SetNumberOfDesktops {
    pub fn new(connection: &Connection, desktops: u32) -> SetNumberOfDesktops {
        SetNumberOfDesktops {
            client_message: xcb::x::ClientMessageEvent::new(
                connection.con.get_setup().roots().next().unwrap().root(),
                connection.atoms._NET_NUMBER_OF_DESKTOPS,
                xcb::x::ClientMessageData::Data32([desktops, 0x00, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=SetNumberOfDesktops{destination: root}
}
// }}}

// _NET_DESKTOP_GEOMETRY width, height, CARDINAL[2]/32
// {{{
#[derive(Debug)]
pub struct GetDesktopGeometryReply {
    pub width: u32,
    pub height: u32,
}

impl From<xcb::x::GetPropertyReply> for GetDesktopGeometryReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetDesktopGeometryReply {
            width: reply.value::<u32>()[0],
            height: reply.value::<u32>()[1],
        }
    }
}

ewmh_get_property! {
    request=GetDesktopGeometry{
        window: root,
        property: _NET_DESKTOP_GEOMETRY,
        xtype: ATOM_CARDINAL
    },
    reply=GetDesktopGeometryReply
}

pub struct SetDesktopGeometry {
    client_message: xcb::x::ClientMessageEvent,
}

impl SetDesktopGeometry {
    pub fn new(connection: &Connection, width: u32, height: u32) -> SetDesktopGeometry {
        SetDesktopGeometry {
            client_message: xcb::x::ClientMessageEvent::new(
                connection.con.get_setup().roots().next().unwrap().root(),
                connection.atoms._NET_DESKTOP_GEOMETRY,
                xcb::x::ClientMessageData::Data32([width, height, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=SetDesktopGeometry{destination: root}
}

// }}}

// _NET_DESTKOP_VIEWPORT x, y, CARDINAL[][2]/32
// {{{
#[derive(Debug)]
pub struct GetDesktopViewportReply {
    pub x: u32,
    pub y: u32,
}

impl From<xcb::x::GetPropertyReply> for GetDesktopViewportReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetDesktopViewportReply {
            x: reply.value::<u32>()[0],
            y: reply.value::<u32>()[1],
        }
    }
}

ewmh_get_property! {
    request=GetDesktopViewport{
        window: root,
        property: _NET_DESKTOP_VIEWPORT,
        xtype: ATOM_CARDINAL
    },
    reply=GetDesktopViewportReply
}

pub struct SetDesktopViewport {
    client_message: xcb::x::ClientMessageEvent,
}

impl SetDesktopViewport {
    pub fn new(connection: &Connection, x: u32, y: u32) -> SetDesktopViewport {
        SetDesktopViewport {
            client_message: xcb::x::ClientMessageEvent::new(
                connection.con.get_setup().roots().next().unwrap().root(),
                connection.atoms._NET_DESKTOP_VIEWPORT,
                xcb::x::ClientMessageData::Data32([x, y, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=SetDesktopViewport{destination: root}
}
// }}}

// _NET_CURRENT_DESKTOP desktop, CARDINAL/32
// {{{
#[derive(Debug)]
pub struct GetCurrentDesktopReply {
    pub desktop: u32,
}

impl From<xcb::x::GetPropertyReply> for GetCurrentDesktopReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetCurrentDesktopReply {
            desktop: reply.value::<u32>()[0],
        }
    }
}

ewmh_get_property! {
    request=GetCurrentDesktop{
        window: root,
        property: _NET_CURRENT_DESKTOP,
        xtype: ATOM_CARDINAL
    },
    reply=GetCurrentDesktopReply
}

pub struct SetCurrentDesktop {
    client_message: xcb::x::ClientMessageEvent,
}

impl SetCurrentDesktop {
    pub fn new(connection: &Connection, desktop: u32) -> SetCurrentDesktop {
        SetCurrentDesktop {
            client_message: xcb::x::ClientMessageEvent::new(
                connection.con.get_setup().roots().next().unwrap().root(),
                connection.atoms._NET_CURRENT_DESKTOP,
                xcb::x::ClientMessageData::Data32([desktop, 0x00, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=SetCurrentDesktop{destination: root}
}
// }}}

// _NET_DESKTOP_NAMES desktop, UTF8_STRING[]
// {{{
#[derive(Debug)]
pub struct GetDesktopNamesReply {
    pub names: Vec<String>,
}

impl From<xcb::x::GetPropertyReply> for GetDesktopNamesReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetDesktopNamesReply {
            names: x_buffer_to_strings(reply.value::<u8>()),
        }
    }
}

ewmh_get_property! {
    request=GetDesktopNames{
        window: root,
        property: _NET_DESKTOP_NAMES,
        xtype: UTF8_STRING
    },
    reply=GetDesktopNamesReply
}

pub struct SetDesktopNames {
    data: Vec<u8>,
}

impl SetDesktopNames {
    pub fn new(names: Vec<&str>) -> SetDesktopNames {
        SetDesktopNames {
            data: strings_to_x_buffer(names),
        }
    }
}

ewmh_set_property! {
    request=SetDesktopNames{
        window: root,
        property: _NET_DESKTOP_NAMES,
        xtype: UTF8_STRING
    }
}
// }}}

// _NET_ACTIVE_WINDOW, WINDOW/32
// {{{
#[derive(Debug)]
pub struct GetActiveWindowReply {
    pub window: xcb::x::Window,
}

impl From<xcb::x::GetPropertyReply> for GetActiveWindowReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetActiveWindowReply {
            window: unsafe { xcb::x::Window::new(reply.value::<u32>()[0]) },
        }
    }
}

ewmh_get_property! {
    request=GetActiveWindow{
        window: root,
        property: _NET_ACTIVE_WINDOW,
        xtype: ATOM_WINDOW
    },
    reply=GetActiveWindowReply
}

pub struct SetActiveWindow {
    client_message: xcb::x::ClientMessageEvent,
}

impl SetActiveWindow {
    pub fn new(
        connection: &Connection,
        window: xcb::x::Window,
        source_indication: u32,
        timestamp: u32,
        requestor_window: Option<xcb::x::Window>,
    ) -> SetActiveWindow {
        SetActiveWindow {
            client_message: xcb::x::ClientMessageEvent::new(
                window,
                connection.atoms._NET_ACTIVE_WINDOW,
                xcb::x::ClientMessageData::Data32([
                    source_indication,
                    timestamp,
                    requestor_window.map_or(0, |w| w.resource_id()),
                    0x00,
                    0x00,
                ]),
            ),
        }
    }
}

ewmh_client_message! {
    request=SetActiveWindow{destination: root}
}
// }}}

// // _NET_WORKAREA, x, y, width, height, CARDINAL[][4]/32
// {{{
#[derive(Debug)]
pub struct GetWorkareaReply {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl From<xcb::x::GetPropertyReply> for GetWorkareaReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetWorkareaReply {
            x: reply.value::<u32>()[0],
            y: reply.value::<u32>()[1],
            width: reply.value::<u32>()[2],
            height: reply.value::<u32>()[3],
        }
    }
}

ewmh_get_property! {
    request=GetWorkarea{
        window: root,
        property: _NET_WORKAREA,
        xtype: ATOM_CARDINAL
    },
    reply=GetWorkareaReply
}
// }}}

// // _NET_SUPPORTING_WM_CHECK, WINDOW/32
// {{{
#[derive(Debug)]
pub struct GetSupportingWmCheckReply {
    pub window: xcb::x::Window,
}

impl From<xcb::x::GetPropertyReply> for GetSupportingWmCheckReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetSupportingWmCheckReply {
            window: unsafe { xcb::x::Window::new(reply.value::<u32>()[0]) },
        }
    }
}

ewmh_get_property! {
    request=GetSupportingWmCheck{
        window: root,
        property: _NET_SUPPORTING_WM_CHECK,
        xtype: ATOM_WINDOW
    },
    reply=GetSupportingWmCheckReply
}
// }}}

// _NET_VIRTUAL_ROOTS, WINDOW/32
// {{{
#[derive(Debug)]
pub struct GetVirtualRootsReply {
    pub window: xcb::x::Window,
}

impl From<xcb::x::GetPropertyReply> for GetVirtualRootsReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetVirtualRootsReply {
            window: unsafe { xcb::x::Window::new(reply.value::<u32>()[0]) },
        }
    }
}

ewmh_get_property! {
    request=GetVirtualRoots{
        window: root,
        property: _NET_VIRTUAL_ROOTS,
        xtype: ATOM_WINDOW
    },
    reply=GetVirtualRootsReply
}
// }}}

// _NET_DESKTOP_LAYOUT, orientation, columns, rows, starting_corner, CARDINAL[4]/32
// {{{
#[derive(Debug)]
pub struct DesktopLayoutReply {
    pub orientation: u32,
    pub columns: u32,
    pub rows: u32,
    pub starting_corner: u32,
}

impl From<xcb::x::GetPropertyReply> for DesktopLayoutReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        DesktopLayoutReply {
            orientation: reply.value::<u32>()[0],
            columns: reply.value::<u32>()[1],
            rows: reply.value::<u32>()[2],
            starting_corner: reply.value::<u32>()[3],
        }
    }
}

ewmh_get_property! {
    request=DesktopLayout{
        window: root,
        property: _NET_DESKTOP_LAYOUT,
        xtype: ATOM_CARDINAL
    },
    reply=DesktopLayoutReply
}
// }}}

// _NET_SHOWING_DESKTOP desktop, CARDINAL/32
// {{{
#[derive(Debug)]
pub struct GetShowingDesktopReply {
    pub is_showing_desktop: bool,
}

impl From<xcb::x::GetPropertyReply> for GetShowingDesktopReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetShowingDesktopReply {
            is_showing_desktop: { reply.value::<u32>()[0] == 1 },
        }
    }
}

ewmh_get_property! {
    request=GetShowingDesktop{
        window: root,
        property: _NET_SHOWING_DESKTOP,
        xtype: ATOM_CARDINAL
    },
    reply=GetShowingDesktopReply
}

pub struct SetShowingDesktop {
    client_message: xcb::x::ClientMessageEvent,
}

impl SetShowingDesktop {
    pub fn new(connection: &Connection, show_desktop: bool) -> SetShowingDesktop {
        let data = if show_desktop { 1 } else { 0 };

        SetShowingDesktop {
            client_message: xcb::x::ClientMessageEvent::new(
                connection.con.get_setup().roots().next().unwrap().root(),
                connection.atoms._NET_SHOWING_DESKTOP,
                xcb::x::ClientMessageData::Data32([data, 0x00, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=SetShowingDesktop{destination: root}
}
// }}}

// _NET_CLOSE_WINDOW
// {{{
pub struct CloseWindow {
    client_message: xcb::x::ClientMessageEvent,
}

impl CloseWindow {
    pub fn new(
        connection: &Connection,
        window: xcb::x::Window,
        source_indication: u32,
        timestamp: u32,
    ) -> CloseWindow {
        CloseWindow {
            client_message: xcb::x::ClientMessageEvent::new(
                window,
                connection.atoms._NET_CLOSE_WINDOW,
                xcb::x::ClientMessageData::Data32([timestamp, source_indication, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=CloseWindow{destination: root}
}

// // _NET_MOVERESIZE_WINDOW
// // {{{
//
// // TODO
//
// // }}}
//
// // _NET_WM_MOVERESIZE
// // {{{
//
// // TODO
//
// // }}}
//
// // _NET_RESTACK_WINDOW
// // {{{
//
// // TODO
//
// // }}}
//
// _NET_REQUEST_FRAME_EXTENTS
// {{{
pub struct RequestFrameExtents {
    client_message: xcb::x::ClientMessageEvent,
}

impl RequestFrameExtents {
    pub fn new(connection: &Connection, window: xcb::x::Window) -> RequestFrameExtents {
        RequestFrameExtents {
            client_message: xcb::x::ClientMessageEvent::new(
                window,
                connection.atoms._NET_REQUEST_FRAME_EXTENTS,
                xcb::x::ClientMessageData::Data32([0x00, 0x00, 0x00, 0x00, 0x00]),
            ),
        }
    }
}

ewmh_client_message! {
    request=RequestFrameExtents{destination: root}
}
// }}}
