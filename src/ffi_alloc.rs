use std::alloc::Layout;

#[unsafe(no_mangle)]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::null_mut(); // Return a null pointer for zero size
    }
    let layout = Layout::from_size_align(size, 1).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    ptr
}

/// Deallocates memory previously allocated by `alloc`.
///
/// # Safety
/// This function is unsafe because it takes a raw pointer and size. 
/// The caller must ensure that `ptr` was allocated by `alloc` with the exact same `size`,
/// and that the pointer is not used after being deallocated.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    if size == 0 || ptr.is_null() {
        return;
    }
    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout) };
}
