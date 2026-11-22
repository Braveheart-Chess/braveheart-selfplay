# Changelog

## [0.5.2](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.5.1...v0.5.2) (2026-11-22)


### Bug Fixes

* **repo:** let dependabot's capitalised bump titles pass ([#36](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/36)) ([33b0b78](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/33b0b78afcb7d9aad167d05acafb4093bf957112))

## [0.5.1](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.5.0...v0.5.1) (2026-11-19)


### Documentation

* record the production Datagen smoke tests before and after the Spot service-linked role ([#27](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/27)) ([cb76e38](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/cb76e38a88a27244a876f64b35993dfd3d9a73d6))

## [0.5.0](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.4.0...v0.5.0) (2026-11-19)


### Features

* move Datagen and Relabel to the production account's positions bucket ([#25](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/25)) ([d873589](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/d873589539cb00df73ed6b2d76ebf1ce3586f2bf))

## [0.4.0](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.3.3...v0.4.0) (2026-11-19)


### Features

* **ci:** check a bucket copy byte for byte on a stratified sample ([#22](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/22)) ([9daa440](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/9daa4402fb5faf504cb4331305d8b58f26011900))


### Bug Fixes

* **ci:** build relabel from whichever crate holds it at the label set's engine commit ([#23](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/23)) ([74b5a38](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/74b5a38eb1894dcd65f7821a399a693d1e68e11a))


### Documentation

* record deleting the partial N8x label set and closing generation 7 at its smoke test ([#20](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/20)) ([c03f555](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/c03f5552e47477e87c16e4a36594b9bbeb852214))
* record the copy of the positions to the production account and its audit ([#24](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/24)) ([4a93755](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/4a937556050196d51dfb4c8e36b7169a8259cd7c))
* record the first Datagen and Relabel runs from this repository, after the cutover ([#18](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/18)) ([14e7c7f](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/14e7c7fa3e6d11b017807f603cb9e4bd86bef6ad))

## [0.3.3](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.3.2...v0.3.3) (2026-11-18)


### Documentation

* show off the self-play dataset in the readme, and fix three stale doc lines ([#15](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/15)) ([a23b408](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/a23b40898864ace922973e79be0e16f828b56664))
* sign in with the jwall-admin Identity Center profile for the datagen and relabel managers ([#17](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/17)) ([5c1d512](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/5c1d5126ee3e1c13aeb6e35bbb1d647753f66cd6))

## [0.3.2](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.3.1...v0.3.2) (2026-11-16)


### Documentation

* record the copy to this bucket and its audit; datasets.md is the ledger now ([#11](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/11)) ([29ca1b7](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/29ca1b7b551f0c0b86da9eb5a6ad93dc8bb58088))

## [0.3.1](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.3.0...v0.3.1) (2026-11-16)


### Documentation

* describe the positions' consumers as this project's half of the handshake ([#10](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/10)) ([5061843](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/5061843db08650b5a1a2a44993d819207b49308b))
* open the readme with the organisation banner ([#8](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/8)) ([4ae2db5](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/4ae2db5bf8c54247999e71b93ba1f06a4062b4db))

## [0.3.0](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.2.0...v0.3.0) (2026-11-16)


### Features

* share the positions, read-only, with braveheart-databricks ([#6](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/6)) ([93d8abc](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/93d8abc0915686da4416a18adce558cae3e3cb0e))

## [0.2.0](https://github.com/Braveheart-Chess/braveheart-selfplay/compare/v0.1.0...v0.2.0) (2026-11-16)


### Features

* declare the positions dataset, versioned, read by the trainer's GPU tasks ([#3](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/3)) ([090fa53](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/090fa53afa9715226a0cfe08d9badcdf549b19ba))
* port Datagen and Relabel to this project, its bucket and the pinned engine ([#4](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/4)) ([298b239](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/298b2398ded498c0e28a64ec322a824ab38ee761))

## 0.1.0 (2026-11-16)


### Features

* build standalone against the engine at v0.54.0, with CI and the repo conventions ([#1](https://github.com/Braveheart-Chess/braveheart-selfplay/issues/1)) ([031ba0f](https://github.com/Braveheart-Chess/braveheart-selfplay/commit/031ba0f700ee6fcb4d3c3019ea1b9bbd3908fd02))
