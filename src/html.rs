use crate::host;
use crate::{Result, Error};
use serde::de::DeserializeOwned;

pub struct Node {
    id: i32,
}

impl Node {
    pub fn new(html: &[u8]) -> Self {
        let id = unsafe { host::parse(html.as_ptr() as i32, html.len() as i32) };
        Self { id }
    }

    pub fn select(&self, selector: &str) -> Result<Vec<Node>> {
        let selector_bytes = selector.as_bytes();
        let packed = unsafe {
            host::select(
                self.id,
                selector_bytes.as_ptr() as i32,
                selector_bytes.len() as i32,
            )
        };
        
        let ids: Vec<i32> = unsafe { from_packed_ptr(packed)? };
        Ok(ids.into_iter().map(|id| Node { id }).collect())
    }

    pub fn text(&self) -> Result<String> {
        let packed = unsafe { host::text(self.id) };
        unsafe { from_packed_ptr(packed) }
    }
    
    pub fn own_text(&self) -> Result<String> {
        let packed = unsafe { host::own_text(self.id) };
        unsafe { from_packed_ptr(packed) }
    }

    pub fn html(&self) -> Result<String> {
        let packed = unsafe { host::html_content(self.id) };
        unsafe { from_packed_ptr(packed) }
    }

    pub fn outer_html(&self) -> Result<String> {
        let packed = unsafe { host::outer_html(self.id) };
        unsafe { from_packed_ptr(packed) }
    }

    pub fn attr(&self, name: &str) -> Result<Option<String>> {
        let name_bytes = name.as_bytes();
        let packed = unsafe { host::attr(self.id, name_bytes.as_ptr() as i32, name_bytes.len() as i32) };
        unsafe { from_packed_ptr(packed) }
    }
}

unsafe fn from_packed_ptr<T: DeserializeOwned>(packed: i64) -> Result<T> {
    let ptr = (packed >> 32) as i32;
    let len = (packed & 0xFFFFFFFF) as i32;
    
    if ptr == 0 || len <= 0 {
        return postcard::from_bytes(&[]).map_err(Error::Postcard);
    }
    
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len as usize) };
    let res = postcard::from_bytes(slice).map_err(Error::Postcard);
    
    unsafe { crate::ffi_alloc::dealloc(ptr as *mut u8, len as usize) };
    
    res
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { host::free(self.id) };
    }
}

