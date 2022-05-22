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
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u32> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: con.con.get_setup().roots().next().unwrap().root(),
                property: con.atoms.$property,
                r#type: xcb::x::$xtype,
                data: &self.data,
            }
        }
    };

    (client_window, $property:ident, UTF8_STRING) => {
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u8> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: self.window,
                property: con.atoms.$property,
                r#type: con.atoms.UTF8_STRING,
                data: &self.data,
            }
        }
    };

    (client_window, $property:ident, ATOM_CARDINAL) => {
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u32> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: self.window,
                property: con.atoms.$property,
                r#type: xcb::x::ATOM_CARDINAL,
                data: &self.data,
            }
        }
    };

    (client_window, $property:ident, ATOM_ATOM) => {
        fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, xcb::x::Atom> {
            xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: self.window,
                property: con.atoms.$property,
                r#type: xcb::x::ATOM_ATOM,
                data: &self.data,
            }
        }
    };
}

macro_rules! ewmh_set_property {
    (request=$request:ident{
        window: $window:ident,
        property: $property:ident,
        xtype: UTF8_STRING
     }) => {
        impl<'a> EwmhRequest<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, u8>;
            type EwmhCookie = xcb::VoidCookie;

            _set_property_base! {$window, $property, UTF8_STRING}

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::EwmhCookie {
                xcb_cookie
            }
        }

        impl<'a> EwmhVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, u8>;

            _set_property_base! {$window, $property, UTF8_STRING}
        }
    };

    (request=$request:ident{
        window: $window:ident,
        property: $property:ident,
        xtype: ATOM_CARDINAL
     }) => {
        impl<'a> EwmhRequest<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, u32>;
            type EwmhCookie = xcb::VoidCookie;

            _set_property_base! {$window, $property, ATOM_CARDINAL}

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::EwmhCookie {
                xcb_cookie
            }
        }

        impl<'a> EwmhVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, u32>;

            _set_property_base! {$window, $property, ATOM_CARDINAL}
        }
    };

    (request=$request:ident{
        window: $window:ident,
        property: $property:ident,
        xtype: ATOM_ATOM
     }) => {
        impl<'a> EwmhRequest<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, xcb::x::Atom>;
            type EwmhCookie = xcb::VoidCookie;

            _set_property_base! {$window, $property, ATOM_ATOM }

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::EwmhCookie {
                xcb_cookie
            }
        }

        impl<'a> EwmhVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, xcb::x::Atom>;

            _set_property_base! {$window, $property, ATOM_ATOM }
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

macro_rules! _get_property_cookies {
    (@private $request:ident, $reply:ident) => {
        paste! {
            impl EwmhCookie for [<$request Cookie>] {
                type XcbCookie = xcb::x::GetPropertyCookie;
            }

            impl EwmhPropertyCookieChecked for [<$request Cookie>] {
                type Reply = $reply;

                fn inner(self) -> xcb::x::GetPropertyCookie {
                    self.0
                }
            }

            impl EwmhPropertyCookieUnchecked for [<$request CookieUnchecked>] {
                type Reply = $reply;

                fn inner(self) -> xcb::x::GetPropertyCookieUnchecked {
                    self.0
                }
            }
        }
    };
}

macro_rules! _get_property_request {
    (@private root, $property:ident, UTF8_STRING) => {
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

    (@private root, $property:ident, $xtype:ident) => {
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

    (@private client, $property:ident, UTF8_STRING) => {
        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: self.0,
                property: con.atoms.$property,
                r#type: con.atoms.UTF8_STRING,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };

    (@private client, $property:ident, $xtype:ident) => {
        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: self.0,
                property: con.atoms.$property,
                r#type: xcb::x::$xtype,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };
}

macro_rules! _get_property_structs {
    (@private $request:ident, root) => {
        paste! {
            pub struct $request;
            pub struct [<$request Cookie>](xcb::x::GetPropertyCookie);
            pub struct [<$request CookieUnchecked>](xcb::x::GetPropertyCookieUnchecked);
        }
    };
    (@private $request:ident, client) => {
        paste! {
            pub struct $request(xcb::x::Window);
            pub struct [<$request Cookie>](xcb::x::GetPropertyCookie);
            pub struct [<$request CookieUnchecked>](xcb::x::GetPropertyCookieUnchecked);

            impl $request {
                fn new(window: xcb::x::Window) -> $request {
                    $request(window)
                }
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
     reply=$reply:ident) => {
       paste!{
           _get_property_structs! {@private $request, $window}

           impl<'a> EwmhRequest<'a> for $request {
                type XcbRequest = xcb::x::GetProperty;
                type EwmhCookie = [<$request Cookie>];

                _get_property_request! {@private $window, $property, $xtype}

                fn convert_cookie(&'a self, xcb_cookie: xcb::x::GetPropertyCookie) -> Self::EwmhCookie {
                    [<$request Cookie>](xcb_cookie)
                }
            }

            impl EwmhPropertyRequestUnchecked for $request {
                paste!{type EwmhCookie = [<$request CookieUnchecked>];}

                #[rustfmt::skip]
                fn convert_cookie(&self, xcb_cookie: xcb::x::GetPropertyCookieUnchecked) -> Self::EwmhCookie {
                    [<$request CookieUnchecked>](xcb_cookie)
                }

                _get_property_request! {@private $window, $property, $xtype}
            }

            _get_property_cookies! {@private $request, $reply}
        }
    };
}
