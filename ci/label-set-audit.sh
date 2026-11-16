#!/usr/bin/env bash
# The full audit of a label set, run locally on a download of it (read-only;
# docs/relabel.md, "Auditing a label set"):
#
#   ci/label-set-audit.sh s3://BUCKET/data/selfplay LABEL_ID GENERATIONS OUT_DIR [--skip-determinism]
#
#   LABEL_ID     the label set, e.g. net7-l050-r41363264869-n5000
#   GENERATIONS  numbers separated by commas, e.g. 0,1,2
#   OUT_DIR      data/ (a resumable copy of both sides), build/, det/ and
#                summary.json; kept, so a second run downloads only what changed
#
# 1. downloads the manifests and positions of the originals and of the label set
# 2. builds `relabel` from the commit that made the label set, in OUT_DIR/build,
#    whatever is checked out here, and checks it prints LABEL_ID: its
#    selfplay_sha in this repository, or, for a label set made in the engine
#    repository (no selfplay_sha), its engine_sha there, fetched from GitHub
# 3. ci/label-set-audit.py: inventory, every record against its source,
#    provenance and checksums, plausibility, and determinism with a negative
#    control
#
# AWS credentials must be able to read the prefix. Exits 1 on any failure, and 2
# if determinism was skipped, which is never a pass. Needs Rust, git, the AWS CLI
# and python3.
set -euo pipefail
root=${1:?an s3://BUCKET/data/selfplay root}
root=${root%/}
label=${2:?a label set id}
gens=${3:?generations, e.g. 0,1,2}
out=${4:?an output directory}
extra=${5:-}
case "$root" in s3://*) ;; *) echo "label-set-audit: the root must be an s3:// URI" >&2; exit 2 ;; esac
[[ "$gens" =~ ^[0-9]+(,[0-9]+)*$ ]] || { echo "label-set-audit: generations must be numbers separated by commas" >&2; exit 2; }
repo="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$out"
out=$(cd "$out" && pwd)

echo "==> downloading generations $gens of the originals and of $label"
IFS=',' read -ra gen_list <<< "$gens"
for g in "${gen_list[@]}"; do
  aws s3 sync --only-show-errors "$root/v1/gen=$g/" "$out/data/source/gen=$g/" \
    --exclude '*' --include '*/manifests/*' --include '*/positions/*' &
  aws s3 sync --only-show-errors "$root/relabel/$label/gen=$g/" "$out/data/labels/gen=$g/" \
    --exclude '*' --include '*/manifests/*' --include '*/positions/*' &
done
wait

first=$(find "$out/data/labels" -path '*/manifests/*.json' -print -quit)
[ -n "$first" ] || { echo "label-set-audit: no manifests for $label" >&2; exit 1; }
read -r sha selfplay_sha < <(python3 -I -c 'import json, sys; r = json.load(open(sys.argv[1]))["relabel"]; print(r["engine_sha"], r.get("selfplay_sha") or "")' "$first")
rm -rf "$out/build/src"; mkdir -p "$out/build/src"
if [ -n "$selfplay_sha" ]; then
  echo "==> building relabel at braveheart-selfplay ${selfplay_sha:0:7}, the commit in the manifests (engine ${sha:0:7})"
  git -C "$repo" archive "$selfplay_sha" | tar -x -C "$out/build/src"
  package=()
else
  # Made in the engine repository, where engine_sha is the commit that held relabel.
  echo "==> building relabel at the engine repository's ${sha:0:7}, the commit in the manifests"
  rm -rf "$out/build/engine.git"
  git init -q --bare "$out/build/engine.git"
  git -C "$out/build/engine.git" fetch -q --depth 1 https://github.com/Braveheart-Chess/braveheart "$sha"
  git -C "$out/build/engine.git" archive "$sha" | tar -x -C "$out/build/src"
  package=(-p braveheart-tools)
fi
(cd "$out/build/src" && RUSTFLAGS="${RUSTFLAGS:--C target-cpu=native}" CARGO_TARGET_DIR="$out/build/target" \
  cargo build --release --locked --quiet ${package[@]+"${package[@]}"} --bin relabel)
relabel="$out/build/target/release/relabel"
nodes=${label##*-n}
built=$("$relabel" --net default --nodes "$nodes" --print-label-id)
[ "$built" = "$label" ] || { echo "label-set-audit: $sha builds $built, not $label" >&2; exit 1; }

python3 -I "$repo/ci/label-set-audit.py" --data "$out/data" --work "$out/det" --label-id "$label" --engine-sha "$sha" \
  ${selfplay_sha:+--selfplay-sha "$selfplay_sha"} \
  --relabel "$relabel" --dataset "$root/v1" --json "$out/summary.json" $extra
