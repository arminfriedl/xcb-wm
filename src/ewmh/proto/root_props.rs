//! Root Window Properties (and Related Messages)
//!
//! see: https://specifications.freedesktop.org/wm-spec/1.5/ar01s03.html#idm45539547193552

use xcb::{Xid, XidNew};

use crate::ewmh::ewmh::Connection;
use crate::ewmh::proto_traits::{EwmhCookie, EwmhCookieUnchecked, EwmhRequest, EwmhRequestData};

// _NET_SUPPORTED, ATOM[]/32
// {{{
ewmh_get_root_request! {
    GetSupported,
    _NET_SUPPORTED,
    ATOM_ATOM,
    GetSupportedCookie,
    GetSupportedCookieUnchecked,
    GetSupportedReply
}

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
// }}}

// _NET_CLIENT_LIST, WINDOW[]/32
// {{{
ewmh_get_root_request! {
    GetClientList,
    _NET_CLIENT_LIST,
    ATOM_WINDOW,
    GetClientListCookie,
    GetClientListCookieUnchecked,
    GetClientListReply
}

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
// }}}

// _NET_CLIENT_LIST_STACKING, WINDOW[]/32
// {{{
ewmh_get_root_request! {
    GetClientListStacking,
    _NET_CLIENT_LIST_STACKING,
    ATOM_WINDOW,
    GetClientListStackingCookie,
    GetClientListStackingCookieUnchecked,
    GetClientListStackingReply
}

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
// }}}

// _NET_NUMBER_OF_DESKTOPS, CARDINAL/32
// {{{
ewmh_get_root_request! {
    GetNumberOfDesktops,
    _NET_NUMBER_OF_DESKTOPS,
    ATOM_CARDINAL,
    GetNumberOfDesktopsCookie,
    GetNumberOfDesktopsCookieUnchecked,
    GetNumberOfDesktopsReply
}

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

pub struct SetNumberOfDesktops {
    data: [u32; 1],
}

impl SetNumberOfDesktops {
    pub fn new(desktops: u32) -> SetNumberOfDesktops {
        SetNumberOfDesktops { data: [desktops] }
    }
}

ewmh_set_root_request! {
    SetNumberOfDesktops,
    _NET_NUMBER_OF_DESKTOPS,
    ATOM_CARDINAL
}

// }}}

// _NET_DESKTOP_GEOMETRY width, height, CARDINAL[2]/32
// {{{
ewmh_get_root_request! {
    GetDesktopGeometry,
    _NET_DESKTOP_GEOMETRY,
    ATOM_CARDINAL,
    GetDesktopGeometryCookie,
    GetDesktopGeometryCookieUnchecked,
    GetDesktopGeometryReply
}

#[derive(Debug)]
pub struct GetDesktopGeometryReply {
    pub width: u32,
    pub height: u32,
}

impl From<xcb::x::GetPropertyReply> for GetDesktopGeometryReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetDesktopGeometryReply {
            width: reply.value::<u32>()[0],
            height: reply.value::<u32>()[0],
        }
    }
}

pub struct SetDesktopGeometry {
    data: [u32; 2],
}

impl SetDesktopGeometry {
    pub fn new(width: u32, height: u32) -> SetDesktopGeometry {
        SetDesktopGeometry {
            data: [width, height],
        }
    }
}

ewmh_set_root_request! {
    SetDesktopGeometry,
    _NET_DESKTOP_GEOMETRY,
    ATOM_CARDINAL
}

// }}}

// _NET_DESTKOP_VIEWPORT x, y, CARDINAL[][2]/32
// {{{
ewmh_get_root_request! {
    GetDesktopViewport,
    _NET_DESKTOP_VIEWPORT,
    ATOM_CARDINAL,
    GetDesktopViewportCookie,
    GetDesktopViewportCookieUnchecked,
    GetDesktopViewportReply
}

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

pub struct SetDesktopViewport {
    data: [u32; 2],
}

impl SetDesktopViewport {
    pub fn new(x: u32, y: u32) -> SetDesktopViewport {
        SetDesktopViewport { data: [x, y] }
    }
}

