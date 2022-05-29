use std::collections::HashMap;

// TODO use xcb::atom_struct!{} for this?
const ATOM_NAMES: [&str; 1] = ["WM_COLORMAP_WINDOWS"];

/// Interned [`xcb::x::Atom`]s for the `icccm` protocol
///
/// The ids for these atoms are created when the [`crate::icccm::Connection`] is established. Hence,
/// are bound to the [`crate::icccm::Connection`] and can only be retrieved from there.
///
/// See also: [`icccm spec`](https://www.x.org/releases/X11R7.7/doc/xorg-docs/icccm/icccm.html)
#[allow(non_snake_case)]
pub struct Atoms {
    pub WM_COLORMAP_WINDOWS: xcb::x::Atom,
}

impl Atoms {
    pub(crate) fn intern(con: &xcb::Connection) -> Atoms {
        let mut cookies: HashMap<&'static str, xcb::x::InternAtomCookie> = HashMap::new();

        for atom in ATOM_NAMES {
            let intern_atom = xcb::x::InternAtom {
                only_if_exists: false,
                name: atom.as_bytes(),
            };

            cookies.insert(atom, con.send_request(&intern_atom));
        }

        let interned_atoms: HashMap<&'static str, xcb::x::Atom> = cookies
            .into_iter()
            .map(|(atom_name, cookie)| (atom_name, con.wait_for_reply(cookie).unwrap()))
            .map(|(atom_name, reply)| (atom_name, reply.atom()))
            .collect();

        Atoms::from_interned_atoms(interned_atoms)
    }

    fn from_interned_atoms(mut atoms: HashMap<&'static str, xcb::x::Atom>) -> Atoms {
        Atoms {
            WM_COLORMAP_WINDOWS: atoms.remove("WM_COLORMAP_WINDOWS").unwrap(),
        }
    }
}
