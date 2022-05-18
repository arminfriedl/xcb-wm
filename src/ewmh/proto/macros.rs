macro_rules! _get_property_base {
    ($reply:ident, $cookie:ident, $cookie_unchecked:ident) => {
        impl EwmhCookie for $cookie {
            type XcbCookie = xcb::x::GetPropertyCookie;
        }

        impl EwmhPropertyCookieChecked for $cookie {
            type Reply = $reply;

            fn inner(self) -> xcb::x::GetPropertyCookie {
                self.0
            }
        }

        impl EwmhPropertyCookieUnchecked for $cookie_unchecked {
            type Reply = $reply;

            fn inner(self) -> xcb::x::GetPropertyCookieUnchecked {
                self.0
            }
        }
    };
}

macro_rules! _get_property_request {
    (root_window, $property:ident, UTF8_STRING) => {
        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms.$property,
                r#type: con.atoms.UTF8_STRING,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };

    (root_window, $property:ident, $xtype:ident) => {
        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms.$property,
                r#type: xcb::x::$xtype,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };

    ($window:ident, $property:ident, UTF8_STRING) => {
        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: $window,
                property: con.atoms.$property,
                r#type: con.atoms.UTF8_STRING,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };

    ($window:ident, $property:ident, $xtype:ident) => {
        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: $window,
                property: con.atoms.$property,
                r#type: xcb::x::$xtype,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };
}

macro_rules! ewmh_get_property {
    (request=$request:ident{
        window: $window:ident,
        property: $property:ident,
        xtype: $xtype: ident
     },
     reply=$reply:ident,
     cookie=$cookie:ident,
     cookie_unchecked=$cookie_unchecked:ident) => {
        impl<'a> EwmhRequest<'a> for $request {
            type XcbRequest = xcb::x::GetProperty;
            type EwmhCookie = $cookie;

            _get_property_request! {$window, $property, $xtype}

            fn convert_cookie(&'a self, xcb_cookie: xcb::x::GetPropertyCookie) -> Self::EwmhCookie {
                $cookie(xcb_cookie)
            }
        }

        impl EwmhPropertyRequestUnchecked for $request {
            type EwmhCookie = $cookie_unchecked;


            #[rustfmt::skip]
            fn convert_cookie(&self,xcb_cookie: xcb::x::GetPropertyCookieUnchecked,) -> Self::EwmhCookie {
                $cookie_unchecked(xcb_cookie)
            }

            _get_property_request! {$window, $property, $xtype}
        }

        _get_property_base! {$reply, $cookie, $cookie_unchecked}
    };
}

macro_rules! _set_property_base {
    (root_window, $property:ident, UTF8_STRING) => {
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u8> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms.$property,
                r#type: con.atoms.UTF8_STRING,
                data: &self.data,
            }
        }
    };

    (root_window, $property:ident, $xtype:ident) => {
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u8> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms.$property,
                r#type: xcb::x::$xtype,
                data: &self.data,
            }
        }
    };

    ($window:ident, $property:ident, UTF8_STRING) => {
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u8> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: $window,
                property: con.atoms.$property,
                r#type: con.atoms.UTF8_STRING,
                data: &self.data,
            }
        }
    };

    ($window:ident, $property:ident, $xtype:ident) => {
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u8> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: $window,
                property: con.atoms.$property,
                r#type: xcb::x::$xtype,
                data: &self.data,
            }
        }
    };
}

macro_rules! ewmh_set_property {
    (request=$request:ident{
        window: $window:ident,
        property: $property:ident,
        xtype: $xtype: ident
     }) => {
        impl<'a> EwmhRequest<'a> for $request {
            // TODO fixing to u8 is probably not flexible enough
            type XcbRequest = xcb::x::ChangeProperty<'a, u8>;
            type EwmhCookie = xcb::VoidCookie;

            _set_property_base! {$window, $property, $xtype}

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::EwmhCookie {
                xcb_cookie
            }
        }

        impl<'a> EwmhVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, u8>;

            _set_property_base! {$window, $property, $xtype}
        }
    };
}

macro_rules! _client_message_base {
    (root_window) => {
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
