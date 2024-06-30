use std::{mem::size_of, slice::from_raw_parts};

/// Convert your struct to an array of bytes.
pub unsafe fn to_bytes<T>(userdata:T)->Vec<u8>
{
    from_raw_parts(&userdata as *const _ as *const u8,
        size_of::<T>()
        )
        .into_iter().map(|b|{
            return *b;
        }).collect::<Vec<u8>>()
}