ewmh_set_root_request! {
    SetDesktopViewport,
    _NET_DESKTOP_VIEWPORT,
    ATOM_CARDINAL
}

// }}}

// _NET_CURRENT_DESKTOP desktop, CARDINAL/32
// {{{
ewmh_get_root_request! {
    GetCurrentDesktop,
    _NET_CURRENT_DESKTOP,
    ATOM_CARDINAL,
    GetCurrentDesktopCookie,
    GetCurrentDesktopCookieUnchecked,
    GetCurrentDesktopReply
}

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

pub struct SetCurrentDesktop {
    data: [u32; 2],
}

impl SetCurrentDesktop {
    pub fn new(new_index: u32, timestamp: u32) -> SetCurrentDesktop {
        SetCurrentDesktop {
            data: [new_index, timestamp],
        }
    }
}

ewmh_set_root_request! {
    SetCurrentDesktop,
    _NET_CURRENT_DESKTOP,
    ATOM_CARDINAL
}

// }}}

// _NET_DESKTOP_NAMES desktop, UTF8_STRING[]
// {{{
ewmh_get_root_request! {
    GetDesktopNames,
    _NET_DESKTOP_NAMES,
    UTF8_STRING,
    GetDesktopNamesCookie,
    GetDesktopNamesCookieUnchecked,
    GetDesktopNamesReply
}

#[derive(Debug)]
pub struct GetDesktopNamesReply {
    pub desktop_names: Vec<String>,
}

impl From<xcb::x::GetPropertyReply> for GetDesktopNamesReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        let mut vals = vec![];
        let mut buf = vec![];

        for b in reply.value::<u8>() {
            if *b != 0x00 {
                buf.push(*b)
            } else if !buf.is_empty() {
                vals.push(String::from_utf8(buf.clone()).unwrap());
                buf.clear();
            } else {
                buf.clear();
            }
        }

        GetDesktopNamesReply {
            desktop_names: vals,
        }
    }
}

pub struct SetDesktopNames {
    data: Vec<u8>,
}

impl SetDesktopNames {
    pub fn new(new_names: Vec<&str>) -> SetDesktopNames {
        let mut data: Vec<u8> = vec![];

        // flatten `new_names` into a continuous array of bytes
        for name in new_names {
            let mut bname = name.as_bytes().to_owned();
            bname.push(0b00);
            data.extend(bname)
        }

        SetDesktopNames { data }
    }
}

ewmh_set_root_request! {
    SetDesktopNames,
    _NET_DESKTOP_NAMES,
    UTF8_STRING
}

// }}}

// _NET_ACTIVE_WINDOW, WINDOW/32
// {{{
ewmh_get_root_request! {
    GetActiveWindow,
    _NET_ACTIVE_WINDOW,
    ATOM_WINDOW,
    GetActiveWindowCookie,
    GetActiveWindowCookieUnchecked,
    GetActiveWindowReply
}

#[derive(Debug)]
pub struct GetActiveWindowReply {
    pub value: xcb::x::Window,
}

impl From<xcb::x::GetPropertyReply> for GetActiveWindowReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetActiveWindowReply {
            value: reply.value::<xcb::x::Window>()[0],
        }
    }
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
        let data = [
            source_indication,
            timestamp,
            requestor_window.map_or(0, |w| w.resource_id()),
            0x00,
            0x00,
        ];

        let client_message = xcb::x::ClientMessageEvent::new(
            window,
            connection.atoms._NET_ACTIVE_WINDOW,
            xcb::x::ClientMessageData::Data32(data),
        );

        SetActiveWindow { client_message }
    }
}

impl<'a> EwmhRequest<'a> for SetActiveWindow {
    type Cookie = xcb::VoidCookieChecked;
    type CookieUnchecked = xcb::VoidCookie;

    fn send_request(&self, con: &Connection) -> Self::Cookie {
        con.con.send_request_checked(&self.get_request_data(con))
    }

    fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked {
        con.con.send_request(&self.get_request_data(con))
    }
}

