/// add two numbers together
#[no_mangle]
pub fn add(a: i32, b: i32) -> i32
{
    a + b
}

/// sum the elements of an array
#[no_mangle]
pub fn sum(ptr: i32, len: i32) -> i32
{
    let arr = unsafe { arr(ptr, len) };

    arr.iter().sum()
}

/// increment each element of an array by 1
#[no_mangle]
pub fn inc(ptr: i32, len: i32)
{
    let arr = unsafe { arr_mut(ptr, len) };

    for i in arr
    {
        *i += 1;
    }
}

/// reconstruct a slice from its offset and length in memory
unsafe fn arr<'a>(ptr: i32, len: i32) -> &'a [i32]
{
    use std::ptr::slice_from_raw_parts;

    &*slice_from_raw_parts(ptr as *const i32, len as usize)
}

/// reconstruct a slice from its offset and length in memory, mutably
unsafe fn arr_mut<'a>(ptr: i32, len: i32) -> &'a mut [i32]
{
    use std::ptr::slice_from_raw_parts_mut;

    &mut *slice_from_raw_parts_mut(ptr as *mut i32, len as usize)
}