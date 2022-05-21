//! Root Window Properties (and Related Messages)
//!
//! see: <https://specifications.freedesktop.org/wm-spec/1.5/ar01s03.html#idm45539547193552>

use xcb::{Xid, XidNew};

use crate::ewmh::proto::util::{strings_to_x_buffer, x_buffer_to_strings};
use crate::ewmh::traits::*;
use crate::ewmh::Connection;

// _NET_SUPPORTED, ATOM[]/32
// {{{
pub struct GetSupported;

pub struct GetSupportedCookie(xcb::x::GetPropertyCookie);

pub struct GetSupportedCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_SUPPORTED,
        xtype: ATOM_ATOM
    },
    reply=GetSupportedReply,
    cookie=GetSupportedCookie,
    cookie_unchecked=GetSupportedCookieUnchecked
}
// }}}

// _NET_CLIENT_LIST, WINDOW[]/32
// {{{
pub struct GetClientList;

pub struct GetClientListCookie(xcb::x::GetPropertyCookie);

pub struct GetClientListCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_CLIENT_LIST,
        xtype: ATOM_WINDOW
    },
    reply=GetClientListReply,
    cookie=GetClientListCookie,
    cookie_unchecked=GetClientListCookieUnchecked
}
// }}}

// _NET_CLIENT_LIST_STACKING, WINDOW[]/32
// {{{
pub struct GetClientListStacking;

pub struct GetClientListStackingCookie(xcb::x::GetPropertyCookie);

pub struct GetClientListStackingCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_CLIENT_LIST_STACKING,
        xtype: ATOM_WINDOW
    },
    reply=GetClientListStackingReply,
    cookie=GetClientListStackingCookie,
    cookie_unchecked=GetClientListStackingCookieUnchecked
}
// }}}

// _NET_NUMBER_OF_DESKTOPS, CARDINAL/32
// {{{
pub struct GetNumberOfDesktops;

pub struct GetNumberOfDesktopsCookie(xcb::x::GetPropertyCookie);

pub struct GetNumberOfDesktopsCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_NUMBER_OF_DESKTOPS,
        xtype: ATOM_CARDINAL
    },
    reply=GetNumberOfDesktopsReply,
    cookie=GetNumberOfDesktopsCookie,
    cookie_unchecked=GetNumberOfDesktopsCookieUnchecked
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
    request=SetNumberOfDesktops{destination: root_window}
}
// }}}

// _NET_DESKTOP_GEOMETRY width, height, CARDINAL[2]/32
// {{{
pub struct GetDesktopGeometry;

pub struct GetDesktopGeometryCookie(xcb::x::GetPropertyCookie);

pub struct GetDesktopGeometryCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_DESKTOP_GEOMETRY,
        xtype: ATOM_CARDINAL
    },
    reply=GetDesktopGeometryReply,
    cookie=GetDesktopGeometryCookie,
    cookie_unchecked=GetDesktopGeometryCookieUnchecked
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
    request=SetDesktopGeometry{destination: root_window}
}

// }}}

// _NET_DESTKOP_VIEWPORT x, y, CARDINAL[][2]/32
// {{{
pub struct GetDesktopViewport;

pub struct GetDesktopViewportCookie(xcb::x::GetPropertyCookie);

pub struct GetDesktopViewportCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_DESKTOP_VIEWPORT,
        xtype: ATOM_CARDINAL
    },
    reply=GetDesktopViewportReply,
    cookie=GetDesktopViewportCookie,
    cookie_unchecked=GetDesktopViewportCookieUnchecked
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
    request=SetDesktopViewport{destination: root_window}
}
// }}}

// _NET_CURRENT_DESKTOP desktop, CARDINAL/32
// {{{
pub struct GetCurrentDesktop;

pub struct GetCurrentDesktopCookie(xcb::x::GetPropertyCookie);

pub struct GetCurrentDesktopCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_CURRENT_DESKTOP,
        xtype: ATOM_CARDINAL
    },
    reply=GetCurrentDesktopReply,
    cookie=GetCurrentDesktopCookie,
    cookie_unchecked=GetCurrentDesktopCookieUnchecked
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
    request=SetCurrentDesktop{destination: root_window}
}
// }}}

