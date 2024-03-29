# Changelog

## 0.2.0 (2024-02-15)

* Bumped minimal supported rust version to 1.75
* Moved all traits to use the new ["return position impl trait in trait" feature](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html)
* Added two functions to `BlockStore` trait:
  - `has_block` for finding out if a blockstore has a block available locally
  - `put_block_keyed` for adding a block to the blockstore with a pre-computed CID
* Added a default implementation for `BlockStore::put_block` utilizing `put_block_keyed` and `create_cid`
* Removed `put_serializable` and `get_serializable` functions from `BlockStore`. Use `Storable::store` and `Storable::load` instead.
* Made all `Result`s in `BlockStore` use `BlockStoreError` as error type instead of `anyhow::Error`
* Expose a blanket implementation of `BlockStore` for any `&impl BlockStore` or `Box<impl BlockStore>`
* Evolve `BlockStore` trait ([#402](https://github.com/wnfs-wg/rs-wnfs/pull/402))

## 0.1.26 (2023-12-06)

* Removed `AsyncSerialize` and `RemembersCid` traits.
* Added `Storable`, `StoreIpld` and `LoadIpld` traits.
* Removed `BlockStore::store_async_serializable`. Use `Storable::store` instead.

These traits allow the `Link` type to be used with data that doesn't necessarily encode as `dag-cbor`, such as `UnixFS` files, which encode as `dag-pb`.
For details see [#378](https://github.com/wnfs-wg/rs-wnfs/pull/378).

## 0.1.25 (2023-09-04)

* Small documentation improvements

## [0.1.23](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-common-v0.1.22...wnfs-common-v0.1.23) (2023-07-21)


### Features

* Switch from Namefilter to Name Accumulators ([#247](https://github.com/wnfs-wg/rs-wnfs/issues/247)) ([7026a37](https://github.com/wnfs-wg/rs-wnfs/commit/7026a379443038fa1b0410df1c7d0bc23649f17a))
* Switch from SHA3-256 to BLAKE3-256 ([#306](https://github.com/wnfs-wg/rs-wnfs/issues/306)) ([e164a1f](https://github.com/wnfs-wg/rs-wnfs/commit/e164a1fc80c30d9446404a61b05fd995d7d88c0e))


### Miscellaneous Chores

* **wnfs-nameaccumulator:** Initial release at 0.1.23 ([eb17ea2](https://github.com/wnfs-wg/rs-wnfs/commit/eb17ea2fa03e248a189cb8db04a033ef542f26db))

## [0.1.22](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-common-v0.1.21...wnfs-common-v0.1.22) (2023-06-23)


### Features

* make changes to BlockStore trait based on feedback ([#286](https://github.com/wnfs-wg/rs-wnfs/issues/286)) ([085242d](https://github.com/wnfs-wg/rs-wnfs/commit/085242d15aa48db17d77ed45e1c7717d13ed105f))

## [0.1.21](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-common-v0.1.20...wnfs-common-v0.1.21) (2023-05-22)


### Miscellaneous Chores

* release 0.1.21 ([#255](https://github.com/wnfs-wg/rs-wnfs/issues/255)) ([2be9f49](https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))

## [0.1.20](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-common-v0.1.19...wnfs-common-v0.1.20) (2023-03-30)


### Features

* adding mutation api for metadata. cleaning up clippy complaints ([#217](https://github.com/wnfs-wg/rs-wnfs/issues/217)) ([05f3739](https://github.com/wnfs-wg/rs-wnfs/commit/05f3739cdc4b5b9cb02427c51e5ddff6803122bd))

## [0.1.19](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-common-v0.1.18...wnfs-common-v0.1.19) (2023-03-23)


### Miscellaneous Chores

* release 0.1.19 ([1f37ec4](https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))
