#let _p = plugin("djvu.wasm")

/// Bytes of a `.djvu` file (`read(..., encoding: none)`). One string per page (form-feed separated in WASM).
#let djvu-pages(data) = str(_p.extract(data)).split("\u{0C}")

#let _needles-in-order(page, needles) = {
  let step(rest, ns) = if ns.len() == 0 {
    true
  } else {
    let needle = str(ns.first())
    let at = rest.position(needle)
    if at == none {
      false
    } else {
      step(rest.slice(at + needle.len(), none), ns.slice(1, none))
    }
  }
  step(page, needles)
}

/// First 1-based page whose text contains all `needles` in order; else `none`.
#let djvu-find(pages, ..needles) = {
  let needles = needles.pos()
  for (i, page) in pages.enumerate() {
    if _needles-in-order(page, needles) {
      return i + 1
    }
  }
  none
}

#let _needle-sep = "\u{001e}"

/// Same as `djvu-find`, but searches a `.djvu` byte string via the Rust plugin.
/// `skip` drops leading pages before search (front matter). Returns 1-based index
/// within the skipped suffix, or `none`.
#let djvu-find-bytes(data, skip: 0, ..needles) = {
  let packed = needles.pos().map(str).join(_needle-sep)
  let hit = bytes(_p.find(data, str(skip), packed))
  if hit.len() == 0 {
    none
  } else {
    int(str(hit))
  }
}
