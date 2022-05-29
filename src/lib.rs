#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]

#[cfg(feature = "ewmh")]
pub mod ewmh;

#[cfg(feature = "icccm")]
pub mod icccm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
