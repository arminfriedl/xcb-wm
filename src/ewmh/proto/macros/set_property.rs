macro_rules! _set_property_base {
    (root, $property:ident, UTF8_STRING) => {
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

    (root, $property:ident, $xtype:ident) => {
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

    (client, $property:ident, UTF8_STRING) => {
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

    (client, $property:ident, ATOM_CARDINAL) => {
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

    (client, $property:ident, ATOM_ATOM) => {
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
