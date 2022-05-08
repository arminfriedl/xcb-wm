use xcb::x::{Atom, GetPropertyReply};

use crate::ewmh::proto_traits::{EwmhCookie, EwmhRequest};

use super::atoms::Atoms;

pub struct Connection<'a> {
    pub(crate) con: &'a xcb::Connection,
    pub atoms: Atoms,
}

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

    fn send_request<R: EwmhRequest>(&self, request: R) -> R::Cookie {
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

        let request = crate::ewmh::proto::GetNetSupportedRequest {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);

        for atom in reply.value {
            let cookie = xcb_con.send_request(&xcb::x::GetAtomName { atom: atom });

            println!("{}", xcb_con.wait_for_reply(cookie).unwrap().name());
        }
    }

    #[test]
    fn client_list() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNetClientListRequest {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn number_of_desktops() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNetNumberOfDesktopsRequest {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_geometry() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNetDesktopGeometryRequest {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_viewport() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNetDesktopViewportRequest {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_current() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNetCurrentDesktopRequest {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn desktop_names() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let ewmh_con = crate::ewmh::ewmh::Connection::connect(&xcb_con);

        let request = crate::ewmh::proto::GetNetDesktopNamesRequest {};
        let cookie = ewmh_con.send_request(request);
        let reply = ewmh_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }
}
