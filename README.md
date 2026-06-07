# typst-djvu

Typst package to extract the DjVu OCR/text layer page-by-page and locate anchors.

**Route A (WASM):** The [`djvu`](https://crates.io/crates/djvu) pure-Rust crate parses `.djvu` bytes, reads each page’s `TXTz`/`TXTa` layer, and joins pages with form-feed (`\u{0C}`), matching `djvutxt`. A `wasm-minimal-protocol` plugin exposes `extract(bytes)`.

## Public API

```typ
#import "@local/djvu:0.1.0": djvu-pages, djvu-find

#let data = read("book.djvu", encoding: none)
#let pages = djvu-pages(data)
#let pg = djvu-find(pages, "E.4.3", "Unique Stabilizing Solution") // 1-based or none
```

- `djvu-pages(data)` — `data` is raw `.djvu` bytes; returns `array(str)` (one entry per page).
- `djvu-find(pages, ..needles)` — first 1-based page where all substrings appear **in order**; pure Typst.

## Build WASM

```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/djvu.wasm .
```

Place `djvu.wasm` next to `lib.typ` in this directory before compiling documents that import the package.

## Use in kalman.v

The thesis wires this repo in as a local Typst package:

```text
paper/packages/local/djvu/0.1.0 -> ../../../../tools/typst-djvu
```

Compile with the package search path pointing at `paper/packages`:

```bash
typst compile paper/thesis.typ paper/thesis.pdf \
  --root . \
  --package-path paper/packages
```

Fetch a prebuilt `djvu.wasm` from CI:

```bash
bash tools/fetch-djvu-wasm.sh
```

## Distribution

Main-branch CI uploads `djvu.wasm` to GitHub Release **`wasm-v0.1.0`**.

## Benchmark

`djvu-find` on cached `djvu-pages` output was benchmarked against Rust alternatives on the Kailath *Linear Estimation* DjVu. Results and harness live at git tag [`bench/kailath`](https://github.com/F1uctus/typst-djvu/tree/bench/kailath).
