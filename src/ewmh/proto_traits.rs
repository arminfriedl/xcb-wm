#![macro_use]

use crate::ewmh::ewmh::Connection;

pub(crate) trait EwmhRequest<'a>: EwmhRequestData<'a> {
    type Cookie;
    type CookieUnchecked;

    fn send_request(&self, con: &Connection) -> Self::Cookie;
    fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked;
}

pub(crate) trait EwmhRequestData<'a> {
    type Request: xcb::Request;
    fn get_request_data(&'a self, con: &Connection) -> Self::Request;
}

pub(crate) trait EwmhCookie {
    type Reply: From<xcb::x::GetPropertyReply>;
    fn reply(self, con: &Connection) -> Self::Reply;
}

pub(crate) trait EwmhCookieUnchecked {
    type Reply;
    fn reply(self, con: &Connection) -> Option<Self::Reply>;
}

macro_rules! ewmh_get_request {
    ($req: ident, $cookie: ident, $cookie_unchecked: ident, $reply: ident) => {
        pub struct $req {}

        pub struct $cookie {
            cookie: xcb::x::GetPropertyCookie,
        }
        pub struct $cookie_unchecked {
            cookie: xcb::x::GetPropertyCookieUnchecked,
        }

        impl $req {
            pub fn new() -> $req {
                $req {}
            }
        }

        impl<'a> EwmhRequest<'a> for $req {
            type Cookie = $cookie;
            type CookieUnchecked = $cookie_unchecked;

            fn send_request(&self, con: &Connection) -> Self::Cookie {
                $cookie {
                    cookie: con.con.send_request(&self.get_request_data(con)),
                }
            }

            fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked {
                $cookie_unchecked {
                    cookie: con.con.send_request_unchecked(&self.get_request_data(con)),
                }
            }
        }

        impl EwmhCookie for $cookie {
            type Reply = $reply;

            fn reply(self, con: &Connection) -> Self::Reply {
                let reply = con.con.wait_for_reply(self.cookie).unwrap();
                reply.into()
            }
        }

        impl EwmhCookieUnchecked for $cookie_unchecked {
            type Reply = $reply;

            fn reply(self, con: &Connection) -> Option<Self::Reply> {
                con.con
                    .wait_for_reply_unchecked(self.cookie)
                    .unwrap()
                    .map(|reply| reply.into())
            }
        }
    };
}

macro_rules! ewmh_get_root_request {
    ($req: ident, $prop: ident, UTF8_STRING, $cookie: ident, $cookie_unchecked: ident, $reply: ident) => {
        ewmh_get_request! {$req, $cookie, $cookie_unchecked, $reply}

        impl<'a> EwmhRequestData<'a> for $req {
            type Request = xcb::x::GetProperty;

            fn get_request_data(&'a self, con: &Connection) -> Self::Request {
                xcb::x::GetProperty {
                    delete: false,
                    window: con.con.get_setup().roots().next().unwrap().root(),
                    property: con.atoms.$prop,
                    r#type: con.atoms.UTF8_STRING,
                    long_offset: 0,
                    long_length: u32::MAX,
                }
            }
        }
    };

    ($req: ident, $prop: ident, $type: ident, $cookie: ident, $cookie_unchecked: ident, $reply: ident) => {
        ewmh_get_request! {$req, $cookie, $cookie_unchecked, $reply}

        impl<'a> EwmhRequestData<'a> for $req {
            type Request = xcb::x::GetProperty;

            fn get_request_data(&'a self, con: &Connection) -> Self::Request {
                xcb::x::GetProperty {
                    delete: false,
                    window: con.con.get_setup().roots().next().unwrap().root(),
                    property: con.atoms.$prop,
                    r#type: xcb::x::$type,
                    long_offset: 0,
                    long_length: u32::MAX,
                }
            }
        }
    };
}

macro_rules! ewmh_set_root_request {
    ($req: ident, $prop: ident, UTF8_STRING) => {
        impl<'a> EwmhRequest<'a> for $req {
            type Cookie = xcb::VoidCookieChecked;
            type CookieUnchecked = xcb::VoidCookie;

            fn send_request(&self, con: &Connection) -> Self::Cookie {
                con.con.send_request_checked(&self.get_request_data(con))
            }

            fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked {
                con.con.send_request(&self.get_request_data(con))
            }
        }

        impl<'a> EwmhRequestData<'a> for $req {
            type Request = xcb::x::ChangeProperty<'a, u8>;

            fn get_request_data(&'a self, con: &Connection) -> Self::Request {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: con.con.get_setup().roots().next().unwrap().root(),
                    property: con.atoms.$prop,
                    r#type: con.atoms.UTF8_STRING,
                    data: self.data.as_slice(),
                }
            }
        }
    };

    ($req: ident, $prop: ident, $type: ident) => {
        impl<'a> EwmhRequest<'a> for $req {
            type Cookie = xcb::VoidCookieChecked;
            type CookieUnchecked = xcb::VoidCookie;

            fn send_request(&self, con: &Connection) -> Self::Cookie {
                con.con.send_request_checked(&self.get_request_data(con))
            }

            fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked {
                con.con.send_request(&self.get_request_data(con))
            }
        }

        impl<'a> EwmhRequestData<'a> for $req {
            type Request = xcb::x::ChangeProperty<'a, u32>;

            fn get_request_data(&'a self, con: &Connection) -> Self::Request {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: con.con.get_setup().roots().next().unwrap().root(),
                    property: con.atoms.$prop,
                    r#type: xcb::x::$type,
                    data: &self.data,
                }
            }
        }
    };
}
