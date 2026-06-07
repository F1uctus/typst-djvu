#let _p = plugin("/djvu.wasm")

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
