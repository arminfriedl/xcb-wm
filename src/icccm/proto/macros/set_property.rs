macro_rules! icccm_set_text_property {
    (request=$request:ident{
        property: $property:ident
    }) => {
        impl<'a, T: 'a + xcb::x::PropEl> IcccmRequest<'a> for $request<T> {
            type XcbRequest = xcb::x::ChangeProperty<'a, T>;
            type IcccmCookie = xcb::VoidCookie;

            fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, T> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: xcb::x::$property,
                    r#type: self.encoding,
                    data: &self.data,
                }
            }

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::IcccmCookie {
                xcb_cookie
            }
        }

        impl<'a, T: 'a + xcb::x::PropEl> IcccmVoidRequestChecked<'a> for $request<T> {
            type XcbRequest = xcb::x::ChangeProperty<'a, T>;

            fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, T> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: xcb::x::$property,
                    r#type: self.encoding,
                    data: &self.data,
                }
            }
        }
    };
}

macro_rules! icccm_set_property {
    (request=$request:ident{
        property: con.$property:ident,
        xtype: $type:ident
     }) => {
        impl<'a> IcccmRequest<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, xcb::x::Window>;
            type IcccmCookie = xcb::VoidCookie;

            fn xcb_request(
                &'a self,
                con: &Connection,
            ) -> xcb::x::ChangeProperty<'a, xcb::x::Window> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: con.atoms.$property,
                    r#type: xcb::x::$type,
                    data: &self.data,
                }
            }

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::IcccmCookie {
                xcb_cookie
            }
        }

        impl<'a> IcccmVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, xcb::x::Window>;

            fn xcb_request(
                &'a self,
                con: &Connection,
            ) -> xcb::x::ChangeProperty<'a, xcb::x::Window> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: con.atoms.$property,
                    r#type: xcb::x::$type,
                    data: &self.data,
                }
            }
        }
    };

    (request=$request:ident{
        property: $property:ident,
        xtype: $type:ident
     }) => {
        impl<'a> IcccmRequest<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, xcb::x::Window>;
            type IcccmCookie = xcb::VoidCookie;

            fn xcb_request(
                &'a self,
                con: &Connection,
            ) -> xcb::x::ChangeProperty<'a, xcb::x::Window> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: xcb::x::$property,
                    r#type: xcb::x::$type,
                    data: &self.data,
                }
            }

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::IcccmCookie {
                xcb_cookie
            }
        }

        impl<'a> IcccmVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, xcb::x::Window>;

            fn xcb_request(
                &'a self,
                con: &Connection,
            ) -> xcb::x::ChangeProperty<'a, xcb::x::Window> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: xcb::x::$property,
                    r#type: xcb::x::$type,
                    data: &self.data,
                }
            }
        }
    };
}

macro_rules! icccm_set_hint_property {
    (request=$request:ident{
        property: $property:ident,
        xtype: $type:ident
     }) => {
        impl<'a> IcccmRequest<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, u32>;
            type IcccmCookie = xcb::VoidCookie;

            fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u32> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: xcb::x::$property,
                    r#type: xcb::x::$type,
                    data: &self.data,
                }
            }

            fn convert_cookie(&'a self, xcb_cookie: xcb::VoidCookie) -> Self::IcccmCookie {
                xcb_cookie
            }
        }

        impl<'a> IcccmVoidRequestChecked<'a> for $request {
            type XcbRequest = xcb::x::ChangeProperty<'a, u32>;

            fn xcb_request(&'a self, con: &Connection) -> xcb::x::ChangeProperty<'a, u32> {
                xcb::x::ChangeProperty {
                    mode: xcb::x::PropMode::Replace,
                    window: self.window,
                    property: xcb::x::$property,
                    r#type: xcb::x::$type,
                    data: &self.data,
                }
            }
        }
    };
}
