#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../../.." && pwd)"
repo="$(cd "$(dirname "$0")/.." && pwd)"
djvu="${1:-$root/references/Kailath T., Sayed A., Hassibi B. - Linear Estimation.djvu}"

if [[ ! -f "$djvu" ]]; then
  echo "missing djvu file: $djvu" >&2
  exit 1
fi

echo "== typst-djvu find benchmark =="
echo "djvu: $djvu"
echo

echo "== correctness (typst query) =="
typst query "$repo/bench/find.typ" "<bench>" --field value --root "$root"
echo

echo "== rust bench =="
cargo run --quiet --manifest-path "$repo/Cargo.toml" --release --bin djvu-bench -- "$djvu"
echo

echo "== typst timings =="
timings="$(mktemp)"
out="$(mktemp --suffix=.pdf)"
start=$(date +%s.%N)
typst compile "$repo/bench/find.typ" "$out" --root "$root" --timings "$timings" >/dev/null
end=$(date +%s.%N)
printf 'wall %.3f s\n' "$(awk -v s="$start" -v e="$end" 'BEGIN { print e - s }')"
rm -f "$out"
if [[ -f "$HOME/.claude/skills/typst/scripts/perf-timings.py" ]]; then
  python3 "$HOME/.claude/skills/typst/scripts/perf-timings.py" "$timings" --top 8 --self-time
else
  echo "timings trace: $timings"
fi
