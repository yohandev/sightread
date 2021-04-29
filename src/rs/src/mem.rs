use std::alloc::{ Layout, alloc, dealloc };

/// use the Wee Allocator for smaller binary footprint
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// allocate an array, length `len`, of 32-bit aligned elements
/// leaks memory, must be freed manually via [dealloc32] and
/// returns a pointer to the start of the array in WASM's
/// linear memory
#[no_mangle]
pub unsafe fn alloc32(len: u32) -> u32
{
    const SIZE: usize = core::mem::size_of::<i32>();
    const ALIGN: usize = core::mem::align_of::<i32>();

    alloc(Layout::from_size_align_unchecked(SIZE * len as usize, ALIGN)) as _
}

/// free an array, length `len`, of 32-bit aligned elements in
/// WASM linear-memory
#[no_mangle]
pub unsafe fn dealloc32(ptr: u32, len: u32)
{
    const SIZE: usize = core::mem::size_of::<i32>();
    const ALIGN: usize = core::mem::align_of::<i32>();

    dealloc(ptr as _, Layout::from_size_align_unchecked(SIZE * len as usize, ALIGN))
}