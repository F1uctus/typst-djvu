# typst-djvu

Typst package to extract the DjVu OCR/text layer page-by-page and locate anchors.

**Route A (WASM):** The [`djvu`](https://crates.io/crates/djvu) pure-Rust crate parses `.djvu` bytes, reads each page’s `TXTz`/`TXTa` layer, and joins pages with form-feed (`\u{0C}`), matching `djvutxt`. A `wasm-minimal-protocol` plugin exposes `extract(bytes)` and `find(bytes, skip, needles)`.

## Public API (Typst)

```typ
#import "@preview/djvu:0.1.0": djvu-pages, djvu-find, djvu-find-bytes

#let data = read("book.djvu", encoding: none)
#let pages = djvu-pages(data)
#let pg = djvu-find(pages, "E.4.3", "Unique Stabilizing Solution") // 1-based or none
#let pg-fast = djvu-find-bytes(data, skip: 25, "E.4.3", "Unique Stabilizing Solution")
```

- `djvu-pages(data)` — `data` is raw `.djvu` bytes; returns `array(str)` (one entry per page).
- `djvu-find(pages, ..needles)` — first 1-based page where all substrings appear **in order**; pure Typst.
- `djvu-find-bytes(data, skip: 0, ..needles)` — same search via the Rust plugin without building a page array first.

## Rust library

The crate also builds as an `rlib` for native Rust callers:

```rust
use djvu::{extract_pages, find_in_djvu, find_page_skipped, needles_in_order};

let bytes = std::fs::read("book.djvu")?;
let pages = extract_pages(&bytes)?;
let page_refs: Vec<&str> = pages.iter().map(String::as_str).collect();

// Typst-style path: materialize pages, then search the body slice.
let hit = find_page_skipped(&page_refs, 25, &["E.4.3", "Unique Stabilizing Solution"]);

// Streaming path: scan pages in the DjVu file without allocating all page strings.
let hit = find_in_djvu(&bytes, 25, &["E.4.3", "Unique Stabilizing Solution"])?;
```

## Build WASM

```bash
cargo build --target wasm32-unknown-unknown --release --lib
cp target/wasm32-unknown-unknown/release/djvu.wasm .
```

## Benchmark

Run against the Kailath reference book:

```bash
./bench/run.sh /path/to/Kailath\ T.,\ Sayed\ A.,\ Hassibi\ B.\ -\ Linear\ Estimation.djvu
```

### Results

Recorded at git tag [`bench/kailath`](https://github.com/F1uctus/typst-djvu/tree/bench/kailath).

Corpus: Kailath *Linear Estimation* DjVu, 875 pages, `front-skip: 25`.

| Query | Body page | Absolute page |
|-------|-----------|---------------|
| `E.4.3` + `Unique Stabilizing Solution` | 781 | 806 |
| `Theorem E.6.2` + `Positive Definite Solution` | 784 | 809 |
| `Theorem 14.5.1` + `Sufficiency` | 514 | 539 |

Timings (30 iterations, 3 queries each):

| Route | Time per query |
|-------|----------------|
| Typst (`djvu-pages` + pure `djvu-find`) | ~64 ms (188 ms extract + 1 ms find, amortized) |
| Rust `find_in_djvu` (stream pages, reparse each query) | ~149 ms |
| Rust `find_page_skipped` (pages cached) | ~1 ms |

Typst compile of `bench/find.typ` (3 queries): ~2.8 s wall; WASM `extract` dominates (~2.36 s). Pure Typst search is ~1 ms/query once pages are materialized.

**Conclusion:** keep `djvu-find` on cached `djvu-pages` output. The Rust streaming route does not beat the Typst path end-to-end here.

## Distribution

Main-branch CI uploads `djvu.wasm` to GitHub Release **`wasm-v0.1.0`**.

To use the package, copy `lib.typ`, `typst.toml`, and `djvu.wasm` into a Typst package directory and import from there.
