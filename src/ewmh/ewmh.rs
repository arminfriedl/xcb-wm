use crate::ewmh::proto_traits::{EwmhCookie, EwmhRequest};

use super::atoms::Atoms;

pub struct Connection<'a> {
    pub(crate) con: &'a xcb::Connection,
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

    fn send_client_message(
        &self,
        window: xcb::x::Window,
        dest: xcb::x::Window,
        atom: xcb::x::Atom,
        data: xcb::x::ClientMessageData,
    ) -> xcb::VoidCookie {
        let event = xcb::x::ClientMessageEvent::new(window, atom, data);

        let request = xcb::x::SendEvent {
            propagate: false,
            destination: xcb::x::SendEventDest::Window(dest),
            event_mask: xcb::x::EventMask::SUBSTRUCTURE_NOTIFY
                | xcb::x::EventMask::SUBSTRUCTURE_REDIRECT,
            event: &event,
        };

        self.con.send_request(&request)
    }

    fn send_request<R: EwmhRequest<'a>>(&self, request: R) -> R::Cookie {
        request.send_request(self)
    }

    fn wait_for_reply<C: EwmhCookie>(&self, cookie: C) -> C::Reply {
        cookie.reply(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn supported_atoms_list() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetSupported::new();
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);

        for atom in reply.atoms {
            let cookie = xcb_con.send_request(&xcb::x::GetAtomName { atom: atom });

            println!("{}", xcb_con.wait_for_reply(cookie).unwrap().name());
        }
    }

    #[test]
    fn client_list() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetClientList::new();
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn number_of_desktops() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNumberOfDesktops::new();
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn set_number_of_desktops() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::SetNumberOfDesktops::new(4);
        let cookie = ewmh_con.send_request(request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn number_of_desktops2() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNumberOfDesktops::new();
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_geometry() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNumberOfDesktops {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn set_desktop_geometry() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::SetDesktopGeometry::new(1024, 800);
        let cookie = ewmh_con.send_request(request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_viewport() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetDesktopViewport {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn set_desktop_viewport() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::SetDesktopViewport::new(200, 200);
        let cookie = ewmh_con.send_request(request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_viewport2() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetDesktopViewport {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_current() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetCurrentDesktop {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_names() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetDesktopNames {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn set_active_window() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetClientList {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);

        let request = crate::ewmh::proto::SetActiveWindow::new(
            &ewmh_con,
            reply.clients.last().unwrap().to_owned(),
            1,
            xcb::x::CURRENT_TIME,
            Option::None,
        );

        let cookie = ewmh_con.send_request(request);
        xcb_con.check_request(cookie).unwrap();
    }

    #[test]
    fn workarea() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetWorkarea {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn supporting_wm_check() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetSupportingWmCheck {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn virtual_roots() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetVirtualRoots {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_layout() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetDesktopLayout {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn showing_desktop() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetShowingDesktop {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn set_showing_desktop() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::SetShowingDesktop::new(&ewmh_con, true);
        let cookie = ewmh_con.send_request(request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn close_window() {
        use xcb::XidNew;

        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let window = unsafe { xcb::x::Window::new(20979719) };

        let request =
            crate::ewmh::proto::CloseWindow::new(&ewmh_con, window, 0, xcb::x::CURRENT_TIME);

        let cookie = ewmh_con.send_request(request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn request_frame_extents() {
        use xcb::XidNew;

        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let window = unsafe { xcb::x::Window::new(20979719) };

        let request = crate::ewmh::proto::RequestFrameExtents::new(&ewmh_con, window);

        let cookie = ewmh_con.send_request(request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }
}
