fn x_buffer_to_strings(xbuf: &[u8]) -> Vec<String> {
    let mut vals = vec![];
    let mut buf = vec![];

    for b in xbuf {
        if *b != 0x00 {
            buf.push(*b)
        } else if !buf.is_empty() {
            vals.push(String::from_utf8(buf.clone()).unwrap());
            buf.clear();
        } else {
            buf.clear();
        }
    }

    vals
}

fn strings_to_x_buffer(strings: Vec<&str>) -> Vec<u8> {
    let mut data = vec![];

    // flatten `strings` into a continuous, NULL separated, array of bytes
    for s in strings {
        let mut bs = s.as_bytes().to_owned();
        bs.push(0b00);
        data.extend(bs)
    }

    data
}

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
}

pub(crate) mod net_desktop_names {
    use crate::ewmh::proto::test_props::{strings_to_x_buffer, x_buffer_to_strings};
    use crate::ewmh::traits::*;
    use crate::ewmh::Connection;

    pub struct GetDesktopNames;

    pub struct GetDesktopNamesCookie(xcb::x::GetPropertyCookie);

    pub struct GetDesktopNamesCookieUnchecked(xcb::x::GetPropertyCookieUnchecked);

    #[derive(Debug)]
    pub struct GetDesktopNamesReply {
        pub desktop_names: Vec<String>,
    }

    impl From<xcb::x::GetPropertyReply> for GetDesktopNamesReply {
        fn from(reply: xcb::x::GetPropertyReply) -> Self {
            GetDesktopNamesReply {
                desktop_names: x_buffer_to_strings(reply.value::<u8>()),
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

    ewmh_client_message! {
        request=SetShowingDesktop{destination: root_window}
    }
}
