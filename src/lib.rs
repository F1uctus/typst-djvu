use wasm_minimal_protocol::wasm_func;

wasm_minimal_protocol::initiate_protocol!();

/// Extract the document text layer as pages joined by form-feed (0x0C), matching djvutxt.
#[wasm_func]
pub fn extract(djvu_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let doc = djvu::Document::from_bytes(djvu_bytes.to_vec())
        .map_err(|e| format!("failed to parse DjVu: {e}"))?;
    let mut out = Vec::new();
    for i in 0..doc.page_count() {
        if i > 0 {
            out.push(0x0C);
        }
        let page = doc.page(i).map_err(|e| format!("page {i}: {e}"))?;
        if let Some(text) = page
            .text()
            .map_err(|e| format!("page {i} text layer: {e}"))?
        {
            out.extend_from_slice(text.as_bytes());
        }
    }
    Ok(out)
}
