pub(crate) mod net_supported {
    use crate::ewmh::traits::*;
    use crate::ewmh::Connection;

    pub struct GetSupported;

    pub struct GetSupportedCookie(xcb::x::GetPropertyCookie);

    pub struct GetSupportedCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

    #[derive(Debug)]
    pub struct GetSupportedReply {
        pub atoms: Vec<xcb::x::Atom>,
    }

    impl<'a> EwmhRequest<'a> for GetSupported {
        type XcbRequest = xcb::x::GetProperty;
        type EwmhCookie = GetSupportedCookie;

        fn xcb_request(&self, con: &Connection) -> Self::XcbRequest {
            xcb::x::GetProperty {
                delete: false,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms._NET_SUPPORTED,
                r#type: xcb::x::ATOM_ATOM,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }

        fn convert_cookie(&'a self, xcb_cookie: xcb::x::GetPropertyCookie) -> Self::EwmhCookie {
            GetSupportedCookie(xcb_cookie)
        }
    }

    impl EwmhPropertyRequestUnchecked for GetSupported {
        type EwmhCookie = GetSupportedCookieUnchecked;

        fn convert_cookie(
            &self,
            xcb_cookie: xcb::x::GetPropertyCookieUnchecked,
        ) -> Self::EwmhCookie {
            GetSupportedCookieUnchecked(xcb_cookie)
        }

        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms._NET_SUPPORTED,
                r#type: xcb::x::ATOM_ATOM,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    }

    impl EwmhCookie for GetSupportedCookie {
        type XcbCookie = xcb::x::GetPropertyCookie;
    }

    impl EwmhPropertyCookieChecked for GetSupportedCookie {
        type Reply = GetSupportedReply;

        fn inner(self) -> xcb::x::GetPropertyCookie {
            self.0
        }
    }

    impl EwmhPropertyCookieUnchecked for GetSupportedCookieUnchecked {
        type Reply = GetSupportedReply;

        fn inner(self) -> xcb::x::GetPropertyCookieUnchecked {
            self.0
        }
    }

    impl From<xcb::x::GetPropertyReply> for GetSupportedReply {
        fn from(reply: xcb::x::GetPropertyReply) -> Self {
            GetSupportedReply {
                atoms: reply.value().into(),
            }
        }
    }
}

pub(crate) mod net_desktop_names {
    use crate::ewmh::traits::*;
    use crate::ewmh::Connection;

    pub struct GetDesktopNames;

    pub struct GetDesktopNamesCookie(xcb::x::GetPropertyCookie);

    pub struct GetDesktopNamesCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

    #[derive(Debug)]
    pub struct GetDesktopNamesReply {
        pub desktop_names: Vec<String>,
    }

    impl<'a> EwmhRequest<'a> for GetDesktopNames {
        type XcbRequest = xcb::x::GetProperty;
        type EwmhCookie = GetDesktopNamesCookie;

        fn xcb_request(&self, con: &Connection) -> Self::XcbRequest {
            xcb::x::GetProperty {
                delete: false,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms._NET_DESKTOP_NAMES,
                r#type: con.atoms.UTF8_STRING,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }

        fn convert_cookie(&'a self, xcb_cookie: xcb::x::GetPropertyCookie) -> Self::EwmhCookie {
            GetDesktopNamesCookie(xcb_cookie)
        }
    }

    impl EwmhPropertyRequestUnchecked for GetDesktopNames {
        type EwmhCookie = GetDesktopNamesCookieUnchecked;

        fn convert_cookie(
            &self,
            xcb_cookie: xcb::x::GetPropertyCookieUnchecked,
        ) -> Self::EwmhCookie {
            GetDesktopNamesCookieUnchecked(xcb_cookie)
        }

        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
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

    impl EwmhCookie for GetDesktopNamesCookie {
        type XcbCookie = xcb::x::GetPropertyCookie;
    }

    impl EwmhPropertyCookieChecked for GetDesktopNamesCookie {
        type Reply = GetDesktopNamesReply;

        fn inner(self) -> xcb::x::GetPropertyCookie {
            self.0
        }
    }

    impl EwmhPropertyCookieUnchecked for GetDesktopNamesCookieUnchecked {
        type Reply = GetDesktopNamesReply;

        fn inner(self) -> xcb::x::GetPropertyCookieUnchecked {
            self.0
        }
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
        pub fn new(names: Vec<&str>) -> SetDesktopNames {
            let mut data: Vec<u8> = vec![];

            // flatten `new_names` into a continuous array of bytes
            for name in names {
                let mut bname = name.as_bytes().to_owned();
                bname.push(0b00);
                data.extend(bname)
            }

            SetDesktopNames { data }
        }
    }

    impl<'a> EwmhRequest<'a> for SetDesktopNames {
        type XcbRequest = xcb::x::ChangeProperty<'a, u8>;
        type EwmhCookie = xcb::VoidCookie;

        fn xcb_request(&'a self, con: &Connection) -> Self::XcbRequest {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms._NET_DESKTOP_NAMES,
                r#type: con.atoms.UTF8_STRING,
                data: &self.data,
            }
        }

        fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::EwmhCookie {
            xcb_cookie
        }
    }

    impl<'a> EwmhVoidRequestChecked<'a> for SetDesktopNames {
        type XcbRequest = xcb::x::ChangeProperty<'a, u8>;

        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u8> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms._NET_DESKTOP_NAMES,
                r#type: con.atoms.UTF8_STRING,
                data: &self.data,
            }
        }
    }
}

pub(crate) mod net_showing_desktop {
    use crate::ewmh::traits::*;
    use crate::ewmh::Connection;

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
        type XcbRequest = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;
        type EwmhCookie = xcb::VoidCookie;

        fn xcb_request(&'a self, con: &Connection) -> Self::XcbRequest {
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

        fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::EwmhCookie {
            xcb_cookie
        }
    }

    impl<'a> EwmhVoidRequestChecked<'a> for SetShowingDesktop {
        type XcbRequest = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;

        fn xcb_request(&'a self, con: &Connection) -> Self::XcbRequest {
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
}
