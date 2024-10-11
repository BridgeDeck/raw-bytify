use std::mem::transmute;

/// From `bytes`, create a reference to `T`.
pub unsafe fn from_bytes<'a, T>(bytes:&'a mut Vec<u8>)->&'a mut T
{
    let (_head, body, _tail) = bytes.align_to_mut::<T>();
    
    &mut body[0]
}

/// Similar to [from_bytes], but returns a static reference to `T`.
pub unsafe fn from_bytes_static<T>(bytes:&mut Vec<u8>)->&'static mut T
{
    transmute(from_bytes::<T>(bytes))
    //let (head, body, tail) = bytes.align_to_mut::<T>();
    //transmute(&mut body[0])
}

// Copies bytes from `bytes` into `output`, effectively making a copy.
// pub unsafe fn from_bytes_copy<T>(bytes:Vec<u8>, output:&mut T)
// {
//     // let (head, body, tail) = bytes.align_to_mut::<T>();
//     // from_bytes::<T>(&mut bytes).clone()

//     // let boxed:&'static mut TU = transmute(from_bytes_static::<T>(&mut bytes));
//     let mut output_bytes = to_bytes_ref(output);
//     let mut index:usize = 0;
//     while index < bytes.len()
//     {
//         let output_byte = &mut output_bytes[index];
//         **output_byte = bytes[index];
//         index+=1;
//     }
// }