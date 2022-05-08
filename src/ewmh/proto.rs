use crate::ewmh::ewmh::Connection;
use crate::ewmh::proto_traits::{EwmhCookie, EwmhCookieUnchecked, EwmhRequest, EwmhRequestData};
use crate::{request, root_request};

root_request! {
    GetNetSupportedRequest,
    _NET_SUPPORTED,
    ATOM_ATOM,
    GetNetSupportedCookie,
    GetNetSupportedCookieUnchecked,
    GetNetSupportedReply
}

#[derive(Debug)]
pub struct GetNetSupportedReply {
    pub value: Vec<xcb::x::Atom>,
}

impl From<xcb::x::GetPropertyReply> for GetNetSupportedReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetSupportedReply {
            value: reply.value().into(),
        }
    }
}

root_request! {
    GetNetClientListRequest,
    _NET_CLIENT_LIST,
    ATOM_WINDOW,
    GetNetClientListCookie,
    GetNetClientListCookieUnchecked,
    GetNetClientListReply
}

#[derive(Debug)]
pub struct GetNetClientListReply {
    pub value: Vec<xcb::x::Window>,
}

impl From<xcb::x::GetPropertyReply> for GetNetClientListReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetClientListReply {
            value: reply.value().into(),
        }
    }
}

root_request! {
    GetNetClientListStackingRequest,
    _NET_CLIENT_LIST_STACKING,
    ATOM_WINDOW,
    GetNetClientListStackingCookie,
    GetNetClientListStackingCookieUnchecked,
    GetNetClientListStackingReply
}

#[derive(Debug)]
pub struct GetNetClientListStackingReply {
    pub value: Vec<xcb::x::Window>,
}

impl From<xcb::x::GetPropertyReply> for GetNetClientListStackingReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetClientListStackingReply {
            value: reply.value().into(),
        }
    }
}

root_request! {
    GetNetNumberOfDesktopsRequest,
    _NET_NUMBER_OF_DESKTOPS,
    ATOM_CARDINAL,
    GetNetNumberOfDesktopsCookie,
    GetNetNumberOfDesktopsCookieUnchecked,
    GetNetNumberOfDesktopsReply
}

#[derive(Debug)]
pub struct GetNetNumberOfDesktopsReply {
    pub value: u32,
}

impl From<xcb::x::GetPropertyReply> for GetNetNumberOfDesktopsReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetNumberOfDesktopsReply {
            value: reply.value::<u32>()[0],
        }
    }
}

root_request! {
    GetNetDesktopGeometryRequest,
    _NET_DESKTOP_GEOMETRY,
    ATOM_CARDINAL,
    GetNetDesktopGeometryCookie,
    GetNetDesktopGeometryCookieUnchecked,
    GetNetDesktopGeometryReply
}

#[derive(Debug)]
pub struct GetNetDesktopGeometryReply {
    pub width: u32,
    pub height: u32,
}

impl From<xcb::x::GetPropertyReply> for GetNetDesktopGeometryReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetDesktopGeometryReply {
            width: reply.value::<u32>()[0],
            height: reply.value::<u32>()[1],
        }
    }
}

root_request! {
    GetNetDesktopViewportRequest,
    _NET_DESKTOP_VIEWPORT,
    ATOM_CARDINAL,
    GetNetDesktopViewportCookie,
    GetNetDesktopViewportCookieUnchecked,
    GetNetDesktopViewportReply
}

#[derive(Debug)]
pub struct GetNetDesktopViewportReply {
    pub x: u32,
    pub y: u32,
}

impl From<xcb::x::GetPropertyReply> for GetNetDesktopViewportReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetDesktopViewportReply {
            x: reply.value::<u32>()[0],
            y: reply.value::<u32>()[1],
        }
    }
}

root_request! {
    GetNetCurrentDesktopRequest,
    _NET_CURRENT_DESKTOP,
    ATOM_CARDINAL,
    GetNetCurrentDesktopCookie,
    GetNetCurrentDesktopCookieUnchecked,
    GetNetCurrentDesktopReply
}

#[derive(Debug)]
pub struct GetNetCurrentDesktopReply {
    pub desktop: u32,
}

impl From<xcb::x::GetPropertyReply> for GetNetCurrentDesktopReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetCurrentDesktopReply {
            desktop: reply.value::<u32>()[0],
        }
    }
}

request! {
    GetNetDesktopNamesRequest,
    GetNetDesktopNamesCookie,
    GetNetDesktopNamesCookieUnchecked,
    GetNetDesktopNamesReply
}

#[derive(Debug)]
pub struct GetNetDesktopNamesReply {
    pub value: Vec<String>,
}

impl EwmhRequestData for GetNetDesktopNamesRequest {
    fn get_request_data(&self, con: &Connection) -> xcb::x::GetProperty {
        xcb::x::GetProperty {
            delete: false,
            window: con.con.get_setup().roots().next().unwrap().root(),
            property: con.atoms._NET_DESKTOP_NAMES,
            r#type: con.atoms.UTF8_STRING,
            long_offset: 0,
            long_length: u32::MAX,
        }
    }
}

impl From<xcb::x::GetPropertyReply> for GetNetDesktopNamesReply {
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

        GetNetDesktopNamesReply { value: vals }
    }
}

root_request! {
    GetNetActiveWindowRequest,
    _NET_ACTIVE_WINDOW,
    ATOM_WINDOW,
    GetNetActiveWindowCookie,
    GetNetActiveWindowCookieUnchecked,
    GetNetActiveWindowReply
}

#[derive(Debug)]
pub struct GetNetActiveWindowReply {
    pub value: xcb::x::Window,
}

impl From<xcb::x::GetPropertyReply> for GetNetActiveWindowReply {
    fn from(reply: xcb::x::GetPropertyReply) -> Self {
        GetNetActiveWindowReply {
            value: reply.value::<xcb::x::Window>()[0],
        }
    }
}
