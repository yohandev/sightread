/// allocate an `i32[]` of length `len`, returning
/// the `ptr` in wasm memory
///
/// fills the array with zero by default.
#[no_mangle]
pub fn alloc_arr_i32(len: i32) -> i32
{
    // leak the allocation
    Box::into_raw(vec![0i32; len as usize].into_boxed_slice()) as *mut () as i32
}

/// allocate an `f32[]` of length `len`, returning
/// the `ptr` in wasm memory
///
/// fills the array with zero by default.
#[no_mangle]
pub fn alloc_arr_f32(len: i32) -> i32
{
    // leak the allocation
    Box::into_raw(vec![0.0f32; len as usize].into_boxed_slice()) as *mut () as i32
}

/// reconstruct a slice from its offset and length in wasm memory
pub unsafe fn reconstruct_slice<'a, T>(ptr: i32, len: i32) -> &'a [T]
{
    use std::ptr::slice_from_raw_parts;

    &*slice_from_raw_parts(ptr as *const i32 as *const T, len as usize)
}

/// reconstruct a slice from its offset and length in wasm memory
pub unsafe fn reconstruct_slice_mut<'a, T>(ptr: i32, len: i32) -> &'a mut [T]
{
    use std::ptr::slice_from_raw_parts_mut;

    &mut *slice_from_raw_parts_mut(ptr as *mut i32 as *mut T, len as usize)
}