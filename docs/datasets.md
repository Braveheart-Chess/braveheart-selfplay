# Self-play datasets

> **The ledger since the data move.** The positions were copied, key for key,
> to this repository's bucket on 2026-11-16 and audited
> ([the bucket move](#the-bucket-move)); from then on this page is the ledger,
> and every run is recorded here. It was refreshed from the engine's
> [docs/datasets.md](https://github.com/Braveheart-Chess/braveheart/blob/main/docs/datasets.md)
> at the move, which had not changed since this copy was taken at engine commit
> `1a25b13`. Datagen and Relabel ran from neither repository from the copy until
> the cutover ([AGENTS.md](../AGENTS.md#until-the-cutover)), when the engine gave
> its dataset up (engine #277, merged 2026-11-18). The first runs from this
> repository are that day's proving runs:
> [generation 7's smoke test](#generation-7-net8x160) and
> [a relabel of part of generation 6](#net8x160-l050-r43390801430-n5000), since
> deleted.

The ledger of every self-play generation, every label set and every run that
made them. Each net in [nets.md](https://github.com/Braveheart-Chess/braveheart-trainer/blob/main/docs/nets.md) names the generations and labels it
trained on; this page says what is in them. How the data is made and laid out is in
[datagen.md](datagen.md), and how it is checked in
[datagen-runbook.md](datagen-runbook.md).

Original data is in `s3://jaws-braveheart-selfplay-positions/data/selfplay/v1/gen=N/run=RUN/`,
in the production account, and [other label sets](#label-sets) are under
`data/selfplay/relabel/`. Both are append-only and kept forever. Records made
before 2026-11-19 name the same keys in the earlier buckets
([the bucket move](#the-bucket-move)). Every number here comes from the shard manifests
or from `datagen-audit`, never from the workflow's progress lines.

## Generations

| Gen | Played by | Positions | Games | Shards | Runs | Full audit |
|--:|:--|--:|--:|--:|--:|:--|
| 0 | PeSTO (`v0.24.0`–`v0.25.0`) | 231,240,344 | 2,387,961 | 9,329 | 3 | GO: 0 P0 failures |
| 1 | net 1 (`v0.29.0`) | 368,354,466 | 3,703,015 | 14,716 | 3 | GO: 0 P0 failures, 0 P1 warnings |
| 2 | net2x80 (`v0.31.1`) | 307,037,810 | 3,265,807 | 12,353 | 1 | GO: 0 P0 problems; 2,304/2,304 games regenerated from their seeds |
| 3 | net4w512 (`v0.36.0`) | 628,213,254 | 6,562,925 | 25,158 | 1 | GO: 0 P0 problems, 0 P1 warnings; 2,304/2,304 games regenerated from their seeds |
| 4 | net 5 (`v0.37.1`) | 495,147,759 | 5,252,191 | 19,838 | 2 | GO: 0 P0 problems, 0 P1 warnings; 3,072/3,072 games regenerated from their seeds |
| 5 | net6k4 (`v0.42.0`) | 318,934,530 | 3,409,848 | 12,755 | 1 | GO: 0 P0 problems, 0 P1 warnings; 2,304/2,304 games regenerated from their seeds |
| 6 | net7rl012 (`v0.47.1`) | 849,868,621 | 9,662,116 | 34,053 | 5 | GO: 0 P0 problems, 0 P1 warnings; 3,840/3,840 games regenerated from their seeds |
| 7 | net8x160 (`v0.54.0`) | 5,905,655 | 66,603 | 256 | 1 | Not run: closed at its smoke test, since `main` moved off `afcfc85`. The next campaign is generation 8 (not started) |

**Generation 7** is played by the pinned engine's default net, net8x160
(`net8x160-l050-r43390801430`) at `v0.54.0`. `datagen.toml` says
`generation = 7`: it was bumped with net8 (engine #235), and net8x160 replaced
net8 before either played a generation. So far it holds only the smoke test of
2026-11-18 ([Generation 7](#generation-7-net8x160)), and nothing trains on it
before its full audit.

What labels each generation has, and which nets trained on them, is under
[Labels](#labels).

## Labels

Every record is `FEN | score | result`. So what a net trained on takes two
choices, and naming a generation alone doesn't identify it:

- **Positions:** a generation's FENs and game results. These never change.
- **Labels:** each record's `score`, supplied by a **label set**. A generation
  can have several, and its original labels are never changed.

The training target is `λ × result + (1 − λ) × sigmoid(score / 400)`, so a label
set changes only the second term. Always name the generations and the label set
of each.

### Label sets

| Key | Label set id | Scores from | Nodes | Location | Status |
|:--|:--|:--|--:|:--|:--|
| `O` | `original` | The net that played each generation, so a mix (below) | 5,000 | `data/selfplay/v1/gen=N/` | Complete, kept forever |
| `N7` | `net7-l050-r41363264869-n5000` | net7, sha256 `0866bde1…1b6f`, for every generation | 5,000 | `data/selfplay/relabel/net7-l050-r41363264869-n5000/gen=N/` | Generations 0–2 audited; generations 3–5 not started |
| `N7r` | `net7rl012-l050-r42054816508-n5000` | net7rl012, sha256 `74af5b46…70ce`, for every generation | 5,000 | `data/selfplay/relabel/net7rl012-l050-r42054816508-n5000/gen=N/` | Generations 3–5 audited; generations 0–2 not started |
| `N8x` | `net8x160-l050-r43390801430-n5000` | net8x160, sha256 `8ea67605…556c`, for every generation | 5,000 | `data/selfplay/relabel/net8x160-l050-r43390801430-n5000/gen=N/` | Deleted 2026-11-18: a proving run's part of generation 6 (618 of 34,053 shards), removed so the id starts clean |

- **`original`** is the name used here for the labels made during play. Those
  prefixes aren't called that in S3, and `Train`'s `labels` input doesn't take
  it: a generation left out of `labels` uses its originals.
- **Any other id** is `<net id>-n<nodes>` and means one labelling, forever. A
  change of net, nodes or method is a new id.

### Which label sets exist

| Gen | Positions | `O`, labelled by | `N7` | `N7r` | `N8x` |
|--:|--:|:--|:--|:--|:--|
| 0 | 231,240,344 | PeSTO | Audited | Not started | Not started |
| 1 | 368,354,466 | net 1 | Audited | Not started | Not started |
| 2 | 307,037,810 | net2x80 | Audited | Not started | Not started |
| 3 | 628,213,254 | net4w512 | Not started | Audited | Not started |
| 4 | 495,147,759 | net 5 | Not started | Audited | Not started |
| 5 | 318,934,530 | net6k4 | Not started | Audited | Not started |
| 6 | 849,868,621 | net7rl012 | Not started | Not started | Deleted (was 618 of 34,053 shards) |

| Status | Meaning |
|:--|:--|
| Not started | No run has been asked for |
| In progress | A run is going, or has stopped with shards missing. `Train` refuses it |
| Complete | Every source shard has a relabelled shard. `Train` accepts it |
| Audited | Complete, and the checks in [Auditing a label set](#auditing-a-label-set) are recorded |
| Deleted | Its objects were deleted. The bucket is versioned, so their versions are kept; the id is free for a new start |

### Which labels each net trained on

Each cell is the label set a net used for that generation, and `—` means the
generation wasn't used. Confirmed from each net's `training.json`: its
`datasets` prefixes, and its `labels` map for nets trained after #190.

| Net | Gen 0 | Gen 1 | Gen 2 | Gen 3 | Gen 4 | Gen 5 | Gen 6 | Prepared set |
|:--|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:--|
| `net1-l050-r41022503872` | `O` | — | — | — | — | — | — | `g0·O` |
| `net2-l050-r41110004652` | `O` | `O` | — | — | — | — | — | `g0-1·O` |
| `net2x80-l050-r41131366975` | `O` | `O` | — | — | — | — | — | `g0-1·O` |
| `net2x80-l025-r41141703294` | `O` | `O` | — | — | — | — | — | `g0-1·O` |
| `net2x80-l075-r41145466004` | `O` | `O` | — | — | — | — | — | `g0-1·O` |
| `net3-l050-r41160495666` | `O` | `O` | `O` | — | — | — | — | `g0-2·O` |
| `net4-l050-r41165115607` | `O` | `O` | `O` | — | — | — | — | `g0-2·O` |
| `net4w512-l050-r41170687764` | `O` | `O` | `O` | — | — | — | — | `g0-2·O` |
| `net5-l050-r41229287546` | `O` | `O` | `O` | `O` | — | — | — | `g0-3·O` |
| `net5x120-l050-r41237487627` | `O` | `O` | `O` | `O` | — | — | — | `g0-3·O` |
| `net5b8-l050-r41265636815` | `O` | `O` | `O` | `O` | — | — | — | `g0-3·O` |
| `net6-l050-r41273387683` | `O` | `O` | `O` | `O` | `O` | — | — | `g0-4·O` |
| `net6k4-l050-r41282883885` | `O` | `O` | `O` | `O` | `O` | — | — | `g0-4·O` |
| `net7-l050-r41363264869` | `O` | `O` | `O` | `O` | `O` | `O` | — | `g0-5·O` |
| `net7rl012-l050-r42054816508` | `N7` | `N7` | `N7` | `O` | `O` | `O` | — | `g0-5·N7(0-2)` |
| `net7rl012345-l050-r42398995769` | `N7` | `N7` | `N7` | `N7r` | `N7r` | `N7r` | — | `g0-5·N7(0-2)·N7r(3-5)` |
| `net7rl012w768-l050-r42411319602` | `N7` | `N7` | `N7` | `O` | `O` | `O` | — | `g0-5·N7(0-2)` |
| `net8-l050-r42564788061` | `N7` | `N7` | `N7` | `O` | `O` | `O` | `O` | `g0-6·N7(0-2)` |
| `net8x160-l050-r43390801430` | `N7` | `N7` | `N7` | `O` | `O` | `O` | `O` | `g0-6·N7(0-2)` |

### Prepared training sets

`prepare` turns a choice of generations and labels into one `train.data` and one
`val.data`. A record holds its score, so their SHA-256s fingerprint both the
positions and the labels: **two nets with the same `val.data` SHA-256 were
scored on identical data.** All used split seed 1, shuffle seed 1 and
`val_fraction` 0.05.

| Prepared set | Training / held-out records | Workers | Shards | `train.data` | `val.data` |
|:--|--:|--:|--:|:--|:--|
| `g0·O` | 214,568,489 / 16,671,855 | 297 / 23 | 9,329 | `9c438ce0…808f` | `342adf6d…28d7` |
| `g0-1·O` | 561,653,589 / 37,941,221 | 537 / 39 | 24,045 | `4819161f…df65` | `7f9e5c0e…91a6` |
| `g0-2·O` | 857,263,237 / 49,369,383 | 722 / 46 | 36,398 | `0774b021…f2a7` | `93379839…a71d` |
| `g0-3·O` | 1,465,509,939 / 69,335,935 | 908 / 52 | 61,556 | `4d3e3337…c07c` | `e2e88636…0ab1` |
| `g0-4·O` | 1,932,314,071 / 97,679,562 | 1,152 / 64 | 81,394 | `cc0c5054…53ef` | `07b94289…5c6e` |
| `g0-5·O` | 2,238,186,465 / 110,741,698 | 1,336 / 72 | 94,149 | `f551e37e…d5f2` | `4dc5921e…b99e` |
| `g0-5·N7(0-2)` | 2,238,186,465 / 110,741,698 | 1,336 / 72 | 94,149 | `9df1226c…07bd` | `ad1f1aa4…f8ef` |
| `g0-5·N7(0-2)·N7r(3-5)` | 2,238,186,465 / 110,741,698 | 1,336 / 72 | 94,149 | `267501c5…182f` | `556b16d9…a14a` |
| `g0-6·N7(0-2)` | 3,034,526,184 / 164,270,600 | 1,636 / 92 | 128,202 | `e8f78490…a7df` | `bf6d6dff…3488` |

- **Naming:** `g<generations>·<label set>`. A mix names the relabelled
  generations in brackets: `g0-5·N7(0-2)` is generations 0–5, with `N7` on
  generations 0–2 and `O` on the rest.
- **A relabelled set must keep the same positions.** Its records, workers,
  shards and held-out workers must match the same generations under `O`, and
  its SHA-256s must differ, because the scores do.

### `net7-l050-r41363264869-n5000`

**Status: generations 0–2 audited.** net7's scores at 5,000 nodes, the same budget that made
the originals. Design and method: [relabel.md](relabel.md).

| Run | Date | Commit | Generations | Instances × minutes | Market | Shards | Records | Records/s an instance | Status and notes |
|:--|:--|:--|:--|:--|:--|--:|--:|--:|:--|
| [41533675760](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41533675760) | 2026-11-02 | `b7f12f7` | 0 | 1 × 10 | On demand, $3.4486/h, after 5 minutes without spot capacity | 0 | 0 | — | **Failed.** Smoke test: `relabel` panicked on the first worker, about 2 minutes after launch, on manifests that predate checksums (exit 134, fixed in #187). The log shows 0 shards and 0 records before the failure, so nothing should have been uploaded; S3 wasn't listed. Under $0.20, an estimate. Fixed binary run locally on 6 generation 0 shards (#187); no cloud run since |
| [41788545675](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41788545675) | 2026-11-04 | `cb9e55b` (`v0.44.3`) | 0, 1, 2 | 3 × 300 asked; t1 ran about 40, t0 about 225, t2 about 288 | Spot only, $1.0582/h at launch in us-east-1d | 23,414 | 582,722,671 | t0 17,731, t1 18,080, t2 17,462 | **Partial: 12,984 shards (323,909,949 records) left.** The first run with batches (#196). **t1 was reclaimed by spot at 07:34:19 UTC and t0 at 10:39:53 UTC**; each lost only its batch in progress (307 and 262 shards). t2 finished its 256 workers. The run's `failure` is those two reclaims. About $10, an estimate from instance-hours at the launch price |
| [41884537437](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41884537437) | 2026-11-04 | `cb9e55b` (`v0.44.3`) | 0, 1, 2 | 2 × 220 asked; none ran | Spot only | 0 | 0 | — | **Failed, no instance launched.** Dispatched from tag `v0.44.3`, but the `ci-task` role's OIDC trust allows only `refs/heads/main` and pull requests, so `sts:AssumeRoleWithWebIdentity` was refused. Both jobs stopped in about 35 seconds. No instance cost. A listing at 18:13 UTC still showed 23,414 relabelled manifests |
| [41885152500](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41885152500) | 2026-11-04 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; none ran | Spot only | 0 | 0 | — | **Failed, no instance launched.** Top-up of 41788545675, with 2 instances rather than 3 so the work left splits evenly. Both tasks got `InsufficientInstanceCapacity` for `c8a.16xlarge` in all five zones (`us-east-1a`, `b`, `c`, `d`, `f`) from 18:15 to about 18:45 UTC, then exited 3. No instance cost. A listing afterwards still showed 23,414 relabelled manifests |
| [41933996045](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41933996045) | 2026-11-04 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; t0 ran about 24, t1 about 62 | Spot only, both in us-east-1d | 3,467 | 86,721,621 | t0 16,194, t1 17,019 | **Partial: 9,517 shards (237,188,328 records) left.** Second top-up of 41788545675, launched at 23:29 UTC. **t0 was reclaimed by spot at 23:54:45 UTC and t1 at 00:32:18 UTC** on the 5th. About $1.50, an estimate from instance-hours at about $1.03/h |
| [41942564123](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41942564123) | 2026-11-05 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; t0 ran about 18, t1 about 27 | Spot only, t0 in us-east-1d, t1 in us-east-1c | 1,789 | 44,738,538 | t0 15,915, t1 17,275 | **Partial: 7,728 shards (192,449,790 records) left.** Third top-up of 41788545675, launched at 00:40 UTC. **t0 was reclaimed by spot at 00:58:40 UTC and t1 at 01:07:36 UTC.** About $0.80, an estimate from instance-hours at about $1.03/h |
| [41947343364](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41947343364) | 2026-11-05 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; t0 ran about 14, t1 about 11 | Spot only, both in us-east-1c | 1,007 | 25,183,123 | t0 17,005, t1 15,699 | **Partial: 6,721 shards (167,266,667 records) left.** Fourth top-up of 41788545675, launched at 01:17 UTC. **t1 was reclaimed by spot at 01:29:01 UTC and t0 at 01:32:11 UTC.** About $0.45, an estimate from instance-hours at about $1.03/h |
| [41961755175](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41961755175) | 2026-11-05 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; t0 and t1 ran about 20 | Spot only, both in us-east-1d | 1,647 | 41,074,827 | t0 17,877, t1 16,212 | **Partial: 5,074 shards (126,191,840 records) left.** Fifth top-up of 41788545675, launched at 03:29 UTC. **t1 was reclaimed by spot at 03:49:41 UTC and t0 at 03:49:42 UTC.** About $0.70, an estimate from instance-hours at about $1.03/h |
| [41967316609](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41967316609) | 2026-11-05 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; t0 ran about 30, t1 about 18 | Spot only, both in us-east-1c | 1,855 | 46,133,744 | t0 16,717, t1 15,456 | **Partial: 3,219 shards (80,058,096 records) left.** Sixth top-up of 41788545675, launched at 04:22 UTC. **t1 was reclaimed by spot at 04:40:38 UTC and t0 at 04:52:43 UTC.** About $0.80, an estimate from instance-hours at about $1.03/h |
| [41971756087](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41971756087) | 2026-11-05 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; t0 ran about 3, t1 under 1 | Spot only, both in us-east-1c | 65 | 1,608,708 | t0 8,292 | **Partial: 3,154 shards (78,449,388 records) left.** Seventh top-up of 41788545675, launched at about 05:06 UTC. **t1 got its spot notice at 05:06:50 UTC, about 50 seconds after launch, and t0 at 05:10:03 UTC, about 4 minutes after.** t1 uploaded no shards and wrote no `task.json`. Under $0.25, an estimate |
| [41976502122](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41976502122) | 2026-11-05 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 3 × 220 asked; t0 and t1 ran about 28, t2 under 1 | Spot only, all in us-east-1b | 2,227 | 55,407,265 | t0 16,602, t1 16,605 | **Partial: 927 shards (23,042,123 records) left.** Eighth top-up of 41788545675, launched at about 05:50 UTC. 3 instances, to split the work left evenly between t0 and t1; t2 had no workers and finished normally. **t0 was reclaimed by spot at 06:19:04 UTC and t1 at 06:19:07 UTC.** About $0.90, an estimate from instance-hours |
| [41980717684](https://github.com/jwallace145/rusty-chess-v2/actions/runs/41980717684) | 2026-11-05 | `cb9e55b` (`v0.44.3`, from `main`) | 0, 1, 2 | 2 × 220 asked; t0 ran about 12, t1 about 13 | Spot only, both in us-east-1d | 927 | 23,042,123 | t0 15,731, t1 14,894 | **Complete, no reclaims: 0 shards left.** Ninth top-up of 41788545675, launched at about 06:30 UTC. About $0.40, an estimate from instance-hours |

Before the top-up, generation 0 needed t1's last 1,400 shards, generation 1 t1's 4,908, and
generation 2 t0's 2,554 and t1's 4,122.

The nine runs that launched instances made 36,398 shards and 906,632,620
records. 20 instances launched and 16 were reclaimed by spot, across about 14.7
task-hours and, as an estimate, about $15.

| Gen | Source shards | Source records | Relabelled shards | Relabelled records | Changed | Mean \|change\| (cp) | Mates clamped | Sign agreement | Correlation | Audit |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 0 | 9,329 | 231,240,344 | 9,329 | 231,240,344 | 99.19% | 492.6 | 794,510 | 92.98% | 0.9519 | Passed, [2026-11-05](#audit-log) |
| 1 | 14,716 | 368,354,466 | 14,716 | 368,354,466 | 98.55% | 178.9 | 741,889 | 95.12% | 0.9594 | Passed, [2026-11-05](#audit-log) |
| 2 | 12,353 | 307,037,810 | 12,353 | 307,037,810 | 98.34% | 120.9 | 125,044 | 95.68% | 0.9700 | Passed, [2026-11-05](#audit-log) |

Records, changes and mates are summed from the manifests. Sign agreement and
correlation are over every record, and sign agreement leaves out records where
either score is zero.

**Reading these:** the same net re-scoring its own generation at the same budget
changes a label by a median of 17 cp and a mean of 30 cp, with 97.7% sign
agreement and correlation 0.990 ([relabel.md](relabel.md)). That's the noise
floor, and only disagreement beyond it is net7's opinion. It was measured on 16
shards of one generation, so it's a guide, not a bar.

### `net7rl012-l050-r42054816508-n5000`

**Status: generations 3–5 audited.** net7rl012's scores at 5,000 nodes. It was
the default net at `v0.45.0`, and these were the positions `N7` didn't cover
yet. The first label set made by the relabel manager (#204): one `slots`
dispatch per slot, 3 slots, every run from `main` at `7cb6ea2` (`v0.45.0`),
280 minutes each, spot only. The manager ran with
`--max-instance-hours 40` and its default 40-launch cap.

| Run | Date | Commit | Slot | Market | Shards | Records | Records/s | Status and notes |
|:--|:--|:--|--:|:--|--:|--:|--:|:--|
| [42340392266](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42340392266) | 2026-11-07 | `7cb6ea2` | 0 | Spot, $0.8119/h at launch in us-east-1b | 3,920 | 97,900,052 | 18,113 | **Reclaimed by spot at 06:24:41 UTC** after 90 minutes |
| [42340394396](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42340394396) | 2026-11-07 | `7cb6ea2` | 1 | Spot, $0.8119/h in us-east-1b | 3,972 | 99,228,427 | 18,382 | **Reclaimed by spot at 06:24:40 UTC** |
| [42340396613](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42340396613) | 2026-11-07 | `7cb6ea2` | 2 | Spot, $0.8119/h in us-east-1b | 3,865 | 96,566,190 | 17,879 | **Reclaimed by spot at 06:24:42 UTC** |
| [42347189266](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42347189266) | 2026-11-07 | `7cb6ea2` | 0 | Spot, $0.9695/h in us-east-1d | 12,203 | 304,715,302 | 17,777 | Relaunched by the manager 4 minutes after the reclaim. Used its 280 minutes and stopped before worker 166 of 214 |
| [42347191513](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42347191513) | 2026-11-07 | `7cb6ea2` | 1 | Spot, $0.9695/h in us-east-1d | 11,815 | 294,975,970 | 17,389 | Used its 280 minutes, stopped before worker 163 of 213 |
| [42347194014](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42347194014) | 2026-11-07 | `7cb6ea2` | 2 | Spot, $0.9695/h in us-east-1d | 12,193 | 304,468,914 | 17,724 | Used its 280 minutes, stopped before worker 165 of 213 |
| [42388140967](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42388140967) | 2026-11-07 | `7cb6ea2` | 0 | Spot, $0.8246/h in us-east-1b | 3,217 | 80,332,495 | 17,118 | **Complete.** Dispatched at 15:48 UTC by a restarted manager (below) |
| [42388143266](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42388143266) | 2026-11-07 | `7cb6ea2` | 1 | Spot, $0.8246/h in us-east-1b | 3,356 | 83,881,403 | 17,548 | **Complete** |
| [42388145645](https://github.com/jwallace145/rusty-chess-v2/actions/runs/42388145645) | 2026-11-07 | `7cb6ea2` | 2 | Spot, $0.8246/h in us-east-1b | 3,210 | 80,226,790 | 17,667 | **Complete** |

Shards, records and seconds are from each run's `task.json`. Records/s is for one
`c8a.16xlarge`. Prices are the spot price each job's log printed at launch.

- **The gap:** the laptop running the manager slept during the second round. The
  manager recorded slot 0's end at 12:22 UTC, about an hour late, and then
  stopped before recording slots 1 and 2 or dispatching the third round. Restarted
  at 15:47 UTC, it read its log, recorded both and dispatched all three slots.
  About 4.5 hours passed with no instance running, which cost time but nothing
  else. Its log is `~/.rusty-chess/relabel-manager/net7rl012-l050-r42054816508-n5000-g345-of3/log.jsonl`
  on that laptop, not in the repository.
- **Totals:** 9 runs, 9 instances, 3 reclaimed by spot, all at once. 57,751
  shards and 1,442,295,543 records. The manager counted 22.86 instance-hours of
  job time. At the launch prices that's about $21, an estimate; actual spot
  billing wasn't read.
- **Not verified:** the manager's `--notify` never fired, since nothing paused.

| Gen | Source shards | Source records | Relabelled shards | Relabelled records | Changed | Mean \|change\| (cp) | Mates clamped | Sign agreement | Correlation | Audit |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 3 | 25,158 | 628,213,254 | 25,158 | 628,213,254 | 98.13% | 112.4 | 301,181 | 96.10% | 0.9745 | Passed, [2026-11-07](#audit-log) |
| 4 | 19,838 | 495,147,759 | 19,838 | 495,147,759 | 97.92% | 81.7 | 198,373 | 96.16% | 0.9770 | Passed, [2026-11-07](#audit-log) |
| 5 | 12,755 | 318,934,530 | 12,755 | 318,934,530 | 97.67% | 61.6 | 16,726 | 96.53% | 0.9817 | Passed, [2026-11-07](#audit-log) |

All columns are from `ci/label-set-audit.sh`'s `summary.json`, over every record.

**Reading these:** they're closer to the originals than `N7` was on generations
0–2 (sign agreement 92.98–95.68%, correlation 0.9519–0.9700). That's expected:
these generations' labellers, net4w512 to net6k4, are nearer net7rl012 in
strength than PeSTO to net2x80 were to net7. Each newer generation moves less,
from 112 cp to 62 cp. Everything still sits above the same-net noise floor (97.7%
and 0.990).

### `net8x160-l050-r43390801430-n5000`

**Status: deleted 2026-11-18** (it had generation 6's first 618 of 34,053
shards; see the bullet at the end of this section). net8x160's scores
at 5,000 nodes. No campaign asked for this label set: it is what the first
`Relabel` run from this repository made, a proving run to check that the
workflow works here after the cutover. The workflow always uses the commit's
default net, which at `afcfc85` is the pinned `v0.54.0`'s net8x160 (sha256
`8ea67605…556c`), so the run started a new label set rather than adding to an
existing one. Design and method: [relabel.md](relabel.md).

| Run | Date | Commit | Slot | Market | Shards | Records | Records/s | Status and notes |
|:--|:--|:--|--:|:--|--:|--:|--:|:--|
| [44254774637](https://github.com/Braveheart-Chess/braveheart-selfplay/actions/runs/44254774637) | 2026-11-18 | `afcfc85` (engine `v0.54.0`, `3fd0223`) | 0 of 1 | Spot, $0.7296/h at launch in us-east-1b, after a first pass that found no spot capacity in any of the five zones | 618 | 15,412,763 | 18,327 | **Out of time, resumable.** Dispatched from `main` at 06:54 UTC with `generations=6 instances=1 slots=0 minutes=10`, 5,000 nodes, and the note "proving run after the split's cutover (2026-11-18): first relabel from this repository". Relabelled the first 6 of generation 6's 320 workers, each whole (`w000`–`w005` of run `20261108-42524365295-1`, task t0), in two batches of 305 and 313 shards, then stopped before the seventh at 841 s: `--minutes` is checked between batches. About $0.18, an estimate from the instance's 15 minutes at the launch price |

Shards, records and seconds are from the run's `task.json`. Records/s is for one
`c8a.16xlarge`, over the task's 841 s, which include the 29 s before the first
batch. The price is the spot price the job's log printed at launch.

| Gen | Source shards | Source records | Relabelled shards | Relabelled records | Changed | Mean \|change\| (cp) | Mates clamped | Sign agreement | Correlation | Audit |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 6 | 34,053 | 849,868,621 | 618 | 15,412,763 | 97.48% | 52.5 | 861 | not measured | not measured | Not audited: in progress |

Records, changes and mates are summed from the 618 manifests, which equal
`task.json`. Sign agreement and correlation need the positions files, which
weren't read.

- **Not a sample of the generation:** 1.8% of its shards, all from one task of
  its first run. On those, the mean change is 52.5 cp, against `N7r`'s 61.6 cp on
  generation 5, and still above the same-net noise floor.
- **One commit a label set:** these shards were made at `afcfc85`. The relabel
  manager refuses to add to a label set made at another commit
  ([relabel.md](relabel.md#the-relabel-manager)), the `ci-task` role trusts only
  `main` and pull requests, and the id names the net and the nodes but not the
  commit. So once `main` moves off `afcfc85`, finishing generation 6 under this
  id needs a decision first. `Train` refuses the label set while it is in
  progress.
- **Deleted, 2026-11-18:** `main` moved off `afcfc85` the same morning (#18),
  so this part could never be finished under this id, and it would block every
  later net8x160 relabel at 5,000 nodes. The owner had its 1,236 objects deleted
  ([audit log](#audit-log)). The run above stays on the record.

### Auditing a label set

Before marking a generation's label set audited, check and record each of these,
how it was checked, and anything that couldn't be:

1. **Inventory:** every source shard has a relabelled positions file and
   manifest, and there are no extras.
2. **Records:** each shard's count matches its source, and its FENs and results
   are identical and in order.
3. **Provenance:** every manifest names the label set, the net's SHA-256, the
   nodes and the commit, and its checksums match its files.
4. **Plausibility:** sign agreement and correlation with the originals, against
   the noise floor above. A wrong point of view couldn't produce high values.
5. **Determinism:** a sample of shards re-run gives byte-identical positions.
6. **Originals:** the `relabel` profile can't write to the source prefix. Record
   that its shard count and a sample of its checksums still match this ledger.

## Generation 0: PeSTO

**Settings** (identical in every manifest): 5,000 nodes per move; 8 random
plies, plus one more at random (the rule at the time, so half the openings
gave white an extra move; generation 1 uses 8 or 10). Openings were replayed
beyond ±400 cp. Win adjudication at 2,000 cp held for 4 plies; draw
adjudication at ≤ 10 cp for 12 plies after ply 80; at most 400 plies;
25,000 positions a shard.

| Run | Date | Engine | Instances | Positions | Games | Notes |
|:--|:--|:--|--:|--:|--:|:--|
| `20261030-41004581936-1` | 2026-10-30 | `v0.24.0` `af669ec` | 1 | 8,197,507 | 84,496 | Smoke test |
| `20261030-41006956708-1` | 2026-10-30 | `v0.25.0` `3e3bdbd` | 1 | 8,461,614 | 87,384 | Second smoke test, after the uploader throughput fix (#110) |
| `20261030-41007876197-1` | 2026-10-30 | `v0.25.0` `3e3bdbd` | 3 (spot) | 214,581,223 | 2,216,081 | t0 49.4M, t1 81.6M, t2 83.5M. t0 was interrupted by spot at 05:51 and lost about 350k positions in progress (the process-group bug fixed in #113). Stopped at about 200M on purpose |

**Audit** (#113, the full pass over every record): 0 P0 failures; bulletformat
parsed all 231M records.

**Quality:**

| Metric | Value |
|:--|:--|
| Results (W / D / B) | 32.2% / 34.5% / 33.3% |
| Duplicate positions | 2.2% |
| \|score\| ≤ 100 | 39.0% |
| Endgame share | 46% |
| Games ended by win adjudication | 64% |

**Findings that shaped generation 1** (details in the runbook):
- At 2,000 cp every adjudicated game was already lost, while 1,200 cp is
  right in 99.98% of cases.
- Games that ended in repetition were level at depth about 95% of the
  time, so the draw label agrees with the score.
- Odd random plies favoured white.

## Generation 1: net 1

**Settings** (identical in every manifest): 5,000 nodes per move; 8 or 10
random plies (always even); openings replayed beyond ±250 cp; win
adjudication at 1,200 cp for 4 plies; the rest as generation 0. Every run
was built with net 1 (`net1-l050-r41022503872`) as the default.

| Run | Date | Engine | Instances × minutes | Market | Positions | Games | Notes |
|:--|:--|:--|:--|:--|--:|--:|:--|
| `20261030-41084379912-1` | 2026-10-30 | `v0.29.0` `708adbd` | 1 × 5 | not recorded | 7,488,073 | 75,190 | Smoke test: about 25,000 positions/s an instance, 11% below PeSTO |
| `20261030-41086960808-1` | 2026-10-30 | `v0.29.0` `708adbd` | 3 × 100 asked; 2 ran about 85 | on demand | 276,116,559 | 2,775,313 | t0 139.5M, t1 136.6M. t2 never launched: no `c8a.16xlarge` capacity, spot or on demand, in any us-east-1 zone. Spot capacity was also gone for t0 and t1, which fell back to on demand. Cancelled early to limit cost |
| `20261030-41091882414-1` | 2026-10-30 | `v0.29.0` `fe9e9f2` | 1 × 100 asked; ran about 55 | on demand | 84,749,834 | 852,512 | Top-up replacing t2. `fe9e9f2` differs from `708adbd` only in the audit's colour-gap bar (#127). Cancelled with the main run |

**Audit:** the full pass (75 batches, about 5 minutes on 16 cores) gave
**GO**:
- 0 P0 failures over all 368,354,466 records;
- bulletformat exact: 32 bytes a record;
- every shard complete, with no orphans after the cancel;
- 256 workers, 256 distinct seeds.

Label spot checks (2,000 fresh re-searches a run): correlation 0.99, and
the same sign 99.5–99.6% of the time.

**Quality:**

| Metric | Value |
|:--|:--|
| Results (W / D / B) | 31.8% / 44.7% / 23.5% |
| Duplicates within a batch | 3.1% |
| \|score\| ≤ 100 | 32.0% (smoke sample) |
| Endgame share | 48.7% (smoke sample) |
| Games ended by win adjudication | 55% |

**Compared with generation 0:**
- **More draws and a white edge.** The colour gap is 8.3 points (the
  bar was widened to 10 in #127): a stronger player shows white's
  first-move advantage, and draws rose from 34.5% to 44.7%.
- **Fewer one-sided games.** Win adjudication fell from 64% to 55%.

**Cost:** about $15–20. It ran on demand rather than spot, because spot
capacity was gone. This is an estimate from the instance-hours, not a bill.

## Generation 2: net2x80

**Settings** (identical in every manifest): the same as generation 1. That's
5,000 nodes a move, 8 or 10 random plies, openings replayed beyond ±250 cp,
and win adjudication at 1,200 cp for 4 plies. Played by
`net2x80-l050-r41131366975` (net 2 trained for 80 superbatches; λ 0.5),
datagen built for the instance's own CPU (`native`, #136).

| Run | Date | Engine | Instances × minutes | Market | Positions | Games | Notes |
|:--|:--|:--|:--|:--|--:|--:|:--|
| `20261031-41153094884-1` | 2026-10-31 | `v0.31.1` `75017a5` | 3 × 70 | spot | 307,037,810 | 3,265,807 | t0 98.1M, t1 104.2M, t2 104.7M. No interruptions. The first run with the native build, about 25,150 positions/s an instance |

**The two benchmark runs for #136** (`upload: false`) measured the native
build against `x86-64-v3` before this generation, with the same seed (7), net
and settings, at `v0.31.0` (`bf27704`), on spot. Neither is part of the
generation:

| Run | Target | Minutes | Positions | Games | Shards | Positions/s at 150 s |
|:--|:--|:--|--:|--:|--:|--:|
| `20261030-41144384553-1` | `native` | 5 asked; **reclaimed by spot** at 23:29:26 UTC after 188 s | 4,062,796 | 43,130 | 192 | 25,235 |
| `20261030-41144392189-1` | `x86-64-v3` | 5 | 6,920,847 | 73,583 | 320 | 23,037 |

- **The native build is about 9.5% faster.**
- **The games are the same, as far as the interrupted run can show.** All 128
  shards the native run finished normally are byte-identical to the other run's
  (3,207,361 positions, 33,979 games), by the SHA-256s in their manifests.
- **The other 64 can't be compared.** They're the shards the interruption
  flushed early (10,947–14,896 positions, against 25,001–25,265), and the
  artifacts hold only checksums, so a prefix match can't be checked.
- **Nothing was uploaded:** both `task.json` files say `"uploaded": false`. The
  native run's log line "192 complete shards are in S3" is the interrupted
  path's fixed wording. S3 itself wasn't listed.

**Full audit** (`ci/datagen-full-audit.sh`, 7 minutes on 16 cores): **GO**.
1. **Inventory:**
   - every shard complete, with no orphans;
   - shard numbers gap-free for all 192 workers;
   - no duplicate shards, and all seeds unique;
   - one settings block, one engine, one generation;
   - each task's `task.json` totals equal its manifests.
2. **Every record:** 0 failures over all 307,037,810 records, and bulletformat
   exact at 32 bytes a record.
3. **No P1 warnings** in any of the 63 batches.
4. **Determinism: 2,304 of 2,304 games regenerated identically** from their
   seeds (12 per worker, all 192 workers).
   - This build plays generation 2 with net2x80 (sha256 `731b30d2…677e`).
   - **Negative controls:** the same seeds with net 1, or at 2,500 nodes,
     regenerated 0 of 48 games identically.
   - So **net2x80 at 5,000 nodes played the whole generation**, even though
     these manifests predate the recorded `provenance.net`.

| Quality | Generation 1 | Generation 2 |
|:--|--:|--:|
| Results (W / D / B) | 31.8 / 44.7 / 23.5% | 32.4 / 43.0 / 24.7% |
| White–black gap | 8.3 points | 7.7 points |
| Games ended by win adjudication | 55% | 56.6% |
| Duplicates within a batch (200 shards) | 3.1% | 1.48% |
| Median game length (engine plies) | 114 | 112 |
| Records a game | 99.5 | 94.0 |
| \|score\| ≤ 100 | 32.0% (smoke sample) | 31.0% |
| Endgame share | 48.7% (smoke sample) | 46.0% |

**Compared with generation 1:**
- slightly more decisive (draws −1.7 points);
- half the within-batch repetition;
- a little shorter.
- **No sign of the self-play collapsing toward fewer positions,** but these
  are within-generation measures. Duplicate positions *across* generations and
  structural novelty aren't measured yet: that needs the cross-generation
  comparison mode the audit cross-reference recommends.

**Cost:** about $5 on spot (3 instances, about 85 minutes including build and
upload).

## Generation 3: net4w512

**Settings** (identical in every manifest): the same as generations 1 and 2.
Played by `net4w512-l050-r41170687764` (512 hidden neurons, on generations
0–2; λ 0.5), the default in `v0.36.0`, with the native build. The first
generation whose manifests record the net (`provenance.net`, sha256
`5d24472c…5b68`).

| Run | Date | Engine | Instances × minutes | Market | Positions | Games | Notes |
|:--|:--|:--|:--|:--|--:|--:|:--|
| `20261031-41180888933-1` | 2026-10-31 | `v0.36.0` `f7605e5` | 3 × 160 | spot | 628,213,254 | 6,562,925 | t0 204.4M, t1 208.2M, t2 215.6M. No interruptions. About 21,800 positions/s an instance, 13% below generation 2's net2x80: the 512-neuron net is slower per node |

**Full audit** (`ci/datagen-full-audit.sh` on 16 cores, after a 17-minute
download of the 10 GB generation): **GO**.
1. **Inventory:** 0 P0 problems over 25,158 shards. One settings block, one
   engine, one generation and one net throughout.
2. **Every record:** 0 failures over all 628,213,254 records, and
   bulletformat exact at 32 bytes a record.
3. **No P1 warnings** in any of the 127 batches.
4. **Determinism: 2,304 of 2,304 games regenerated identically** from their
   seeds (12 per worker, all 192 workers), with the tool built at
   `f7605e5`. The net named in the manifests is the net this build plays.

| Quality | Generation 1 | Generation 2 | Generation 3 |
|:--|--:|--:|--:|
| Results (W / D / B) | 31.8 / 44.7 / 23.5% | 32.4 / 43.0 / 24.7% | 30.3 / 46.4 / 23.3% |
| White–black gap | 8.3 points | 7.7 points | 7.0 points |
| Games ended by win adjudication | 55% | 56.6% | 53.2% |
| Duplicates within a batch (200 shards) | 3.1% | 1.48% | 1.76% |
| Median game length (engine plies) | 114 | 112 | 112 |
| Records a game | 99.5 | 94.0 | 95.7 |
| \|score\| ≤ 100 | 32.0% (smoke sample) | 31.0% | 31.8% |
| Endgame share | 48.7% (smoke sample) | 46.0% | 47.1% |

**Compared with generation 2:**
- **more drawn** (+3.4 points), the reverse of generation 1 → 2: the stronger
  net converts fewer games to a win adjudication;
- a slightly smaller colour gap;
- within-batch repetition about level (1.76% against 1.48%);
- the same game length.

**Cost:** about $9 on spot (3 × `c8a.16xlarge` at $1.149/h for about 2.7
hours each; estimated from the run's spot price, not billed figures).

## Generation 4: net 5

**Settings** (identical in every manifest): the same as generations 1–3.
Played by `net5-l050-r41229287546` (512 hidden, generations 0–3; λ 0.5), the
default in `v0.37.0`–`v0.37.1`, with the native build. Two engine commits,
the same engine: the top-up was built from `1a6c3d3`, which differs from
`v0.37.1` (`60a5252`) only in `docs/nets.md` and `docs/roadmap.md` (#160);
both report 0.37.1, bench 1101129 and net 5's sha256 `acd8e049…906b`.

| Run | Date | Engine | Instances × minutes | Market | Positions | Games | Notes |
|:--|:--|:--|:--|:--|--:|--:|:--|
| `20261101-41259904529-1` | 2026-11-01 | `v0.37.1` `60a5252` | 3 × 135 asked; t1 ran 16 | spot | 374,338,166 | 3,970,953 | t0 177.8M, t2 177.5M, about 21,950 positions/s an instance. **t1 was reclaimed by spot at 01:26 UTC, 16 minutes in:** its 777 complete shards (19.0M positions) were kept, only the games in progress lost. GitHub can't re-run a job while its run is going, so a top-up replaced it |
| `20261101-41263763971-1` | 2026-11-01 | `0.37.1` `1a6c3d3` | 1 × 125 asked; ran about 90 | spot | 120,809,593 | 1,281,238 | Top-up replacing t1. **Cancelled** at about 120M to limit cost once the generation passed the night's reduced target of about 300M; it ended at about 495M |

**Full audit** (`ci/datagen-full-audit.sh` on 16 cores, built at `1a6c3d3`;
a 13-minute download of 8.1 GB, then 12 minutes): **GO**.
1. **Inventory:** 0 P0 problems over 19,838 shards, including the
   interrupted task and the cancelled run: no gaps, orphans, duplicate
   shards or shared seeds; one settings block, one generation, one net.
2. **Every record:** 0 failures over all 495,147,759 records, and
   bulletformat exact at 32 bytes a record.
3. **No P1 warnings** in any of the 101 batches.
4. **Determinism: 3,072 of 3,072 games regenerated identically** from their
   seeds (12 per worker, all 256 workers of both runs).

| Quality | Generation 2 | Generation 3 | Generation 4 |
|:--|--:|--:|--:|
| Results (W / D / B) | 32.4 / 43.0 / 24.7% | 30.3 / 46.4 / 23.3% | 29.7 / 47.2 / 23.1% |
| White–black gap | 7.7 points | 7.0 points | 6.6 points |
| Games ended by win adjudication | 56.6% | 53.2% | 52.5% |
| Duplicates within a batch (200 shards) | 1.48% | 1.76% | 1.71% |
| Median game length (engine plies) | 112 | 112 | 109 |
| Records a game | 94.0 | 95.7 | 94.3 |
| \|score\| ≤ 100 | 31.0% | 31.8% | 32.4% |
| Endgame share | 46.0% | 47.1% | 46.0% |

**Compared with generation 3:**
- **draws up again,** +0.8 points to 47.2% (generation 2 → 3 was +3.4).
  Every generation so far has drawn more; the audit's bar is 55%, and as
  draws rise the game results carry less signal for training;
- the colour gap a little smaller, repetition level, games slightly shorter;
- no sign of collapse toward fewer distinct positions within a batch.

**Cost:** about $8 on spot, an estimate from instance-hours at generation
3's spot price (about $1.15/h), not billed figures.

## Generation 5: net6k4

**Settings** (identical in every manifest): the same as generations 1–4.
Played by `net6k4-l050-r41282883885` (512 hidden, 8 output buckets, **4 king
buckets**; generations 0–4), the default in `v0.42.0` (`a921f3f`), with the
native build. One engine commit throughout.

| Run | Date | Engine | Instances × minutes | Market | Positions | Games | Notes |
|:--|:--|:--|:--|:--|--:|--:|:--|
| `20261101-41352126811-1` | 2026-11-01 | `v0.42.0` `a921f3f` | 3 × 150 asked; t1 ran 84, t0 and t2 about 95 | spot | 318,934,530 | 3,409,848 | t0 111.0M, t1 98.5M, t2 109.5M. **t1 was reclaimed by spot at 22:52 UTC**, 84 minutes in (its 3,960 complete shards kept). The run was **cancelled** at 23:05 to limit cost, at about 317M of the 500M asked for, with no top-up. About **19,570 positions/s an instance**, 11% below generation 4's: net6k4 is about 11% slower per node |

**Full audit** (`ci/datagen-full-audit.sh` on 16 cores, built at
`a921f3f`): **GO**.
1. **Inventory:** 0 P0 problems over 12,755 shards, the interrupted task
   included: no gaps, orphans, duplicate shards or shared seeds; one settings
   block, one engine, one generation, one net.
2. **Every record:** 0 failures over all 318,934,530 records, and
   bulletformat exact at 32 bytes a record.
3. **No P1 warnings** in any of the 65 batches.
4. **Determinism: 2,304 of 2,304 games regenerated identically** from their
   seeds (12 per worker, all 192 workers), with net6k4 named in every
   manifest and played by the build.

| Quality | Generation 3 | Generation 4 | Generation 5 |
|:--|--:|--:|--:|
| Results (W / D / B) | 30.3 / 46.4 / 23.3% | 29.7 / 47.2 / 23.1% | 29.2 / 47.7 / 23.1% |
| White–black gap | 7.0 points | 6.6 points | 6.1 points |
| Games ended by win adjudication | 53.2% | 52.5% | 52.0% |
| Duplicates within a batch (200 shards) | 1.76% | 1.71% | **0.98%** |
| Median game length (engine plies) | 112 | 109 | 108 |
| Records a game | 95.7 | 94.3 | 93.5 |
| \|score\| ≤ 100 | 31.8% | 32.4% | 34.1% |
| Endgame share | 47.1% | 46.0% | 45.6% |

**Compared with generation 4:**
- **draws up again, by less:** +0.5 points to 47.7% (generations 3 → 4:
  +0.8; 2 → 3: +3.4), still well inside the 55% bar;
- **within-batch repetition nearly halved,** 1.71% → 0.98%: the king-bucketed
  net's games vary more;
- more level positions (|score| ≤ 100 up 1.7 points), a smaller colour gap,
  slightly shorter games.

**Cost:** about $5–6 on spot (about 4.7 instance-hours), an estimate, not
billed figures.

## Generation 6: net7rl012

**Settings** (identical in every manifest): the same as generations 1–5.
Played by `net7rl012-l050-r42054816508` (512 hidden, 8 output buckets, 4 king
buckets; generations 0–5, with `N7` labels on 0–2), the default since
`v0.45.0`, at `v0.47.1` (`863241e`), which also carries the pawn-structure
correction history (#221) in the search that played. The native build; one
engine commit throughout. **The first generation made by the datagen
manager** (#230): five one-instance runs at 280 minutes each, dispatched and
relaunched from a laptop toward a target of 600M positions.

| Run | Date | Engine | Instance | Market | Shards | Positions | Games | Notes |
|:--|:--|:--|:--|:--|--:|--:|--:|:--|
| `20261108-42524365295-1` | 2026-11-08 | `v0.47.1` `863241e` | 1 × 280 asked; ran about 143 | Spot, $0.8773/h at launch, us-east-1b | 6,620 | 165,074,404 | 1,875,873 | Dispatched 21:16 UTC. **Reclaimed by spot at 23:38:47 UTC**; its complete shards kept |
| `20261108-42524367808-1` | 2026-11-08 | `v0.47.1` `863241e` | 1 × 280; ran all 280 | Spot, $0.8773/h, us-east-1b | 12,564 | 314,061,753 | 3,570,912 | Dispatched 21:16. Finished at 02:01 UTC: 314,061,753 positions in 16,825 s, **18,666 positions/s an instance** (generation 5: 19,570) |
| `20261108-42524370121-1` | 2026-11-08 | `v0.47.1` `863241e` | 1 × 280; ran about 146 | Spot, $0.8773/h, us-east-1b | 6,558 | 163,478,110 | 1,858,591 | Dispatched 21:16. **Reclaimed at 23:42:33 UTC** |
| `20261108-42536885510-1` | 2026-11-08 | `v0.47.1` `863241e` | 1 × 280; ran about 15 | Spot, $0.8773/h, us-east-1b | 640 | 15,023,775 | 171,021 | Replacement dispatched 23:47 with 19,849 shards (about 482M) in S3. **Reclaimed at 00:02:11 UTC** |
| `20261109-42538201838-1` | 2026-11-09 | `v0.47.1` `863241e` | 1 × 280; ran about 169 | Spot, $0.8803/h, us-east-1b | 7,671 | 192,230,579 | 2,185,719 | Replacement dispatched 00:04 with 21,220 shards (about 516M) in S3. **Cancelled by the owner at 02:53 UTC** once the generation had passed its target; the instance was terminated at 02:53:53 and its complete shards kept |

Shards, positions and games are summed from the manifests; the finished run's
`task.json` (the only one, since interrupted and cancelled tasks don't upload
theirs) agrees with its manifests exactly. Seeds are the run ids (the
workflow's default), so no two runs share one.

**The campaign** (`ci/datagen-manager.py`, log
`~/.rusty-chess/datagen-manager/gen6-863241e/log.jsonl` on the owner's laptop):
5 dispatches, 5 launches, 3 spot reclaims. It reached 600M at about 01:00 UTC
and ended at 849.9M, 42% over, because the manager of #230 dispatched from
what was in S3 and let each instance run its full 280 minutes: the replacement
launched at 516M alone made 192M. The owner cancelled the last instance by
hand; #231 makes the manager size dispatches to the positions still needed
and cancel at the target. About **12.6 instance-hours, about $11 on spot**,
an estimate from the launch prices, not a billed figure; the manager's own log
counts 9.8 hours because it was quit before the cancelled run ended.

**Full audit** (`ci/datagen-full-audit.sh` on 16 cores, the tool built at
`6815bfe`, whose Rust is `863241e`'s; the determinism step builds the engine
at the data's own commit): **GO**.
1. **Inventory:** 0 P0 problems over 34,053 shards, the three interrupted
   tasks and the cancelled one included: no gaps, orphans, duplicate shards
   or shared seeds; one settings block, one engine (`0.47.1 863241e`), one
   generation, one net (`74af5b46…70ce`); the `task.json` present matches its
   manifests.
2. **Every record:** 0 failures over all 849,868,621 records (every record
   rebuilt from its replayed game, every termination confirmed by the final
   position, every FEN round-tripped, every checksum matched), and
   bulletformat exact at 32 bytes a record, 27,195,795,872 bytes.
3. **No P1 warnings** in any of the 173 batches.
4. **Determinism: 3,840 of 3,840 games regenerated identically** from their
   seeds (12 per worker, all 320 workers), with net7rl012 named in every
   manifest and played by the build.

| Quality | Generation 4 | Generation 5 | Generation 6 |
|:--|--:|--:|--:|
| Results (W / D / B) | 29.7 / 47.2 / 23.1% | 29.2 / 47.7 / 23.1% | 28.9 / 47.5 / 23.6% |
| White–black gap | 6.6 points | 6.1 points | 5.3 points |
| Games ended by win adjudication | 52.5% | 52.0% | 52.2% |
| Duplicates within a batch (200 shards) | 1.71% | 0.98% | 1.12% |
| Median game length (engine plies) | 109 | 108 | 102 |
| Records a game | 94.3 | 93.5 | 88.0 |
| \|score\| ≤ 100 | 32.4% | 34.1% | 33.7% |
| Endgame share | 46.0% | 45.6% | 43.4% |

Batch figures weighted by records (games for the median, which is the
games-weighted median of the batch medians).

**Compared with generation 5:**
- **draws level,** 47.7% → 47.5%, the first generation without a rise, well
  inside the 55% bar; the colour gap narrowed again, to 5.3 points;
- **shorter games:** median 108 → 102 plies and 93.5 → 88.0 records a game,
  with a smaller endgame share (45.6% → 43.4%): the stronger net decides
  games earlier;
- **within-batch repetition up a little,** 0.98% → 1.12%, still a fifth of the
  5% bar;
- **slower generation:** 18,666 positions/s an instance, 4.6% below
  generation 5's 19,570: the same architecture, with the correction history's
  cost per node (−2.0% ± 1.6% on the CI bench) and the shorter games' extra
  openings.

**Cost:** about $11 on spot (about 12.6 instance-hours), an estimate, not a
billed figure.

## Generation 7: net8x160

**Settings** (identical in every manifest so far): the same as generations
1–6. Played by `net8x160-l050-r43390801430` (sha256 `8ea67605…556c`), the
default net of `v0.54.0` (`3fd0223`), the engine this repository pins, with the
native build. **The first generation made by this repository:** every manifest
records its commit, `selfplay_sha` `afcfc85`, beside the engine's.

| Run | Date | Engine | Instance | Market | Shards | Positions | Games | Notes |
|:--|:--|:--|:--|:--|--:|--:|--:|:--|
| `20261118-44254770264-1` | 2026-11-18 | `v0.54.0` `3fd0223`, braveheart-selfplay `afcfc85` | 1 × 5; ran all 5 | Spot, $0.7296/h at launch, us-east-1b, after a first pass that found no spot capacity in any of the five zones | 256 | 5,905,655 | 66,603 | **Smoke test**, run [44254770264](https://github.com/Braveheart-Chess/braveheart-selfplay/actions/runs/44254770264), and the first Datagen from this repository, with the note "proving run after the split's cutover (2026-11-18): first datagen from this repository into the positions bucket". Dispatched from `main` at 06:54 UTC and finished normally at 07:02: 5,905,655 positions in 300 s of play, **19,685 positions/s an instance** (positions over the 5 minutes, as the job summary gives it; `task.json`'s 325 s also count 18 s of build and bench). One 5-minute run, so not the generation's rate |

Shards, positions and games are summed from the manifests and equal the run's
`task.json`. The seed is the run id (the workflow's default). The manifests show
one settings block, one engine (`0.54.0` `3fd0223`, bench 919,304), one
`selfplay_sha` and one net, and 64 workers with 64 distinct seeds; each worker's
fourth and last shard (15,139–19,106 positions) is the one flushed when the 5
minutes ended. Summed from the same manifests, as a smoke test's numbers and
not an audit's: results 28.6 / 48.6 / 22.9% (W / D / B), 51.2% of games ended by
win adjudication, 88.7 records a game.

- **Not run:** the runbook's sample check and the full audit. Nothing trains on
  generation 7 before its full audit says GO.
- **One commit a generation:** these shards were made at `afcfc85`, `main` when
  they ran. The datagen manager refuses to add to a generation whose shards were
  made at another commit ([datagen-runbook.md](datagen-runbook.md#the-datagen-manager)),
  and nothing here is deleted, so once `main` moves off `afcfc85`, the rest of
  generation 7 needs a decision first.
- **Closed, 2026-11-18:** `main` moved off `afcfc85` the same morning (#18), so
  generation 7 stays at this smoke test. The owner is holding the next campaign;
  it starts as generation 8, with a `datagen.toml` bump at the commit it runs from.

**Cost:** about $0.08 on spot (the instance ran 6 minutes 12 seconds at
$0.7296/h), an estimate, not a billed figure.

## The bucket move

On 2026-11-19 the positions were copied once more, key for key, from the
management account's `s3://jwall-engineering-braveheart-selfplay-positions` into
the production account that jaws is moving every project to:
`s3://jaws-braveheart-selfplay-positions`, versioned like the original. **The
workflows moved the same day** (the cutover): Datagen and Relabel run in the
production account from then on, and write only to the production bucket.
Paths this page recorded before the cutover name the management account's
bucket; the keys are the same in both.

Nothing was written to either bucket between the copy and the cutover, so the
final `sync` the plan called for had nothing to copy and was not run. A
re-audit just before the cutover found both buckets identical, key, size and
ETag ([audit log](#audit-log), the second 2026-11-19 entry).

braveheart-trainer's GPU tasks stay in the management account until its G and VT
quota in production is approved. Its readers resolve to each account's own
roles, so until the trainer moves it reads the management account's bucket,
which holds every generation up to the cutover but nothing written after it.

Two Datagen smoke tests from `main` at `v0.5.0` (`5bb1cfb`) checked the
production task runner on 2026-11-19, each one `c8a.16xlarge` task of 5 minutes
with upload off:

| Run | Instance | Result |
|:--|:--|:--|
| `Datagen` [44546406893](https://github.com/Braveheart-Chess/braveheart-selfplay/actions/runs/44546406893), 14:41 UTC | none | Failed at launch: `AuthFailure.ServiceLinkedRoleCreationNotPermitted`. The production account had no EC2 Spot service-linked role, and the task launch role may not create one. The error is not a capacity error, so the on-demand fallback did not run |
| `Datagen` [44622087967](https://github.com/Braveheart-Chess/braveheart-selfplay/actions/runs/44622087967), 21:46 UTC | on-demand `c8a.16xlarge` `i-0854d4859569f4947`, us-east-1a | Passed: 65,382 games, 5,793,585 positions, 256 shards in 324 s (about 19,300 positions a second). Spot was refused for capacity (`InsufficientInstanceCapacity`) in all five zones for the profile's 5 minutes, 61 times, placement score 1, and the task fell back to on demand |

Between the two, jaws took the role into its foundation, which creates it in
every account that runs a task runner (jwallace145/jaws#248 and #250, released
as v0.65.0 and v0.66.0). The second run's refusals are capacity errors, so the
role is in place. **Not yet seen in production:** a spot instance actually
launching; that waits on spot capacity for `c8a.16xlarge`.

**Cost:** about $0.38, an estimate from the instance's 6 min 33 s between
launch and termination at $3.4486 an hour, not a billed figure. The failed run
launched nothing.

The owner's production admin role made the copy inside S3, with AWS CLI
2.34.59. It reads the management account's bucket through that bucket's policy,
which grants it read and list only. The copy was `aws s3 sync --only-show-errors` on each of 39
prefixes (every task of every run, and every run of every label set), 8 at a
time with 32 requests each, then once over `data/` for anything beside them. A
trial copy of generation 7 alone came first and was audited before the rest. The
full copy ran from 00:46:19 to 00:59:22 UTC and printed no error.

Only current versions were copied: the versions and delete markers of the
deleted `N8x` label set stay in the management account's bucket alone. That
bucket was not changed and is kept. The audit is in the [audit log](#audit-log), under 2026-11-19.

On 2026-11-16, when the Braveheart project split, the positions moved again,
key for key, from `s3://jwall-engineering-braveheart-nnue` to this project's
`s3://jwall-engineering-braveheart-selfplay-positions`, which is versioned; the
nets went to braveheart-trainer's `s3://jwall-engineering-braveheart-trainer-nets`
([nets.md](https://github.com/Braveheart-Chess/braveheart-trainer/blob/main/docs/nets.md#audit-log)).
A path this page records under either earlier bucket is the same key in the new
one. The owner ran the copy inside S3 with `aws s3 sync` and admin credentials,
one prefix to each bucket; the split's plan gives these commands:

```sh
aws s3 sync s3://jwall-engineering-braveheart-nnue/data/selfplay/ s3://jwall-engineering-braveheart-selfplay-positions/data/selfplay/ --only-show-errors
aws s3 sync s3://jwall-engineering-braveheart-nnue/data/nets/ s3://jwall-engineering-braveheart-trainer-nets/data/nets/ --only-show-errors
```

The copy's exact invocation, output and times were not recorded. The old bucket
was not changed and is kept until the owner deletes it. The audit of the copy
is in the [audit log](#audit-log), under 2026-11-16: every object is in the new
bucket with the same size and ETag.

The move before it: until 2026-11-15 this data was in `s3://jwall-engineering-rusty-chess-nnue`,
the bucket of the project's old name. It was copied, key for key, to
`s3://jwall-engineering-braveheart-nnue`: a path this page or
[nets.md](https://github.com/Braveheart-Chess/braveheart-trainer/blob/main/docs/nets.md) records under the old bucket is the same key in the new one.
Recorded runs keep the path that was true when they ran.

The copy ran inside S3, with AWS CLI 2.34.59 and admin credentials:

```sh
aws s3 sync s3://jwall-engineering-rusty-chess-nnue s3://jwall-engineering-braveheart-nnue --only-show-errors
```

It printed no error. Its start and end times were not recorded. The old bucket
was not changed and is kept until the owner deletes it. The audit of the copy
is in the [audit log](#audit-log), under 2026-11-15.

The workflows moved to the new bucket and the new remote-task project
(`braveheart`, from `rusty-chess`) with `v0.54.0` (#267). Two runs from `main`
at `v0.54.0` (`3fd0223`) then checked the task runner, both on 2026-11-15:

| Run | Instance | Result |
|:--|:--|:--|
| `GPU smoke` [43763493287](https://github.com/Braveheart-Chess/braveheart/actions/runs/43763493287) | on-demand `g6.xlarge` `i-042db66bcbf603c79`, us-east-1a, NVIDIA L4, driver 595.112.07, CUDA 13.2 | 8 checks passed, 0 failed: a CUDA kernel compiles and runs, the trainer builds with bullet's CUDA backend (32 s), and the dataset grants hold in both directions (list and read under `data/selfplay/` and write under `data/nets/` allowed; list of `data/` and write under `data/selfplay/` denied) |
| `Match` [43763495868](https://github.com/Braveheart-Chess/braveheart/actions/runs/43763495868), `v0.54.0` against `v0.53.0`, 10+0.1, 200 games | on-demand `c8a.16xlarge` `i-00e9990bf49ba3200`, us-east-1a | The task ran and reported: −17.4 Elo (99% interval −49.0 to +14.0), LOS 7.7%, 23 W / 33 L / 144 D, pentanomial 2, 24, 56, 18, 0 |

The match is a check that the runner plays and reports, not a strength
reading. The two releases differ in no search or eval code (`v0.54.0` adds the
names above and a docs change, and the bench signature was 919,304 before and
after #267), so the expected difference is zero and the interval includes it.
With 200 games it could not have shown a small difference either way.

The GPU smoke test wrote `data/nets/smoke-20261115T212604Z/hello.txt` (28
bytes), so the new bucket has one object the old one does not. Both instances
were terminated by the task runner.

**Cost:** at most about $0.04 and $0.27, estimates from the workflows' whole
run times (3 min 3 s at $0.8048 an hour, 4 min 43 s at $3.4486 an hour), not
billed figures; the instances ran for less than that.

**Not checked:** the `sprt`, `datagen`, `relabel` and `cpu` profiles, and a
`Train` run, under the new project. Their launch templates and grants were
created with the others; none has launched yet.

### Checking a copy

Two read-only tools check a copy. Both are run, and both results go in the
[audit log](#audit-log):

- **`ci/bucket-copy-audit.sh`** compares **every** object's key, size and ETag
  in the two listings. Where an ETag differs, it compares S3's full-object
  checksum instead. An ETag is the MD5 of the bytes only for an object written
  in one part to an SSE-S3 or unencrypted bucket. Every object here is
  written that way.
- **`ci/bucket-copy-sample.py`** does not trust S3's metadata. It downloads a
  sample from both buckets and compares SHA-256s. The sample is one random
  object from every directory (every task's positions, games and manifests in
  every run and label set) plus a random 0.5% on top, or every object with
  `--all`. It writes each sampled key's size and both hashes to `sample.json`.

```sh
ci/bucket-copy-audit.sh --prefix data/selfplay/ SOURCE DESTINATION <dir>
ci/bucket-copy-sample.py --prefix data/selfplay/ SOURCE DESTINATION <dir>
```

## Adding a run, a generation or a label set

After every Datagen or Relabel run, including smoke tests, benchmarks and
failed or cancelled runs:

1. Add a row to the generation's table, with the run id, date, engine
   version and commit, instances × minutes asked and run, market, positions
   and games (summed from the manifests), and what happened (interruptions,
   capacity, cancellations).
2. When the generation is complete, run `ci/datagen-full-audit.sh`
   ([datagen-runbook.md](datagen-runbook.md)) and record its verdict, the
   determinism result and the quality numbers, including how they moved from
   the previous generation.
3. Update the Generations table.
4. **For a Relabel run:** add a row to its label set's run table. Update the
   per-generation results and [Which label sets exist](#which-label-sets-exist),
   and run [the audit](#auditing-a-label-set).
5. **For a new label set:** give it a key and a row in
   [Label sets](#label-sets), a column in
   [Which label sets exist](#which-label-sets-exist) and its own section. Never
   reuse an id.
6. **For a new net:** add a row to
   [Which labels each net trained on](#which-labels-each-net-trained-on), from
   its `training.json`. Add its prepared set if it's new, or check that its
   SHA-256s match the existing one.
7. Take numbers from the manifests, `task.json` or `datagen-audit`, and label
   anything else, such as costs, as an estimate. The full rule is in
   [AGENTS.md](../AGENTS.md#experiments-and-the-research-record).

## Audit log

Checks of this ledger against its primary sources.

**2026-11-03,** at `v0.44.1`:

| Check | Result |
|:--|:--|
| Every `Datagen` run listed by GitHub (13) has an entry here | Two were missing, the #136 benchmark runs, now under generation 2 with the comparison #136 promised |
| Each generation's runs sum to its positions and games | Exact for all six |
| Shards of generations 0–5 against net7's `split.json` (94,149) and net6's (81,394 for generations 0–4) | Exact |
| Positions of each net's generations against its training + held-out records | Exact for nets 1, 2, 3, 5, 6 and 7 |
| Records a game in the quality tables, generations 1–5 | Match positions ÷ games |

**Not checked:** the shard manifests themselves. S3 needed AWS credentials this
audit didn't have, so the totals are checked against each other and against
`training.json`, not recounted. No audit verdict was re-run.

**2026-11-04,** at `v0.44.3`, adding the label tables:

| Check | Result |
|:--|:--|
| Which labels each of the 14 trained nets used, from each `training.json` | All 14 list only `data/selfplay/v1/` prefixes, and none has a `labels` map, so all used `O` |
| Prepared sets and their SHA-256s | 6 distinct sets. Nets given the same recipe's data share `train.data` and `val.data` SHA-256s, including the whole λ sweep |
| Held-out workers from each prepared set to the next | Each contains the previous set's, so "the same held-out workers" holds |
| Split settings | Split seed 1, shuffle seed 1 and `val_fraction` 0.05 for all 14 |
| Label set id the run will write | `relabel --net default --nodes 5000 --print-label-id` prints `{N7}` at this commit's net; the manifests will confirm it |

**Not checked:** the manifests and S3, still without AWS credentials. The `N7`
results are pending its run.

**2026-11-04,** of `N7` after run 41788545675, read-only with
`AWS_PROFILE=prog-strength-admin`. Local scripts, not a repository tool; sample
seed 20261104:

| Check | Result |
|:--|:--|
| Source manifests of generations 0–2 against this ledger | 36,398 shards and 906,632,620 records: exact for each generation, the first recount from the manifests |
| Inventory | 23,414 of 36,398 shards have a relabelled manifest and positions file. No extras or orphans, and no worker is partly done |
| Totals | The manifests' shards, records, changes, change sum and mates equal the three `task.json` files exactly |
| Provenance, all 23,414 manifests | Label id, net id and SHA-256, 5,000 nodes and commit `cb9e55b` throughout. Positions counts, games, results, seeds, settings and original provenance match the source manifest |
| Source checksums, generations 1–2 (15,485 shards) | Each manifest's source positions and games SHA-256s match the original manifest. Generation 0's originals have no checksums |
| Sample, 60 shards (1,481,843 records) | Both files' SHA-256s match, and all FENs and results are identical and in order. Each shard's change sum matches its manifest |
| Originals | Shard counts match the ledger. The newest object under generations 0–2 dates from 2026-10-31, before the run |

**Not checked:**
- This isn't the full audit, since the label set isn't complete.
- FENs, results and checksums of positions files outside the 60-shard sample.
- Determinism: no shard was re-run.
- The `relabel` profile's write policy wasn't read; the originals check is by
  listing.

**Not explained:** net7's scores run larger than the originals in the same
sample: mean \|score\| 769 against 294 in generation 0, 428 against 285 in
generation 1 and 385 against 291 in generation 2. So changes sit far above the
17–30 cp noise floor, and sign agreement and correlation fall below that
measurement's 97.7% and 0.990. The cause, and what it does to training targets,
hasn't been measured.

**2026-11-05,** of `N7` after run 41980717684, read-only with
`AWS_PROFILE=prog-strength-admin`. Local scripts, not a repository tool; same
sample seed, drawn from all 36,398 shards:

| Check | Result |
|:--|:--|
| Inventory | 36,398 of 36,398 shards have a relabelled manifest and positions file. No extras, orphans or empty files |
| Totals | Every run's `task.json` sums to 36,398 shards, 906,632,620 records, 894,301,324 changed, a change sum of 216,951,612,859 and 1,661,443 mates. All equal the manifests exactly. t1 of 41971756087 wrote no `task.json` and uploaded no shards |
| Provenance, all 36,398 manifests | Label id, net id and SHA-256, 5,000 nodes and commit `cb9e55b` throughout. Positions counts, games, results, seeds, settings and original provenance match the source manifest |
| Source checksums, generations 1–2 (27,069 shards) | Each manifest's source positions and games SHA-256s match the original manifest. Generation 0's originals have no checksums |
| Sample, 60 shards (1,503,924 records) | Both files' SHA-256s match, and all FENs and results are identical and in order. Each shard's change sum matches its manifest |
| Originals | Shard counts match the ledger. The newest object under generations 0–2 still dates from 2026-10-31 |

**Not checked, so `N7` is complete but not audited:**
- Records and checksums of positions files outside the 60-shard sample.
- Determinism: no shard was re-run.
- The `relabel` profile's write policy.
- The larger net7 scores noted on 2026-11-04 remain unexplained.

**2026-11-05, the full audit of `N7` on generations 0–2: passed.** Read-only with
`AWS_PROFILE=prog-strength-admin`, on an Apple M-series Mac with 16 cores, using
local scripts that aren't in the repository. The criteria were written down before
the run. All 906,632,620 records were read from 22 GB of positions files:

| Check | Result |
|:--|:--|
| 1. Inventory | 36,398 of 36,398 shards, with no extras or orphans |
| 2. Records | 0 failures: every count matches its source and manifest, and every FEN and result is identical and in order |
| 3. Provenance | 0 failures: label id, net, nodes and commit in every manifest, and every positions file's SHA-256 matches. Changes, change sum and largest change recomputed from the files equal each manifest |
| 4. Plausibility | Sign agreement 92.98–95.68% and correlation 0.9519–0.9700. Not what a wrong point of view would give, though below the same-net noise floor, as expected of a different net |
| 5. Determinism | **42 of 42 shards byte-identical** (6 from each of the 7 runs, seed 20261105), rebuilt from tag `v0.44.3`, and manifests equal apart from `seconds`. The cloud runs used x86-64 `c8a.16xlarge` and these re-runs Apple arm64, so the labels did not depend on the hardware in this sample. **Negative control:** 3 of them at 2,500 nodes all differ |
| 6. Originals | The `relabel` role's only S3 writes are under `selfplay/relabel/` and its task-results bucket. Source shard counts match this ledger, and all 27,069 source checksums in generations 1–2 match their files |

Scores against game results, a measure this audit added and set no bar for:

| Gen | Mean \|score\|, net7 / original | Loss against results, net7 / original | Sign matches a decisive result, net7 / original | \|score\| over 3,000, net7 |
|--:|--:|--:|--:|--:|
| 0 | 767 / 293 | 0.4804 / 0.5364 | 88.93% / 88.27% | 8,786,495 |
| 1 | 433 / 289 | 0.5687 / 0.5840 | 88.30% / 88.02% | 163,584 |
| 2 | 379 / 288 | 0.5312 / 0.5509 | 87.91% / 87.80% | 97,023 |

Loss is the binary cross-entropy of `sigmoid(score / 400)` against the game result.

- **net7's larger scores fit the results better at the training scale,** in every
  generation, so they don't look like a fault. This doesn't measure what they do
  to a net's strength, and why they're larger hasn't been measured either.
- **Scores beyond ±3,000 exist.** Only mates are clamped to ±3,000 by design, and
  9,047,102 other records score higher, up to the search's own values. The target
  saturates well before that. The originals' count wasn't taken.

**Not checked:**
- The `relabel` role can overwrite objects under `selfplay/relabel/`, so `N7`'s
  append-only status rests on the workflow, not on a policy.
- The other roles' write access to `selfplay/v1/`.
- Generation 0's source files against any earlier record, since their manifests
  have no checksums.

**2026-11-05,** `N7` on generations 0–2 re-audited with
`ci/label-set-audit.sh` (#199), the tool made from the scripts above, from an empty
directory at `cb9e55b`'s build: **passed**, with every number above reproduced, 42
of 42 shards byte-identical and all 3 negative-control shards different.

Its first run failed. The tool compares every field of the source manifest, which
the scripts above didn't, and 121 shards had a `seconds` that differs from the
source by at most 7e-15 of a second (62, 50 and 9 in generations 0, 1 and 2). The
difference appears to be `relabel` reading and rewriting the float through JSON;
that cause wasn't tested in isolation. Nothing reads that field, so it alone now
has a tolerance of 1e-9. So the earlier statement that metadata matches the source
holds for the fields it named, not for this one at full precision.

**2026-11-07, the full audit of `N7r` on generations 3–5: passed.** Run with
`ci/label-set-audit.sh` at `7cb6ea2`, read-only with
`AWS_PROFILE=prog-strength-admin`, on an Apple M3 Max (arm64, 16 cores), from an
empty directory. All 1,442,295,543 records were read from 17.7 GB of relabelled
positions and their sources:

```sh
ci/label-set-audit.sh s3://jwall-engineering-rusty-chess-nnue/data/selfplay \
  net7rl012-l050-r42054816508-n5000 3,4,5 <dir>
```

| Check | Result |
|:--|:--|
| 1. Inventory | 57,751 of 57,751 shards, with no extras or orphans. Equal to this ledger's 25,158 + 19,838 + 12,755 source shards |
| 2. Records | 0 failures: every count matches its source and manifest, and every FEN and result is identical and in order |
| 3. Provenance | 0 failures: label id, net SHA-256 `74af5b46…70ce`, 5,000 nodes and commit `7cb6ea2` in every manifest, and every positions file's SHA-256 matches. All 57,751 source checksums match their files |
| 4. Plausibility | Sign agreement 96.10–96.53% and correlation 0.9745–0.9817 ([the table above](#net7rl012-l050-r42054816508-n5000)). Not what a wrong point of view would give |
| 5. Determinism | **24 of 24 shards byte-identical** (6 from each of the 4 source runs, seed 20261105), rebuilt from `7cb6ea2`, and manifests equal apart from `seconds`. The cloud runs were x86-64 `c8a.16xlarge` and the re-runs Apple arm64. **Negative control:** 3 of them at 2,500 nodes all differ |
| 6. Originals | The `relabel` role's only S3 writes are under `selfplay/relabel/` and its task-results prefix, read from its two inline policies. Every object under `selfplay/v1/gen=3/` to `gen=5/` dates from 2026-11-01 or earlier, before the campaign |

Scores against game results, as for `N7`:

| Gen | Mean \|score\|, net7rl012 / original | Loss against results, net7rl012 / original | Sign matches a decisive result, net7rl012 / original | \|score\| over 3,000, net7rl012 |
|--:|--:|--:|--:|--:|
| 3 | 375 / 290 | 0.5427 / 0.5563 | 88.35% / 88.42% | 456,043 |
| 4 | 347 / 296 | 0.5526 / 0.5599 | 87.68% / 87.80% | 277,504 |
| 5 | 324 / 291 | 0.5580 / 0.5619 | 87.64% / 87.84% | 106,158 |

- **The new scores fit the results better** by loss in every generation, by less
  in each newer one, as `N7` did on generations 0–2. Their signs match a decisive
  result slightly less often, by 0.07–0.20 points. The scores are also larger,
  and loss rewards confidence, so the two measures can disagree.
- **The scores grew less than net7's did:** a mean \|score\| 11–29% above the
  originals, against 32–162% for `N7` on generations 0–2.

**Not checked:**
- **Which relabel run made which shard:** manifests don't record it. So the
  determinism sample was drawn by source run, and isn't known to cover all 9
  instances.
- The `relabel` role can still overwrite objects under `selfplay/relabel/`, as
  noted for `N7`.
- A training set that mixes `N7` and `N7r`: nothing has been prepared from one yet.

**2026-11-09, the full audit of generation 6: GO.** `ci/datagen-full-audit.sh`
on a download of the whole generation (14 GB, 102,160 objects) to an Apple M3
Max, read-only. Every figure in the generation's section comes from
`summary.json`, `inventory.json` and the 173 batch logs; the run table's shards,
positions and games are summed from the downloaded manifests by run, and the
runs' times, zones and prices from their job logs.

| Check | Result |
|:--|:--|
| Inventory | 0 P0 problems; 34,053 shards, 320 workers, 5 tasks, one engine, one net, one settings block |
| Every record | 0 failures over 849,868,621 records; bulletformat exact |
| P1 bars | Passed in all 173 batches |
| Determinism | 3,840 of 3,840 games identical, every worker |
| Manifests against the ledger's Generations row | Equal: 849,868,621 positions, 9,662,116 games, 34,053 shards, 5 runs |

**Not checked:** the billed cost (the dollar figure is from launch prices and
job hours); the manager's own hour count misses the cancelled run.

**2026-11-15,** the [bucket move](#the-bucket-move), with
`ci/bucket-copy-audit.sh` as added by #267, read-only with
`AWS_PROFILE=prog-strength-admin`, from 20:32 to 20:36 UTC:

```sh
ci/bucket-copy-audit.sh jwall-engineering-rusty-chess-nnue jwall-engineering-braveheart-nnue <dir>
```

| Check | Result |
|:--|:--|
| Objects and bytes in each bucket, from `list-objects-v2` | 573,051 objects and 84,480,796,787 bytes in both |
| Every source key is in the destination with the same size | All 573,051; none missing, none of another size |
| Every source key has the same ETag in the destination | All 573,051 |
| Keys only in the destination | None |
| What an equal ETag means here | No ETag in either listing is a multipart one (none has a `-`), the largest object is 4,744,768 bytes, and both buckets use SSE-S3 (`AES256`), so every ETag is the MD5 of the object's bytes |
| Objects under each prefix | `data/selfplay/v1/`: 384,618 objects, 55,466,430,799 bytes. `data/selfplay/relabel/`: 188,298 objects, 28,965,933,481 bytes. `data/nets/`: 135 objects, 48,432,507 bytes |
| `head-object` on 6 objects in both buckets (the largest object, which is a `net.nnue`, another `net.nnue`, and four chosen at random): metadata, content type, length, ETag, encryption | Identical for all 6. None carries user metadata |
| `net.nnue` of the 6 retired nets, downloaded from the new bucket, against the SHA-256 in `crates/search/src/nnue.rs` | All 6 match: `net1-l050-r41022503872`, `net2x80-l050-r41131366975`, `net4w512-l050-r41170687764`, `net5-l050-r41229287546`, `net6k4-l050-r41282883885`, `net7rl012-l050-r42054816508` |

**Not checked:** no object was read back and hashed except those 6 nets; the
rest rests on the ETags. No `datagen-audit` and no label-set audit was re-run
from the new bucket, and no audit verdict changes. Whether anything wrote to
the old bucket during the copy was not watched; the two listings, taken after
it, are equal, and the last Datagen, Relabel and Train runs before it were on
2026-11-09, 2026-11-07 and 2026-11-13. The copied objects start in S3 Standard
and move to Intelligent-Tiering by the bucket's lifecycle rule; storage class
was not compared.

**2026-11-16,** the [bucket move](#the-bucket-move) to this project's bucket,
with `ci/bucket-copy-audit.sh --prefix` at this repository's `2cf7af5`,
read-only with `AWS_PROFILE=jwall-admin` (the owner's Identity Center admin
role), from 14:51 to 14:55 UTC:

```sh
ci/bucket-copy-audit.sh --prefix data/selfplay/ jwall-engineering-braveheart-nnue jwall-engineering-braveheart-selfplay-positions <dir>
```

| Check | Result |
|:--|:--|
| Objects and bytes under `data/selfplay/` in each bucket, from `list-objects-v2` | 572,916 objects and 84,432,364,280 bytes in both |
| Every source key is in the destination with the same size | All 572,916; none missing, none of another size |
| Every source key has the same ETag in the destination | All 572,916 |
| Keys only in the destination | None |
| What an equal ETag means here | No ETag in either listing is a multipart one, the largest object is 320,178 bytes, and both buckets use SSE-S3 (`AES256`), so every ETag is the MD5 of the object's bytes |
| Objects under each prefix, against the 2026-11-15 audit | `data/selfplay/v1/`: 384,618 objects, 55,466,430,799 bytes. `data/selfplay/relabel/`: 188,298 objects, 28,965,933,481 bytes. Both exactly as on 2026-11-15 |
| `head-object` on 4 objects in both buckets, chosen at random (a positions and a games shard under `v1/`, a positions shard and a manifest under `relabel/`): length, ETag, content type, encryption, metadata | Identical for all 4. None carries user metadata |

The same audit of `data/nets/`, with the retired nets' SHA-256s, is in
braveheart-trainer's
[nets.md](https://github.com/Braveheart-Chess/braveheart-trainer/blob/main/docs/nets.md#audit-log).

**Not checked:** no object was read back and hashed; the result rests on the
ETags. No `datagen-audit` and no label-set audit was re-run from the new bucket,
and no audit verdict changes. The listings compare current versions only.
Whether anything wrote to the old bucket during the copy was not watched; the
two listings, taken after it, are equal, and hold exactly the objects the
2026-11-15 audit counted. Storage class was not compared.

**2026-11-18,** the two proving runs after the cutover
([generation 7's smoke test](#generation-7-net8x160) and
[`N8x` on part of generation 6](#net8x160-l050-r43390801430-n5000)), read-only
with `AWS_PROFILE=jwall-admin`: both new prefixes listed, and every manifest
under them read, with local scripts that aren't in the repository. The runs'
`task.json` files are from their artifacts, and times, zones and prices from
their job logs.

| Check | Result |
|:--|:--|
| Objects under `data/selfplay/v1/gen=7/` | 769 objects, 103,769,667 bytes, all under `run=20261118-44254770264-1/task=t0/`: 256 positions files, 256 games files, 256 manifests and the task's `task.json` |
| Generation 7's manifests against its `task.json` | 256 shards, 5,905,655 positions and 66,603 games in both |
| Generation 7's provenance | One engine (`0.54.0` `3fd0223`, bench 919,304), one `selfplay_sha` (`afcfc85`), one net (`net8x160-l050-r43390801430`, `8ea67605…556c`) and one settings block, with generation 7; 64 workers, 64 distinct seeds |
| Objects under `data/selfplay/relabel/net8x160-l050-r43390801430-n5000/` | 1,236 objects, 190,635,049 bytes, all under `gen=6/run=20261108-42524365295-1/task=t0/`: 618 positions files and 618 manifests |
| The label set's manifests against its `task.json` | 618 shards, 15,412,763 records, 15,024,792 changed, a change sum of 809,774,335 and 861 mates in both |
| Its provenance, all 618 manifests | Label id, net and SHA-256, 5,000 nodes, engine `3fd0223` and `selfplay_sha` `afcfc85` throughout |
| Its manifests against their 618 source manifests | Equal records and source positions count, source positions SHA-256, games file SHA-256, games, results, seed and original provenance for all 618 |
| Which source shards | Workers `w000`–`w005` of run `20261108-42524365295-1` task t0, each whole: 100, 103, 102, 105, 104 and 104 shards, as many as their sources |
| Generation 6's source shards | 34,053 manifests, as this ledger and the run's `shards_todo_at_start` say |

**Not checked:** no positions or games file was read or hashed; no
`datagen-audit` sample, label-set audit or determinism re-run; sign agreement and
correlation of the new labels; the billed cost (the dollar figures are from the
launch prices and the instances' times in the job logs).

**2026-11-18, `N8x` deleted** (owner's decision), with `AWS_PROFILE=jwall-admin`:
`aws s3 rm --recursive` on
`s3://jwall-engineering-braveheart-selfplay-positions/data/selfplay/relabel/net8x160-l050-r43390801430-n5000/`.
The bucket is versioned, so that adds a delete marker to each object and keeps
every version.

| Check | Result |
|:--|:--|
| Before: current objects under the prefix | 1,236 objects, 190,635,049 bytes, all under `gen=6/`, written 07:03–07:10 UTC by run [44254774637](https://github.com/Braveheart-Chess/braveheart-selfplay/actions/runs/44254774637) (the counts in the 2026-11-18 entry above) |
| After: current objects under the prefix | 0; the prefix no longer lists |
| After: versions | 1,236 versions kept, 1,236 delete markers. Removing the markers restores the objects |
| The other label sets | `N7` 72,796 objects (11,251,825,156 bytes) and `N7r` 115,502 (17,714,108,325 bytes): 188,298 together, the relabel total of the 2026-11-15 audit |

**Not checked:** the other label sets' contents (only their listings), and the
kept versions' bytes.

**2026-11-19,** the copy to the production account's bucket
([the bucket move](#the-bucket-move)). Read-only: the production admin role for
both buckets' objects, and `AWS_PROFILE=jwall-admin` for this bucket's version
history. The tools are `ci/bucket-copy-audit.sh`, `ci/bucket-copy-sample.py`
(#22), `ci/datagen-full-audit.sh`, and `ci/label-set-audit.sh` with #23's fix.
A baseline listing of this bucket was taken on 2026-11-18 at 23:30 UTC, before
anything was copied.

```sh
ci/bucket-copy-audit.sh --prefix data/ jwall-engineering-braveheart-selfplay-positions jaws-braveheart-selfplay-positions <dir>
ci/bucket-copy-sample.py --prefix data/ jwall-engineering-braveheart-selfplay-positions jaws-braveheart-selfplay-positions <dir>
```

| Check | Result |
|:--|:--|
| The trial copy of generation 7, before the rest | 769 objects: all with the same size and ETag, and all 769 with the same SHA-256 when downloaded from both buckets |
| Objects and bytes under `data/` in each bucket, from `list-objects-v2` | 573,685 objects and 84,536,133,947 bytes in both, and in the baseline: the 2026-11-16 audit's 572,916 and 84,432,364,280 plus generation 7's 769 and 103,769,667 |
| Every source key is in the destination with the same size and ETag | All 573,685; none missing, none of another size |
| Keys only in the destination | None |
| What an equal ETag means here | No ETag in either listing is a multipart one, the largest object is 320,178 bytes, and both buckets use SSE-S3 (`AES256`), so every ETag is the MD5 of the object's bytes |
| Versions in the destination | 573,685, one for each object; no delete markers |
| SHA-256 of objects downloaded from both buckets: one at random from each of the 141 directories, plus 0.5% of all objects at random (seed 20261118) | 3,009 objects, 444,179,505 bytes from each bucket: all identical |
| `head-object` on 25 of generation 7's objects in both buckets: length, ETag, content type, encoding, encryption, metadata | Identical for all 25. None carries user metadata. Storage class Standard in both |
| `ci/datagen-full-audit.sh` at `afcfc85` on generation 7, downloaded from the destination | GO: 256 shards, 66,603 games, 5,905,655 records, 0 failures; the first 12 games of all 64 workers regenerated identically |
| The engine repository's `ci/datagen-full-audit.sh` at `863241e` on generation 6's run `20261108-42536885510-1`, downloaded from the destination | GO: 640 shards, 171,021 games, 15,023,775 records, 0 failures; the first 12 games of all 64 workers regenerated identically |
| `ci/label-set-audit.sh` on generation 0 of `net7-l050-r41363264869-n5000`, read from the destination | PASS: 9,329 shards, 231,240,344 records checked against their sources, 0 failures; 18 of 18 positions files byte-identical across 3 re-labellings; all 3 half-nodes controls differ |
| `ci/label-set-audit.sh` on generation 5 of `net7rl012-l050-r42054816508-n5000`, read from the destination | PASS: 12,755 shards, 318,934,530 records, 0 failures; 6 of 6 positions files byte-identical; all 3 half-nodes controls differ |
| This bucket against the baseline, after the copy | The same 573,685 keys, each with the same size, ETag and last-modified time; the same 1,236 noncurrent versions and 1,236 delete markers; versioning and encryption unchanged |

**Not checked:** objects were read back and hashed for the sample and for
generation 7, not all 573,685; the rest rests on the ETags. The domain audits
read generation 7, one run of generation 6 and one generation of each label
set, not every generation. Noncurrent versions were not copied, so nothing of
`N8x` is in the new bucket. An object's last-modified time in the new bucket is
the copy's. No task has run in the production account, so its task roles'
grants on the new bucket are unexercised. The copy's script and the audit's
wrapper are local scripts that aren't in this repository.

**2026-11-19,** the re-audit before the cutover
([the bucket move](#the-bucket-move)). Read-only, with `AWS_PROFILE=jaws-prod`
(the production admin role, which reads the management account's bucket through
its policy) and AWS CLI 2.34.59, using `ci/bucket-copy-audit.sh` at `261bcb4`. The
listings were taken at 14:29:36 UTC (management) and 14:31:25 UTC (production).
At the time, no task instance was running in either account, and no Datagen or
Relabel run had started in this repository since the two proving runs of
2026-11-18 (06:54 UTC).

```sh
ci/bucket-copy-audit.sh --prefix data/ jwall-engineering-braveheart-selfplay-positions jaws-braveheart-selfplay-positions <dir>
```

| Check | Result |
|:--|:--|
| Objects and bytes under `data/` in each bucket | 573,685 objects and 84,536,133,947 bytes in both: the same totals as this morning's audit of the copy |
| Every source key is in the destination with the same size and ETag | All 573,685; none missing, none of another size, none with another ETag |
| Keys only in the destination | None |
| The two `list-objects-v2` listings (key, size, ETag), byte for byte | Identical: SHA-256 `3a3a82b9f4cd420e6e0f531e6a1779562e99897b8519d2108f2f0a5852c63d1a` for both |

So the final `sync` had nothing to copy and was not run. Equal ETags mean equal
bytes here, for the reasons the copy's audit gives (single-part uploads, SSE-S3).

**Not checked:** no object was downloaded or hashed again, and the domain
audits were not repeated; they stand from the audit of the copy, since every
object is unchanged. Versions were not compared. The production task roles'
grants are still unexercised until the first run after the cutover.
