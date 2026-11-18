#!/usr/bin/env python3
"""The byte-for-byte check of a bucket copied to another (docs/datasets.md, "The
bucket move"), next to ci/bucket-copy-audit.sh. Read-only: it lists the source
and downloads the sample from both buckets; it never writes or deletes in S3.

    ci/bucket-copy-sample.py [--prefix PREFIX] [--fraction F] [--seed N] [--all]
                             [--jobs J] SOURCE_BUCKET DESTINATION_BUCKET [WORK_DIR]

The copy audit compares every object's key, size and ETag, which S3 computes
itself. This check does not trust S3's metadata: it downloads objects from both
buckets and compares their SHA-256. The sample is stratified so that no
partition goes unchecked:

  - one random object from every directory under PREFIX (for the positions: every
    task's positions, games and manifests of every run and label set, and every
    task.json);
  - plus a random FRACTION (default 0.005) of all objects on top;
  - or every object, with --all (for the nets: 137 objects, 48 MB).

The seed (default: today's date, printed) makes a sample repeatable. WORK_DIR
(a temporary directory when not given) gets sample.json: every sampled key with
its size and both SHA-256s, for the dataset's ledger. Exit 0 only when every
sampled object exists in both buckets, has the listed size in the source, and
has the same SHA-256 in both.

Needs AWS CLI v2 and credentials that can read both buckets.
"""

import argparse
import datetime
import hashlib
import json
import os
import random
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor


def stratum(key):
    """The directory an object is in: the unit the sample must cover."""
    return key.rsplit('/', 1)[0] if '/' in key else ''


def select(objects, fraction, seed, everything=False):
    """The sorted sample: one random key per directory, plus `fraction` of all
    keys at random (or every key)."""
    keys = sorted(objects)
    if everything:
        return keys
    rng = random.Random(seed)
    by_stratum = {}
    for key in keys:
        by_stratum.setdefault(stratum(key), []).append(key)
    chosen = {rng.choice(members) for _, members in sorted(by_stratum.items())}
    extra = min(len(keys), round(fraction * len(keys)))
    chosen.update(rng.sample(keys, extra))
    return sorted(chosen)


def compare(keys, sizes, source, destination, fetch, jobs=1):
    """Download each key from both buckets and compare. `fetch(bucket, key)`
    returns the object's bytes, or None when it does not exist."""

    def one(key):
        a, b = fetch(source, key), fetch(destination, key)
        if a is None or b is None:
            return key, 'missing', None, None, 0
        sa, sb = hashlib.sha256(a).hexdigest(), hashlib.sha256(b).hexdigest()
        if len(a) != sizes[key]:
            return key, 'size_differs', sa, sb, len(a)
        return key, 'matched' if sa == sb else 'differs', sa, sb, len(a)

    result = {'matched': 0, 'differs': [], 'missing': [], 'size_differs': [], 'bytes': 0, 'objects': {}}
    with ThreadPoolExecutor(max_workers=jobs) as pool:
        for i, (key, verdict, sa, sb, size) in enumerate(pool.map(one, keys), 1):
            result['objects'][key] = {'size': size, 'source_sha256': sa, 'destination_sha256': sb, 'verdict': verdict}
            if verdict == 'matched':
                result['matched'] += 1
                result['bytes'] += size
            else:
                result[verdict].append(key)
            if i % 500 == 0:
                print(f'  compared {i:,}/{len(keys):,}', file=sys.stderr)
    for name in ('differs', 'missing', 'size_differs'):
        result[name].sort()
    return result


def ok(result):
    return result['matched'] > 0 and not (result['differs'] or result['missing'] or result['size_differs'])


def s3_fetch(bucket, key):
    run = subprocess.run(['aws', 's3', 'cp', '--only-show-errors', f's3://{bucket}/{key}', '-'], capture_output=True)
    if run.returncode != 0:
        if b'404' in run.stderr or b'Not Found' in run.stderr or b'NoSuchKey' in run.stderr:
            return None
        raise RuntimeError(f's3://{bucket}/{key}: {run.stderr.decode(errors="replace").strip()}')
    return run.stdout


def s3_list(bucket, prefix):
    out = subprocess.run(
        ['aws', 's3api', 'list-objects-v2', '--bucket', bucket, '--prefix', prefix,
         '--query', 'Contents[].[Key,Size]', '--output', 'json'],
        capture_output=True, text=True, check=True).stdout
    return {key: int(size) for key, size in (json.loads(out) or [])}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    parser.add_argument('source')
    parser.add_argument('destination')
    parser.add_argument('work', nargs='?')
    parser.add_argument('--prefix', default='')
    parser.add_argument('--fraction', type=float, default=0.005)
    parser.add_argument('--seed', type=int, default=int(datetime.date.today().strftime('%Y%m%d')))
    parser.add_argument('--all', action='store_true', help='every object, not a sample')
    parser.add_argument('--jobs', type=int, default=16)
    args = parser.parse_args()
    if not 0 <= args.fraction <= 1:
        parser.error('--fraction must be between 0 and 1')
    work = args.work or tempfile.mkdtemp()
    os.makedirs(work, exist_ok=True)

    print(f'listing s3://{args.source}/{args.prefix} ...', file=sys.stderr)
    sizes = s3_list(args.source, args.prefix)
    if not sizes:
        print(f'the source s3://{args.source}/{args.prefix} lists no objects: wrong bucket name or prefix')
        return 1
    keys = select(sizes, args.fraction, args.seed, everything=args.all)
    strata = {stratum(k) for k in sizes}
    print(f'source s3://{args.source}/{args.prefix}: {len(sizes):,} objects in {len(strata):,} directories')
    how = 'every object' if args.all else f'one per directory plus {args.fraction:.2%} at random, seed {args.seed}'
    print(f'sample: {len(keys):,} objects ({how}), {sum(sizes[k] for k in keys):,} bytes from each bucket')

    result = compare(keys, sizes, args.source, args.destination, s3_fetch, jobs=args.jobs)
    covered = {stratum(k) for k in keys}
    print(f'directories covered:                {len(covered):>9,} of {len(strata):,}')
    print(f'same SHA-256 in both buckets:       {result["matched"]:>9,}  ({result["bytes"]:,} bytes)')
    print(f'SHA-256 DIFFERS:                    {len(result["differs"]):>9,}')
    print(f'missing from a bucket:              {len(result["missing"]):>9,}')
    print(f'source size differs from listing:   {len(result["size_differs"]):>9,}')
    record = {
        'source': args.source, 'destination': args.destination, 'prefix': args.prefix,
        'seed': args.seed, 'fraction': args.fraction, 'all': args.all,
        'listed_objects': len(sizes), 'directories': len(strata), 'directories_covered': len(covered),
        **result,
    }
    with open(os.path.join(work, 'sample.json'), 'w') as f:
        json.dump(record, f, indent=1, sort_keys=True)
    print(f'  record: {os.path.join(work, "sample.json")}')
    passed = ok(result) and covered == strata
    print('RESULT:', 'every sampled object is byte-identical in both buckets' if passed else 'NOT VERIFIED')
    return 0 if passed else 1


if __name__ == '__main__':
    sys.exit(main())
