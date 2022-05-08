use crate::ewmh::ewmh::Connection;

pub(crate) trait EwmhRequest: EwmhRequestData {
    type Cookie;
    type CookieUnchecked;

    fn send_request(&self, con: &Connection) -> Self::Cookie;
    fn send_request_unchecked(&self, con: &Connection) -> Self::CookieUnchecked;
}

pub(crate) trait EwmhRequestData {
    fn get_request_data(&self, con: &Connection) -> xcb::x::GetProperty;
}

pub(crate) trait EwmhCookie {
    type Reply: From<xcb::x::GetPropertyReply>;
    fn reply(self, con: &Connection) -> Self::Reply;
}

pub(crate) trait EwmhCookieUnchecked {
    type Reply;
    fn reply(self, con: &Connection) -> Option<Self::Reply>;
}

#[macro_export]
macro_rules! request {
    ($req: ident, $cookie: ident, $cookie_unchecked: ident, $reply: ident) => {
        pub struct $req {}
        pub struct $cookie {
            cookie: xcb::x::GetPropertyCookie,
        }
        pub struct $cookie_unchecked {
            cookie: xcb::x::GetPropertyCookieUnchecked,
        }

        impl EwmhRequest for $req {
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

#[macro_export]
macro_rules! root_request {
    ($req: ident, $prop: ident, $type: ident, $cookie: ident, $cookie_unchecked: ident, $reply: ident) => {
        request! {$req, $cookie, $cookie_unchecked, $reply}

        impl EwmhRequestData for $req {
            fn get_request_data(&self, con: &Connection) -> xcb::x::GetProperty {
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
