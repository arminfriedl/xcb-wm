pub(crate) fn x_buffer_to_strings(xbuf: &[u8]) -> Vec<String> {
    let mut vals = vec![];
    let mut buf = vec![];

    for b in xbuf {
        if *b != 0x00 {
            buf.push(*b)
        } else if !buf.is_empty() {
            vals.push(String::from_utf8(buf.clone()).unwrap());
            buf.clear();
        }
    }

    if !buf.is_empty() {
        // drain buf
        vals.push(String::from_utf8(buf.clone()).unwrap());
    }

    vals
}

pub(crate) fn strings_to_x_buffer(strings: Vec<&str>) -> Vec<u8> {
    let mut data = vec![];

    // flatten `strings` into a continuous, NULL separated, array of bytes
    for s in strings {
        let mut bs = s.as_bytes().to_owned();
        bs.push(0b00);
        data.extend(bs)
    }

    data
}
