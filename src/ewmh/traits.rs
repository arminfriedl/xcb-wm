use crate::ewmh::connection::Connection;

/// Default for a request sent by [`Connection::send_request`]
///
/// For `GetProperty` request (i.e. requests that expect a reply) this is usually
/// a checked request.
/// For `SetProperty` and `ClientMessage` requests (i.e. requests that expect void/no reply) this
/// is usually an unchecked request.
///
/// This is a bit mind-bending and makes these traits hard to grasp. This was however done to align
/// with how the parent [`xcb`] crate works.
pub trait EwmhRequest<'a> {
    /// The underlying [`xcb::Request`] used to do the actual heavy lifting
    type XcbRequest: xcb::Request;

    /// The ewmh wrapper for the [`xcb::Request::Cookie`]
    type EwmhCookie: EwmhCookie<XcbCookie = <Self::XcbRequest as xcb::Request>::Cookie>;

    /// Construct the [`xcb::Request`]. This is implementation specific.
    fn xcb_request(&'a self, con: &Connection) -> Self::XcbRequest;

    /// Convert the [`xcb::Request::Cookie`] to the ewmh cookie wrapper
    fn convert_cookie(
        &'a self,
        xcb_cookie: <Self::XcbRequest as xcb::Request>::Cookie,
    ) -> Self::EwmhCookie;

    /// Default implementation. Delegate the request to the [`xcb::Connection`] and wrap the
    /// response in the ewmh cookie wrapper.
    ///
    /// There is usually no need to override the provided default implementation.
    fn send(&'a self, con: &Connection) -> Self::EwmhCookie {
        let xcb_request = self.xcb_request(con);
        let xcb_cookie = con.con.send_request(&xcb_request);
        self.convert_cookie(xcb_cookie)
    }
}

/// Requests that can be sent by [`Connection::send_request_checked`]
///
/// To quote the parent [`xcb`]:
/// > Checked requests do not expect a reply, but the returned cookie can be used to check for
/// > errors using Connection::check_request.
///
/// For [`xcb-wm`] this means either `SetProperty` or `ClientMessage` requests.
pub trait EwmhVoidRequestChecked<'a> {
    type XcbRequest: xcb::RequestWithoutReply;

    /// Construct the [`xcb::Request`]. This is implementation specific.
    fn xcb_request(&'a self, con: &Connection) -> Self::XcbRequest;

    /// Default implementation. Delegate the request to the [`xcb::Connection`].
    ///
    /// Since we don't need to convert the reply we just re-use the [`xcb::VoidCookieChecked`]
    /// without wrapping it.
    ///
    /// There is usually no need to override the provided default implementation.
    fn send(&'a self, con: &Connection) -> xcb::VoidCookieChecked {
        let xcb_request = self.xcb_request(con);
        con.con.send_request_checked(&xcb_request)
    }
}

/// Requests that can be sent by [`Connection::send_request_unchecked`]
///
/// To quote the parent [`xcb`]:
/// > Unchecked requests expect a reply that is to be retrieved by Connection::wait_for_reply_unchecked.
/// > Unchecked means that the error is not checked when the reply is fetched. Instead, the error will
/// > be sent to the event loop
///
/// For [`xcb-wm`] this means a `GetProperty` requests.
pub trait EwmhPropertyRequestUnchecked {
    type EwmhCookie: EwmhPropertyCookieUnchecked;

    /// Construct the [`xcb::Request`]. This is implementation specific.
    fn xcb_request(&self, con: &Connection) -> xcb::x::GetProperty;

    /// Convert the [`xcb::Request::Cookie`] to the ewmh cookie wrapper
    fn convert_cookie(&self, xcb_cookie: xcb::x::GetPropertyCookieUnchecked) -> Self::EwmhCookie;

    /// Default implementation. Delegate the request to the [`xcb::Connection`].
    ///
    /// Returns a wrapped cookie that tracks the EwmhReply type which is needed
    /// to convert the reply after it is retrieved via [`Connection::wait_for_reply_unchecked`].
    ///
    /// There is usually no need to override the provided default implementation.
    fn send(&self, con: &Connection) -> Self::EwmhCookie {
        let xcb_request = self.xcb_request(con);
        let xcb_cookie = con.con.send_request_unchecked(&xcb_request);
        self.convert_cookie(xcb_cookie)
    }
}

/// Most generic ewmh wrapper for an [`xcb::Cookie`]
///
/// This is needed for [`EwmhRequest`] which basically cuts through
/// all traits. I.e. it is an _unchecked_ request for void requests
/// but a _checked_ request for requests with replies.
///
/// At the same time it may have a _response_ for reply requests or it
/// may have no _response_ for void requests.
pub trait EwmhCookie {
    /// The wrapped [`xcb::Cookie`]
    type XcbCookie: xcb::Cookie;
}

/// Blanket impl for [`xcb::VoidCookie`] (basically unchecked void cookies)
///
/// This is implemented here because there are no special ewmh cookie wrapper
/// for SetProperty and ClientMessage requests needed. [`xcb::VoidCookie`] satisfies
/// all that is needed for [`ewmh`]. In order to use it with [`EwmhRequest`] we need
/// this impl.
impl EwmhCookie for xcb::VoidCookie {
    type XcbCookie = xcb::VoidCookie;
}

/// ewmh wrapper for checked GetProperty requests
///
/// This is needed for 2 purposes:
/// - Carry the reply type from the request to the retrieval of the response
/// - Restrict the methods that can be called with it, i.e. [`Connection::wait_for_reply`]
pub trait EwmhPropertyCookieChecked {
    type Reply: EwmhPropertyReply;

    /// Retrieve the inner [`xcb::Cookie`]
    ///
    /// This is needed for sending a [`xcb::Connection::wait_reply`] request which
    /// expects the xcb cookie
    fn inner(self) -> xcb::x::GetPropertyCookie;
}

/// ewmh wrapper for unchecked GetProperty requests
///
/// This is needed for 2 purposes:
/// - Carry the reply type from the request to the retrieval of the response
/// - Restrict the methods that can be called with it, i.e. [`Connection::wait_for_reply_unchecked`]
pub trait EwmhPropertyCookieUnchecked {
    type Reply: EwmhPropertyReply;

    /// Retrieve the inner [`xcb::Cookie`]
    ///
    /// This is needed for sending a [`xcb::Connection::wait_reply_unchecked`] request which
    /// expects the xcb cookie
    fn inner(self) -> xcb::x::GetPropertyCookieUnchecked;
}

/// Marker trait with blanket implementation for everything that implements
/// [`From<xcb::x::GetPropertyReply`].
///
/// The ewmh property reply trait is used to convert a generic reply to a [`xcb::x::GetProperty`]
/// request to a specific ewmh reply struct.
///
/// The connection between a ewmh request and the reply struct is made via ewmh property cookies
/// ([`EwmhPropertyCookieChecked`] and [`EwmhPropertyCookieUnchecked`]
pub trait EwmhPropertyReply: From<xcb::x::GetPropertyReply> {}
impl<T> EwmhPropertyReply for T where T: From<xcb::x::GetPropertyReply> {}
