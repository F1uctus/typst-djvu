pub mod core;

pub use core::{
    extract_joined, extract_pages, find_in_djvu, find_page, find_page_skipped, join_needles,
    needles_in_order, split_needles,
};

#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

/// Extract the document text layer as pages joined by form-feed (0x0C), matching `djvutxt`.
#[cfg(target_arch = "wasm32")]
#[wasm_func]
pub fn extract(djvu_bytes: &[u8]) -> Result<Vec<u8>, String> {
    core::extract_joined(djvu_bytes)
}

/// Packed needles use U+001E between entries. `skip` is UTF-8 digits only.
/// Returns ASCII page number (1-based within the skipped suffix), or empty bytes for `none`.
#[cfg(target_arch = "wasm32")]
#[wasm_func]
pub fn find(djvu_bytes: &[u8], skip: &[u8], packed_needles: &[u8]) -> Result<Vec<u8>, String> {
    let skip_pages = std::str::from_utf8(skip)
        .map_err(|e| format!("skip pages: {e}"))?
        .parse::<usize>()
        .map_err(|e| format!("skip pages: {e}"))?;
    let packed = std::str::from_utf8(packed_needles).map_err(|e| format!("needles: {e}"))?;
    let needles = core::split_needles(packed);
    match core::find_in_djvu(djvu_bytes, skip_pages, &needles)? {
        Some(page) => Ok(page.to_string().into_bytes()),
        None => Ok(Vec::new()),
    }
}
