//! Native DjVu text extraction and ordered substring search.

const NEEDLE_SEP: char = '\u{001e}';

/// Split packed needles on the record separator used by the WASM `find` export.
pub fn split_needles(packed: &str) -> Vec<&str> {
    packed.split(NEEDLE_SEP).collect()
}

/// Join needles for the WASM `find` export.
pub fn join_needles(needles: &[&str]) -> String {
    needles.join(&NEEDLE_SEP.to_string())
}

/// Extract one string per page from a DjVu document.
pub fn extract_pages(djvu_bytes: &[u8]) -> Result<Vec<String>, String> {
    let doc = djvu_parser::Document::from_bytes(djvu_bytes.to_vec())
        .map_err(|e| format!("failed to parse DjVu: {e}"))?;
    let mut pages = Vec::with_capacity(doc.page_count());
    for i in 0..doc.page_count() {
        let page = doc.page(i).map_err(|e| format!("page {i}: {e}"))?;
        let text = match page.text().map_err(|e| format!("page {i} text layer: {e}"))? {
            Some(text) => text.to_owned(),
            None => String::new(),
        };
        pages.push(text);
    }
    Ok(pages)
}

/// Extract the document text layer as pages joined by form-feed (0x0C), matching `djvutxt`.
pub fn extract_joined(djvu_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let doc = djvu_parser::Document::from_bytes(djvu_bytes.to_vec())
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

/// Whether `page` contains every needle in order.
pub fn needles_in_order(page: &str, needles: &[&str]) -> bool {
    let mut rest = page;
    for needle in needles {
        let Some(at) = rest.find(needle) else {
            return false;
        };
        rest = &rest[at + needle.len()..];
    }
    true
}

/// First 1-based page index in `pages` whose text contains all `needles` in order.
pub fn find_page(pages: &[impl AsRef<str>], needles: &[&str]) -> Option<usize> {
    for (i, page) in pages.iter().enumerate() {
        if needles_in_order(page.as_ref(), needles) {
            return Some(i + 1);
        }
    }
    None
}

/// Like [`find_page`], but search only `pages[skip..]`. The returned index is still
/// 1-based within that suffix (matching Typst `pages.slice(skip)` + `djvu-find`).
pub fn find_page_skipped(
    pages: &[impl AsRef<str>],
    skip: usize,
    needles: &[&str],
) -> Option<usize> {
    find_page(pages.get(skip..)?, needles)
}

/// Search a DjVu file page-by-page without materializing all page strings first.
pub fn find_in_djvu(djvu_bytes: &[u8], skip_pages: usize, needles: &[&str]) -> Result<Option<usize>, String> {
    let doc = djvu_parser::Document::from_bytes(djvu_bytes.to_vec())
        .map_err(|e| format!("failed to parse DjVu: {e}"))?;
    for i in skip_pages..doc.page_count() {
        let page = doc.page(i).map_err(|e| format!("page {i}: {e}"))?;
        if let Some(text) = page.text().map_err(|e| format!("page {i} text layer: {e}"))? {
            if needles_in_order(text.as_str(), needles) {
                return Ok(Some(i + 1 - skip_pages));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needles_in_order_basic() {
        assert!(needles_in_order("abc def ghi", &["abc", "def"]));
        assert!(needles_in_order("abc def ghi", &["def", "ghi"]));
        assert!(!needles_in_order("abc def ghi", &["ghi", "def"]));
        assert!(!needles_in_order("abc def ghi", &["xyz"]));
    }

    #[test]
    fn join_and_split_needles_roundtrip() {
        let needles = ["E.4.3", "Unique Stabilizing Solution"];
        let packed = join_needles(&needles);
        assert_eq!(split_needles(&packed), needles);
    }
}