impl<'a> EwmhRequestData<'a> for SetActiveWindow {
    type Request = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;

    fn get_request_data(
        &'a self,
        con: &Connection,
    ) -> xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent> {
        xcb::x::SendEvent {
            propagate: false,
            destination: xcb::x::SendEventDest::Window(
                con.con.get_setup().roots().next().unwrap().root(),
            ),
            event_mask: xcb::x::EventMask::SUBSTRUCTURE_NOTIFY
                | xcb::x::EventMask::SUBSTRUCTURE_REDIRECT,
            event: &self.client_message,
        }
    }
}
// }}}

// _NET_WORKAREA, x, y, width, height, CARDINAL[][4]/32
// {{{
ewmh_get_root_request! {
    GetWorkarea,
    _NET_WORKAREA,
    ATOM_CARDINAL,
    GetWorkareaCookie,
    GetWorkareaCookieUnchecked,
    GetWorkareaReply
}

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
// }}}

// _NET_SUPPORTING_WM_CHECK, WINDOW/32
// {{{
ewmh_get_root_request! {
    GetSupportingWmCheck,
    _NET_SUPPORTING_WM_CHECK,
    ATOM_WINDOW,
    GetSupportingWmCheckCookie,
    GetSupportingWmCheckCookieUnchecked,
    GetSupportingWmCheckReply
}

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
ewmh_get_root_request! {
    GetVirtualRoots,
    _NET_VIRTUAL_ROOTS,
    ATOM_WINDOW,
    GetVirtualRootsCookie,
    GetVirtualRootsCookieUnchecked,
    GetVirtualRootsReply
}

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
// }}}

// _NET_DESKTOP_LAYOUT, orientation, columns, rows, starting_corner, CARDINAL[4]/32
// {{{
ewmh_get_root_request! {
    GetDesktopLayout,
    _NET_DESKTOP_LAYOUT,
    ATOM_CARDINAL,
    GetDesktopLayoutCookie,
    GetDesktopLayoutCookieUnchecked,
    GetDesktopLayoutReply
}

#[derive(Debug)]
pub struct GetDesktopLayoutReply {
    orientation: u32,
    columns: u32,
    rows: u32,
    starting_corner: u32,
}

impl From<xcb::x::GetPropertyReply> for GetDesktopLayoutReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetDesktopLayoutReply {
            orientation: reply.value::<u32>()[0],
            columns: reply.value::<u32>()[1],
            rows: reply.value::<u32>()[2],
            starting_corner: reply.value::<u32>()[3],
        }
    }
}
// }}}

// _NET_SHOWING_DESKTOP desktop, CARDINAL/32
// {{{
ewmh_get_root_request! {
    GetShowingDesktop,
    _NET_SHOWING_DESKTOP,
    ATOM_CARDINAL,
    GetShowingDesktopCookie,
    GetShowingDesktopCookieUnchecked,
    GetShowingDesktopReply
}

#[derive(Debug)]
pub struct GetShowingDesktopReply {
    showing_desktop: bool,
}

impl From<xcb::x::GetPropertyReply> for GetShowingDesktopReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetShowingDesktopReply {
            showing_desktop: match reply.value::<u32>()[0] {
                0 => false,
                1 => true,
                _ => unreachable!(),
            },
        }
    }
}

pub struct SetShowingDesktop {
    client_message: xcb::x::ClientMessageEvent,
}

impl SetShowingDesktop {
    pub fn new(connection: &Connection, show_desktop: bool) -> SetShowingDesktop {
        let data = match show_desktop {
            false => 0 as u32,
            true => 1 as u32,
        };

        let client_message = xcb::x::ClientMessageEvent::new(
            connection.con.get_setup().roots().next().unwrap().root(),
            connection.atoms._NET_SHOWING_DESKTOP,
            xcb::x::ClientMessageData::Data32([data, 0x00, 0x00, 0x00, 0x00]),
        );

        SetShowingDesktop { client_message }
    }
}

impl<'a> EwmhRequest<'a> for SetShowingDesktop {
    type Cookie = xcb::VoidCookieChecked;
    type CookieUnchecked = xcb::VoidCookie;