// _NET_DESKTOP_NAMES desktop, UTF8_STRING[]
// {{{
pub struct GetDesktopNames;

pub struct GetDesktopNamesCookie(xcb::x::GetPropertyCookie);

pub struct GetDesktopNamesCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_DESKTOP_NAMES,
        xtype: UTF8_STRING
    },
    reply=GetDesktopNamesReply,
    cookie=GetDesktopNamesCookie,
    cookie_unchecked=GetDesktopNamesCookieUnchecked
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
        window: root_window,
        property: _NET_DESKTOP_NAMES,
        xtype: UTF8_STRING
    }
}
// }}}

// _NET_ACTIVE_WINDOW, WINDOW/32
// {{{
pub struct GetActiveWindow;

pub struct GetActiveWindowCookie(xcb::x::GetPropertyCookie);

pub struct GetActiveWindowCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_ACTIVE_WINDOW,
        xtype: ATOM_WINDOW
    },
    reply=GetActiveWindowReply,
    cookie=GetActiveWindowCookie,
    cookie_unchecked=GetActiveWindowCookieUnchecked
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
    request=SetActiveWindow{destination: root_window}
}
// }}}

// // _NET_WORKAREA, x, y, width, height, CARDINAL[][4]/32
// {{{

pub struct GetWorkarea;

pub struct GetWorkareaCookie(xcb::x::GetPropertyCookie);

pub struct GetWorkareaCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_WORKAREA,
        xtype: ATOM_CARDINAL
    },
    reply=GetWorkareaReply,
    cookie=GetWorkareaCookie,
    cookie_unchecked=GetWorkareaCookieUnchecked
}

// }}}

// // _NET_SUPPORTING_WM_CHECK, WINDOW/32
// {{{
pub struct GetSupportingWmCheck;

pub struct GetSupportingWmCheckCookie(xcb::x::GetPropertyCookie);

pub struct GetSupportingWmCheckCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
// }}}

// _NET_VIRTUAL_ROOTS, WINDOW/32
// {{{
pub struct GetVirtualRoots;

pub struct GetVirtualRootsCookie(xcb::x::GetPropertyCookie);

pub struct GetVirtualRootsCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_VIRTUAL_ROOTS,
        xtype: ATOM_WINDOW
    },
    reply=GetVirtualRootsReply,
    cookie=GetVirtualRootsCookie,
    cookie_unchecked=GetVirtualRootsCookieUnchecked
}
// }}}

// _NET_DESKTOP_LAYOUT, orientation, columns, rows, starting_corner, CARDINAL[4]/32
// {{{
pub struct DesktopLayout;

pub struct DesktopLayoutCookie(xcb::x::GetPropertyCookie);

pub struct DesktopLayoutCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

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
        window: root_window,
        property: _NET_DESKTOP_LAYOUT,
        xtype: ATOM_CARDINAL
    },
    reply=DesktopLayoutReply,
    cookie=DesktopLayoutCookie,
    cookie_unchecked=DesktopLayoutCookieUnchecked
}
// }}}

// _NET_SHOWING_DESKTOP desktop, CARDINAL/32
// {{{
pub struct GetShowingDesktop;

pub struct GetShowingDesktopCookie(xcb::x::GetPropertyCookie);

pub struct GetShowingDesktopCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

#[derive(Debug)]
pub struct GetShowingDesktopReply {
    pub is_showing_desktop: bool,
}

impl From<xcb::x::GetPropertyReply> for GetShowingDesktopReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetShowingDesktopReply {
            is_showing_desktop: if reply.value::<u32>()[0] == 1 {
                true
            } else {
                false
            },
        }
    }
}

ewmh_get_property! {
    request=GetShowingDesktop{
        window: root_window,
        property: _NET_SHOWING_DESKTOP,
        xtype: ATOM_CARDINAL
    },
    reply=GetShowingDesktopReply,
    cookie=GetShowingDesktopCookie,
    cookie_unchecked=GetShowingDesktopCookieUnchecked
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
    request=SetShowingDesktop{destination: root_window}
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
    request=CloseWindow{destination: root_window}
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
    request=RequestFrameExtents{destination: root_window}
}
// }}}
