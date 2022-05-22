macro_rules! _client_message_base {
    (root) => {
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
    };

    ($destination: ident) => {
        fn xcb_request(&'a self, con: &Connection) -> Self::XcbRequest {
            xcb::x::SendEvent {
                propagate: false,
                destination: xcb::x::SendEventDest::Window($destination),
                event_mask: xcb::x::EventMask::SUBSTRUCTURE_NOTIFY
                    | xcb::x::EventMask::SUBSTRUCTURE_REDIRECT,
                event: &self.client_message,
            }
        }
    };
}

macro_rules! ewmh_client_message {
    (request=$request:ident{destination: $destination:ident}) => {
        impl<'a> EwmhRequest<'a> for $request {
            type XcbRequest = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;
            type EwmhCookie = xcb::VoidCookie;

            _client_message_base! {$destination}

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::EwmhCookie {
                xcb_cookie
            }
        }

        impl<'a> EwmhVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::SendEvent<'a, xcb::x::ClientMessageEvent>;
            _client_message_base! {$destination}
        }
    };
}
