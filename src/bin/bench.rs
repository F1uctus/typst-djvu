use std::path::PathBuf;
use std::time::{Duration, Instant};

use djvu::{extract_pages, find_in_djvu, find_page_skipped};

const DEFAULT_PATH: &str =
    "/mnt/d/Dev/rocq/unn-rocq-kalman/references/Kailath T., Sayed A., Hassibi B. - Linear Estimation.djvu";

const FRONT_SKIP: usize = 25;
const WARMUP: usize = 3;
const ITERS: usize = 30;

struct Query {
    anchor: &'static str,
    quoted: &'static str,
}

const QUERIES: &[Query] = &[
    Query {
        anchor: "E.4.3",
        quoted: "Unique Stabilizing Solution",
    },
    Query {
        anchor: "Theorem E.6.2",
        quoted: "Positive Definite Solution",
    },
    Query {
        anchor: "Theorem 14.5.1",
        quoted: "Sufficiency",
    },
];

fn mean_ms(total: Duration, samples: usize) -> f64 {
    total.as_secs_f64() * 1000.0 / samples as f64
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_PATH));
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("failed to read {}: {e}", path.display());
        std::process::exit(1);
    });

    let pages = extract_pages(&bytes).expect("extract pages");

    println!("file: {}", path.display());
    println!("pages: {}", pages.len());
    println!("front-skip: {FRONT_SKIP}");
    println!();

    for query in QUERIES {
        let needles = [query.anchor, query.quoted];
        let typst_style = find_page_skipped(&pages, FRONT_SKIP, &needles);
        let rust_style = find_in_djvu(&bytes, FRONT_SKIP, &needles).expect("find in djvu");
        println!(
            "query: {} + {:?} -> body={typst_style:?} absolute={:?}",
            query.anchor,
            query.quoted,
            typst_style.map(|hit| FRONT_SKIP + hit)
        );
        assert_eq!(typst_style, rust_style);
    }
    println!();

    let mut typst_extract = Duration::ZERO;
    let mut typst_find = Duration::ZERO;
    let mut rust_streaming = Duration::ZERO;
    let mut rust_cached_find = Duration::ZERO;

    for i in 0..WARMUP + ITERS {
        let t0 = Instant::now();
        let pages = extract_pages(&bytes).expect("extract pages");
        let extract_elapsed = t0.elapsed();

        let t1 = Instant::now();
        for query in QUERIES {
            let needles = [query.anchor, query.quoted];
            let _ = find_page_skipped(&pages, FRONT_SKIP, &needles);
        }
        let find_elapsed = t1.elapsed();

        let t2 = Instant::now();
        for query in QUERIES {
            let needles = [query.anchor, query.quoted];
            let _ = find_in_djvu(&bytes, FRONT_SKIP, &needles).expect("find in djvu");
        }
        let streaming_elapsed = t2.elapsed();

        let t3 = Instant::now();
        for query in QUERIES {
            let needles = [query.anchor, query.quoted];
            let _ = find_page_skipped(&pages, FRONT_SKIP, &needles);
        }
        let cached_find_elapsed = t3.elapsed();

        if i < WARMUP {
            continue;
        }
        typst_extract += extract_elapsed;
        typst_find += find_elapsed;
        rust_streaming += streaming_elapsed;
        rust_cached_find += cached_find_elapsed;
    }

    let queries_per_iter = QUERIES.len();

    println!("averages over {ITERS} iterations:");
    println!(
        "  typst path (extract + pure find, per query): {:.2} ms + {:.2} ms = {:.2} ms",
        mean_ms(typst_extract, ITERS),
        mean_ms(typst_find, ITERS * queries_per_iter),
        mean_ms(typst_extract, ITERS) / queries_per_iter as f64
            + mean_ms(typst_find, ITERS * queries_per_iter)
    );
    println!(
        "  rust find_in_djvu (stream pages, reparse each query): {:.2} ms/query",
        mean_ms(rust_streaming, ITERS * queries_per_iter)
    );
    println!(
        "  rust find_page_skipped (pages cached, per query): {:.2} ms/query",
        mean_ms(rust_cached_find, ITERS * queries_per_iter)
    );
}
