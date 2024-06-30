use std::mem::transmute;

/// From `bytes`, create a reference to `T`.
pub unsafe fn from_bytes<'a, T>(bytes:&'a mut Vec<u8>)->&'a mut T
{
    let (head, body, tail) = bytes.align_to_mut::<T>();
    &mut body[0]
}

/// Similar to [from_bytes], but returns a static reference to `T`.
pub unsafe fn from_bytes_static<T>(bytes:&mut Vec<u8>)->&'static mut T
{
    transmute(from_bytes::<T>(bytes))
    //let (head, body, tail) = bytes.align_to_mut::<T>();
    //transmute(&mut body[0])
}

/// Returns a clone of `T` from `bytes`.
pub unsafe fn from_bytes_owned<T>(mut bytes:Vec<u8>)->T
where T: Clone
{
    //let (head, body, tail) = bytes.align_to_mut::<T>();
    from_bytes::<T>(&mut bytes).clone()
}