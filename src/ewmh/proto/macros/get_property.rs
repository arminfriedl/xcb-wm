macro_rules! _get_property_cookies {
    (@ewmh_priv $request:ident, $reply:ident) => {
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
    (@ewmh_priv root, $property:ident, UTF8_STRING) => {
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

    (@ewmh_priv root, $property:ident, $xtype:ident) => {
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

    (@ewmh_priv client, $property:ident, UTF8_STRING) => {
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

    (@ewmh_priv client, $property:ident, $xtype:ident) => {
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
    (@ewmh_priv $request:ident, root) => {
        paste! {
            pub struct $request;
            pub struct [<$request Cookie>](xcb::x::GetPropertyCookie);
            pub struct [<$request CookieUnchecked>](xcb::x::GetPropertyCookieUnchecked);
        }
    };
    (@ewmh_priv $request:ident, client) => {
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
           _get_property_structs! {@ewmh_priv $request, $window}

           impl<'a> EwmhRequest<'a> for $request {
                type XcbRequest = xcb::x::GetProperty;
                type EwmhCookie = [<$request Cookie>];

                _get_property_request! {@ewmh_priv $window, $property, $xtype}

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

                _get_property_request! {@ewmh_priv $window, $property, $xtype}
            }

            _get_property_cookies! {@ewmh_priv $request, $reply}
        }
    };
}
