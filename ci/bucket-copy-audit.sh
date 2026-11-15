#!/usr/bin/env bash
# The audit of a bucket copied to another, server side (docs/datasets.md, "The
# bucket move"). Read-only: it lists both buckets and reads object checksums;
# it never writes or deletes.
#
#   ci/bucket-copy-audit.sh SOURCE_BUCKET DESTINATION_BUCKET [WORK_DIR]
#
#   WORK_DIR  both listings as JSON, and a file of keys for each kind of
#             difference; a temporary directory when not given
#
# Needs AWS CLI v2 and credentials that can list both buckets.
#
# For every key in the source:
#   1. the key exists in the destination, with the same size;
#   2. the ETag matches; where it does not (S3 changes the ETag of an object
#      uploaded or copied in parts), the full-object checksum S3 holds
#      (CRC64NVME or another) matches instead.
# An ETag is the MD5 of the object only when it was written in one part and the
# bucket uses SSE-S3 or no encryption: check both before reading a match as
# "the same bytes". Keys that exist only in the destination are reported and
# do not fail the audit. Exit 0 only when every source object is accounted for.
set -euo pipefail
src="$1"; dst="$2"; work="${3:-$(mktemp -d)}"
mkdir -p "$work"
for b in "$src" "$dst"; do
  echo "listing s3://$b ..." >&2
  aws s3api list-objects-v2 --bucket "$b" --output json \
    --query 'Contents[].[Key,Size,ETag]' > "$work/$b.json"
done
python3 - "$src" "$dst" "$work" <<'PY'
import json, subprocess, sys
src, dst, work = sys.argv[1:4]
def load(b):
    rows = json.load(open(f"{work}/{b}.json")) or []
    return {k: (int(size), etag) for k, size, etag in rows}
a, b = load(src), load(dst)
print(f"source      s3://{src}: {len(a):>9,} objects, {sum(s for s, _ in a.values()):>16,} bytes")
print(f"destination s3://{dst}: {len(b):>9,} objects, {sum(s for s, _ in b.values()):>16,} bytes")
missing = sorted(k for k in a if k not in b)
extra = sorted(k for k in b if k not in a)
size_diff = sorted(k for k in a if k in b and a[k][0] != b[k][0])
etag_diff = sorted(k for k in a if k in b and a[k][0] == b[k][0] and a[k][1] != b[k][1])
def checksum(bucket, key):
    out = subprocess.run(
        ["aws", "s3api", "head-object", "--bucket", bucket, "--key", key,
         "--checksum-mode", "ENABLED", "--output", "json"],
        capture_output=True, text=True, check=True).stdout
    head = json.loads(out)
    return {k: v for k, v in head.items() if k.startswith("Checksum") and k != "ChecksumType"}
unverified, bad = [], []
for i, key in enumerate(etag_diff, 1):
    ca, cb = checksum(src, key), checksum(dst, key)
    common = set(ca) & set(cb)
    if not common:
        unverified.append(key)
    elif any(ca[c] != cb[c] for c in common):
        bad.append(key)
    if i % 200 == 0:
        print(f"  checksums compared: {i}/{len(etag_diff)}", file=sys.stderr)
print(f"same key, size and ETag:            {len(a) - len(missing) - len(size_diff) - len(etag_diff):>9,}")
print(f"ETag differs, checksum matches:     {len(etag_diff) - len(unverified) - len(bad):>9,}")
print(f"ETag differs, no common checksum:   {len(unverified):>9,}   <- compare these another way")
print(f"ETag differs, checksum DIFFERS:     {len(bad):>9,}")
print(f"missing from the destination:       {len(missing):>9,}")
print(f"size differs:                       {len(size_diff):>9,}")
print(f"only in the destination:            {len(extra):>9,}")
for name, keys in (("missing", missing), ("size-differs", size_diff), ("checksum-differs", bad),
                   ("unverified", unverified), ("only-in-destination", extra)):
    if keys:
        path = f"{work}/{name}.txt"
        open(path, "w").write("\n".join(keys) + "\n")
        print(f"  {name}: {path}")
if not a:
    print(f"the source s3://{src} lists no objects: wrong bucket name, or nothing to verify")
ok = bool(a) and not (missing or size_diff or bad or unverified)
print("RESULT:", "every source object is in the destination and matches" if ok else "NOT VERIFIED")
sys.exit(0 if ok else 1)
PY
