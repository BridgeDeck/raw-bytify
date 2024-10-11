use std::{mem::size_of, slice::{from_raw_parts, from_raw_parts_mut}};

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

// Get the bytes of your struct as a reference
pub unsafe fn to_bytes_ref<'a, T>(userdata:&'a T)->Vec<&'a u8>
{
    from_raw_parts(&userdata as *const _ as *const u8,
        size_of::<T>()
        )
        .into_iter().map(|b|{
            return b;
        }).collect::<Vec<&u8>>()
}