# Changelog

## 0.2.0 (2024-02-15)

* Bumped minimal supported rust version to 1.75
* Moved all traits to use the new ["return position impl trait in trait" feature](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html)
* Update to new wnfs-common `BlockStore` trait
* Update rug version to `1.24` and make encoding big integers more efficient
* Evolve `BlockStore` trait ([#402](https://github.com/wnfs-wg/rs-wnfs/pull/402))

## 0.1.26 (2023-12-06)

* Abstracted out the integer library used to support multiple backends ([#373](https://github.com/wnfs-wg/rs-wnfs/pull/373)).
  Every type now has a `<B: Big>` type parameter, where `Big` is a trait that abstracts over integer operations.
  By default `B` is set to the `DefaultBig` backend, which is based on either `num-bigint-dig` (enabled by default) or `rug` (enabled with the `rug` feature).
* Modified all use of integer serialization to use big-endian, to reduce room for error and match specification plans (see [#76](https://github.com/wnfs-wg/spec/pull/76) in the spec).

## [0.1.25](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-nameaccumulator-v0.1.24...wnfs-nameaccumulator-v0.1.25) (2023-08-30)


### Bug Fixes

* Fix `search_latest` on `write` and file mountpoints ([#341](https://github.com/wnfs-wg/rs-wnfs/issues/341)) ([dae79cd](https://github.com/wnfs-wg/rs-wnfs/commit/dae79cd1b95148cf54d6fdf57357b76adcf192ae))

## [0.1.24](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-nameaccumulator-v0.1.23...wnfs-nameaccumulator-v0.1.24) (2023-08-17)


### Bug Fixes

* More reliably cache `NameAccumulator` modexps ([#326](https://github.com/wnfs-wg/rs-wnfs/issues/326)) ([380ee8c](https://github.com/wnfs-wg/rs-wnfs/commit/380ee8c7b07a73912100c2689334596e3ad8d9c0))

## [0.1.23](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-nameaccumulator-v0.1.22...wnfs-nameaccumulator-v0.1.23) (2023-07-21)


### Features

* **api:** adds missing metadata functions for the private side ([#146](https://github.com/wnfs-wg/rs-wnfs/issues/146)) ([88e9f19](https://github.com/wnfs-wg/rs-wnfs/commit/88e9f19a69fbbb99e3ee78c831eeb520a33f0b46))
* Switch from Namefilter to Name Accumulators ([#247](https://github.com/wnfs-wg/rs-wnfs/issues/247)) ([7026a37](https://github.com/wnfs-wg/rs-wnfs/commit/7026a379443038fa1b0410df1c7d0bc23649f17a))
* Switch from SHA3-256 to BLAKE3-256 ([#306](https://github.com/wnfs-wg/rs-wnfs/issues/306)) ([e164a1f](https://github.com/wnfs-wg/rs-wnfs/commit/e164a1fc80c30d9446404a61b05fd995d7d88c0e))


### Miscellaneous Chores

* release 0.1.10 ([#114](https://github.com/wnfs-wg/rs-wnfs/issues/114)) ([9cbc320](https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
* release 0.1.16 ([#178](https://github.com/wnfs-wg/rs-wnfs/issues/178)) ([89e4d36](https://github.com/wnfs-wg/rs-wnfs/commit/89e4d36dc9b27ec1ab67db6fc214670efe768f32))
* release 0.1.19 ([1f37ec4](https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))
* release 0.1.21 ([#255](https://github.com/wnfs-wg/rs-wnfs/issues/255)) ([2be9f49](https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))
* **wnfs-nameaccumulator:** Initial release at 0.1.23 ([eb17ea2](https://github.com/wnfs-wg/rs-wnfs/commit/eb17ea2fa03e248a189cb8db04a033ef542f26db))