    fn send_request(&self, con: &Connection) -> Self::Cookie {
        con.con.send_request_checked(&self.get_request_data(con))
    }

    fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked {
        con.con.send_request(&self.get_request_data(con))
    }
}

impl<'a> EwmhRequestData<'a> for SetShowingDesktop {
    type Request = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;

    fn get_request_data(
        &'a self,
        con: &Connection,
    ) -> xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent> {
        xcb::x::SendEvent {
            propagate: false,
            destination: xcb::x::SendEventDest::Window(
                con.con.get_setup().roots().next().unwrap().root(),
            ),
            event_mask: xcb::x::EventMask::SUBSTRUCTURE_NOTIFY
                | xcb::x::EventMask::SUBSTRUCTURE_REDIRECT,
            event: &self.client_message,
        }
    }
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
        let data = [timestamp, source_indication, 0x00, 0x00, 0x00];

        let client_message = xcb::x::ClientMessageEvent::new(
            window,
            connection.atoms._NET_CLOSE_WINDOW,
            xcb::x::ClientMessageData::Data32(data),
        );

        CloseWindow { client_message }
    }
}

impl<'a> EwmhRequest<'a> for CloseWindow {
    type Cookie = xcb::VoidCookieChecked;
    type CookieUnchecked = xcb::VoidCookie;

    fn send_request(&self, con: &Connection) -> Self::Cookie {
        con.con.send_request_checked(&self.get_request_data(con))
    }

    fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked {
        con.con.send_request(&self.get_request_data(con))
    }
}

impl<'a> EwmhRequestData<'a> for CloseWindow {
    type Request = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;

    fn get_request_data(
        &'a self,
        con: &Connection,
    ) -> xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent> {
        xcb::x::SendEvent {
            propagate: false,
            destination: xcb::x::SendEventDest::Window(
                con.con.get_setup().roots().next().unwrap().root(),
            ),
            event_mask: xcb::x::EventMask::SUBSTRUCTURE_NOTIFY
                | xcb::x::EventMask::SUBSTRUCTURE_REDIRECT,
            event: &self.client_message,
        }
    }
}
// }}}

// _NET_MOVERESIZE_WINDOW
// {{{

// TODO

// }}}

// _NET_WM_MOVERESIZE
// {{{

// TODO

// }}}

// _NET_RESTACK_WINDOW
// {{{

// TODO

// }}}

// _NET_REQUEST_FRAME_EXTENTS
// {{{
pub struct RequestFrameExtents {
    client_message: xcb::x::ClientMessageEvent,
}

impl RequestFrameExtents {
    pub fn new(connection: &Connection, window: xcb::x::Window) -> RequestFrameExtents {
        let client_message = xcb::x::ClientMessageEvent::new(
            window,
            connection.atoms._NET_REQUEST_FRAME_EXTENTS,
            xcb::x::ClientMessageData::Data32([0x0, 0x0, 0x0, 0x0, 0x0]),
        );

        RequestFrameExtents { client_message }
    }
}

impl<'a> EwmhRequest<'a> for RequestFrameExtents {
    type Cookie = xcb::VoidCookieChecked;
    type CookieUnchecked = xcb::VoidCookie;

    fn send_request(&self, con: &Connection) -> Self::Cookie {
        con.con.send_request_checked(&self.get_request_data(con))
    }

    fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked {
        con.con.send_request(&self.get_request_data(con))
    }
}

impl<'a> EwmhRequestData<'a> for RequestFrameExtents {
    type Request = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;

    fn get_request_data(
        &'a self,
        con: &Connection,
    ) -> xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent> {
        xcb::x::SendEvent {
            propagate: false,
            destination: xcb::x::SendEventDest::Window(
                con.con.get_setup().roots().next().unwrap().root(),
            ),
            event_mask: xcb::x::EventMask::SUBSTRUCTURE_NOTIFY
                | xcb::x::EventMask::SUBSTRUCTURE_REDIRECT,
            event: &self.client_message,
        }
    }
}
// }}}
