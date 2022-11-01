macro_rules! _get_property_cookies {
    (@icccm_priv $request:ident, $reply:ident) => {
        paste! {
            impl IcccmCookie for [<$request Cookie>] {
                type XcbCookie = xcb::x::GetPropertyCookie;
            }

            impl IcccmPropertyCookieChecked for [<$request Cookie>] {
                type Reply = $reply;

                fn inner(self) -> xcb::x::GetPropertyCookie {
                    self.0
                }
            }

            impl IcccmPropertyCookieUnchecked for [<$request CookieUnchecked>] {
                type Reply = $reply;

                fn inner(self) -> xcb::x::GetPropertyCookieUnchecked {
                    self.0
                }
            }
        }
    };
}

macro_rules! _get_property_request {
    //TODO do we need the `client` here? Probs just c&p
    (@icccm_priv client, WM_COLORMAP_WINDOWS, $xtype:ident) => {
        fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: self.0,
                property: con.atoms.WM_COLORMAP_WINDOWS,
                r#type: xcb::x::$xtype,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };

    (@icccm_priv client, $property:ident, $xtype:ident) => {
        fn xcb_request(&self, _con: &Connection) -> xcb::x::GetProperty {
            xcb::x::GetProperty {
                delete: false,
                window: self.0,
                property: xcb::x::$property,
                r#type: xcb::x::$xtype,
                long_offset: 0,
                long_length: u32::MAX,
            }
        }
    };
}

macro_rules! icccm_get_property {
    (request=$request:ident{
        window: $window:ident,
        property: $property:ident,
        xtype: $xtype: ident
     },
     reply=$reply:ident) => {
       paste!{
            pub struct $request(xcb::x::Window);
            pub struct [<$request Cookie>](xcb::x::GetPropertyCookie);
            pub struct [<$request CookieUnchecked>](xcb::x::GetPropertyCookieUnchecked);

            impl $request {
                pub fn new(window: xcb::x::Window) -> $request {
                    $request(window)
                }
            }

           impl<'a> IcccmRequest<'a> for $request {
                type XcbRequest = xcb::x::GetProperty;
                type IcccmCookie = [<$request Cookie>];

                _get_property_request! {@icccm_priv $window, $property, $xtype}

                fn convert_cookie(&'a self, xcb_cookie: xcb::x::GetPropertyCookie) -> Self::IcccmCookie {
                    [<$request Cookie>](xcb_cookie)
                }
            }

            impl IcccmPropertyRequestUnchecked for $request {
                paste!{type IcccmCookie = [<$request CookieUnchecked>];}

                #[rustfmt::skip]
                fn convert_cookie(&self, xcb_cookie: xcb::x::GetPropertyCookieUnchecked) -> Self::IcccmCookie {
                    [<$request CookieUnchecked>](xcb_cookie)
                }

                _get_property_request! {@icccm_priv $window, $property, $xtype}
            }

            _get_property_cookies! {@icccm_priv $request, $reply}
        }
    };
}
