use crate::ewmh::atoms::Atoms;
use crate::ewmh::traits::{
    EwmhPropertyCookieChecked, EwmhPropertyCookieUnchecked, EwmhPropertyRequestUnchecked,
    EwmhRequest, EwmhVoidRequestChecked,
};

/// The main `ewmh` entry point
///
/// `Connection` is a thin wrapper around [`xcb::Connection`]. It provides a
/// subset of the [`xcb::Connection`] API covering the functionality needed for
/// `ewmh`.
///
/// The provided subset works the same as the corresponding API in
/// [`xcb::Connection`]. That is, methods with the same name do the same thing.
/// The general usage pattern is the same for both crates.
///
/// This wrapper is needed because `ewmh` has to prepare the `Connection` for
/// `ewmh` requests and store additional data on it. Concretely, this mostly
/// means interning atoms.
pub struct Connection<'a> {
    pub(crate) con: &'a xcb::Connection,

    /// Interned [`Atoms`] for the `ewmh` protocol
    pub atoms: Atoms,
}

#[allow(dead_code)]
impl<'a> Connection<'a> {
    pub fn connect(xcb_con: &'a xcb::Connection) -> Connection<'a> {
        Connection {
            atoms: Atoms::intern(xcb_con),
            con: xcb_con,
        }
    }

    pub fn send_request<'b, R>(&self, request: &'b R) -> R::EwmhCookie
    where
        R: EwmhRequest<'b>,
    {
        request.send(self)
    }

    pub fn send_request_checked<'b, R>(&self, request: &'b R) -> xcb::VoidCookieChecked
    where
        R: EwmhVoidRequestChecked<'b>,
    {
        request.send(self)
    }

    pub fn send_request_unchecked<R>(&self, request: &R) -> R::EwmhCookie
    where
        R: EwmhPropertyRequestUnchecked,
    {
        request.send(self)
    }

    pub fn wait_for_reply<C>(&self, cookie: C) -> C::Reply
    where
        C: EwmhPropertyCookieChecked,
    {
        let xcb_reply = self.con.wait_for_reply(cookie.inner());
        xcb_reply.unwrap().into()
    }

    pub fn wait_for_reply_unchecked<C>(&self, cookie: C) -> C::Reply
    where
        C: EwmhPropertyCookieUnchecked,
    {
        let xcb_reply = self.con.wait_for_reply_unchecked(cookie.inner());
        xcb_reply.unwrap().unwrap().into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn supported_atoms_list() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetSupported;
        let cookie = ewmh_con.send_request(&request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);

        for atom in reply.atoms {
            let cookie = xcb_con.send_request(&xcb::x::GetAtomName { atom: atom });

            println!("{}", xcb_con.wait_for_reply(cookie).unwrap().name());
        }
    }
    //
    // #[test]
    // fn client_list() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetClientList::new();
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    #[test]
    fn number_of_desktops() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNumberOfDesktops;
        let cookie = ewmh_con.send_request(&request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    // #[test]
    // fn number_of_desktops2() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetNumberOfDesktops::new();
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn desktop_geometry() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetNumberOfDesktops {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn set_desktop_geometry() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::SetDesktopGeometry::new(1024, 800);
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = xcb_con.check_request(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn desktop_viewport() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetDesktopViewport {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn set_desktop_viewport() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::SetDesktopViewport::new(200, 200);
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = xcb_con.check_request(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn desktop_viewport2() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetDesktopViewport {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn desktop_current() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetCurrentDesktop {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    #[test]
    fn desktop_names() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetDesktopNames {};
        let cookie = ewmh_con.send_request(&request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn set_desktop_names() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::SetDesktopNames::new(vec!["A", "B", "Z"]);
        let cookie = ewmh_con.send_request_checked(&request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }
    //
    // #[test]
    // fn set_active_window() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetClientList {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //
    //     let request = crate::ewmh::proto::SetActiveWindow::new(
    //         &ewmh_con,
    //         reply.clients.last().unwrap().to_owned(),
    //         1,
    //         xcb::x::CURRENT_TIME,
    //         Option::None,
    //     );
    //
    //     let cookie = ewmh_con.send_request(request);
    //     xcb_con.check_request(cookie).unwrap();
    // }
    //
    // #[test]
    // fn workarea() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetWorkarea {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn supporting_wm_check() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetSupportingWmCheck {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn virtual_roots() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetVirtualRoots {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn desktop_layout() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetDesktopLayout {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn showing_desktop() {
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let request = crate::ewmh::proto::GetShowingDesktop {};
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    #[test]
    fn set_showing_desktop() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::SetShowingDesktop::new(&ewmh_con, true);
        let cookie = ewmh_con.send_request_checked(&request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }

    // #[test]
    // fn close_window() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(20979719) };
    //
    //     let request =
    //         crate::ewmh::proto::CloseWindow::new(&ewmh_con, window, 0, xcb::x::CURRENT_TIME);
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = xcb_con.check_request(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn request_frame_extents() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(20979719) };
    //
    //     let request = crate::ewmh::proto::RequestFrameExtents::new(&ewmh_con, window);
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = xcb_con.check_request(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn wm_name() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(77594744) };
    //
    //     let request = crate::ewmh::proto::GetWmName::new(window);
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn wm_visible_name() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(20983603) };
    //
    //     let request = crate::ewmh::proto::GetWmVisibleName::new(window);
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn wm_desktop() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(20983603) };
    //
    //     let request = crate::ewmh::proto::GetWmDesktop::new(window);
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn set_wm_type() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(20995834) };
    //
    //     let request = crate::ewmh::proto::SetWmWindowType::new(
    //         window,
    //         vec![
    //             ewmh_con.atoms._NET_WM_WINDOW_TYPE_UTILITY,
    //             ewmh_con.atoms._NET_WM_WINDOW_TYPE_SPLASH,
    //         ],
    //     );
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = xcb_con.check_request(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn get_wm_type() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(20995834) };
    //
    //     let request = crate::ewmh::proto::GetWmWindowType::new(window);
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn set_wm_state() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(21002424) };
    //
    //     let request = crate::ewmh::proto::SetWmState::new(
    //         window,
    //         vec![
    //             ewmh_con.atoms._NET_WM_STATE_STICKY,
    //             ewmh_con.atoms._NET_WM_STATE_DEMANDS_ATTENTION,
    //         ],
    //     );
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = xcb_con.check_request(cookie);
    //     println!("{:?}", reply);
    // }
    //
    // #[test]
    // fn get_wm_state() {
    //     use xcb::XidNew;
    //
    //     let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
    //     let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);
    //
    //     let window = unsafe { xcb::x::Window::new(21002424) };
    //
    //     let request = crate::ewmh::proto::GetWmWindowType::new(window);
    //
    //     let cookie = ewmh_con.send_request(request);
    //     let reply = ewmh_con.wait_for_reply(cookie);
    //     println!("{:?}", reply);
    // }
}
