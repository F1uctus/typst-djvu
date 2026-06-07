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

## Local package install

Per [Typst Local Packages](https://github.com/typst/packages/blob/main/README.md#local-packages), store this repo at `{data-dir}/typst/packages/local/djvu/0.1.0` or pass `--package-path` to a parent of `local/djvu/0.1.0`.

Clone a tagged release (includes `lib.typ`, `typst.toml`, and `djvu.wasm`):

```bash
mkdir -p paper/packages/local/djvu
rm -rf paper/packages/local/djvu/0.1.0
git clone --depth 1 --branch v0.1.0 \
  https://github.com/F1uctus/typst-djvu \
  paper/packages/local/djvu/0.1.0
```

Or symlink a checkout:

```bash
ln -sf "$(pwd)/tools/typst-djvu" paper/packages/local/djvu/0.1.0
```

Compile with the package search path:

```bash
typst compile paper/thesis.typ paper/thesis.pdf \
  --root . \
  --package-path paper/packages
```

## Build WASM

When Rust sources change, rebuild and commit `djvu.wasm` beside `lib.typ`:

```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/djvu.wasm .
```

CI verifies the committed binary matches a fresh build.

## Benchmark

`djvu-find` on cached `djvu-pages` output was benchmarked against Rust alternatives on the Kailath *Linear Estimation* DjVu. Results and harness live at git tag [`bench/kailath`](https://github.com/F1uctus/typst-djvu/tree/bench/kailath).
