#[macro_use]
mod get_property;

#[macro_use]
mod set_property;

#[macro_use]
mod client_message;

macro_rules! x_buffer_string_or_default {
    ($reply:expr) => {{
        x_buffer_to_strings($reply.value::<u8>())
            .first()
            .map_or_else(Default::default, ToOwned::to_owned)
    }};
}

macro_rules! nth_u32_value_or_default {
    ($reply:expr, $index:expr) => {{
        $reply
            .value::<u32>()
            .get($index)
            .map_or_else(Default::default, ToOwned::to_owned)
    }};
}
