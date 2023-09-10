# Changelog

## [0.1.23](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-common-v0.1.24...wnfs-common-v0.1.23) (2023-09-10)


### Features

* adding mutation api for metadata. cleaning up clippy complaints ([#217](https://github.com/wnfs-wg/rs-wnfs/issues/217)) ([05f3739](https://github.com/wnfs-wg/rs-wnfs/commit/05f3739cdc4b5b9cb02427c51e5ddff6803122bd))
* **api:** adds missing metadata functions for the private side ([#146](https://github.com/wnfs-wg/rs-wnfs/issues/146)) ([88e9f19](https://github.com/wnfs-wg/rs-wnfs/commit/88e9f19a69fbbb99e3ee78c831eeb520a33f0b46))
* Implement storing externally encrypted content in `Metadata` ([#340](https://github.com/wnfs-wg/rs-wnfs/issues/340)) ([2d15fbd](https://github.com/wnfs-wg/rs-wnfs/commit/2d15fbdf61f0461435b1df4339879394859118b5))
* make changes to BlockStore trait based on feedback ([#286](https://github.com/wnfs-wg/rs-wnfs/issues/286)) ([085242d](https://github.com/wnfs-wg/rs-wnfs/commit/085242d15aa48db17d77ed45e1c7717d13ed105f))
* Make log optional ([#189](https://github.com/wnfs-wg/rs-wnfs/issues/189)) ([12cd842](https://github.com/wnfs-wg/rs-wnfs/commit/12cd8428514d7c145b443a78e279dc468fa01a91))
* Redundant sha2 ([#191](https://github.com/wnfs-wg/rs-wnfs/issues/191)) ([231f4e9](https://github.com/wnfs-wg/rs-wnfs/commit/231f4e929378d7a02c9f7f8b095f1c2b1175ec2e))
* Remove `Share` struct, add documentation, add `rc` constructor variants ([#343](https://github.com/wnfs-wg/rs-wnfs/issues/343)) ([e6cee87](https://github.com/wnfs-wg/rs-wnfs/commit/e6cee873273e154c7855d17e9c756717a635874b))
* Switch from Namefilter to Name Accumulators ([#247](https://github.com/wnfs-wg/rs-wnfs/issues/247)) ([7026a37](https://github.com/wnfs-wg/rs-wnfs/commit/7026a379443038fa1b0410df1c7d0bc23649f17a))
* Switch from SHA3-256 to BLAKE3-256 ([#306](https://github.com/wnfs-wg/rs-wnfs/issues/306)) ([e164a1f](https://github.com/wnfs-wg/rs-wnfs/commit/e164a1fc80c30d9446404a61b05fd995d7d88c0e))


### Miscellaneous Chores

* release 0.1.10 ([#114](https://github.com/wnfs-wg/rs-wnfs/issues/114)) ([9cbc320](https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
* release 0.1.16 ([#178](https://github.com/wnfs-wg/rs-wnfs/issues/178)) ([89e4d36](https://github.com/wnfs-wg/rs-wnfs/commit/89e4d36dc9b27ec1ab67db6fc214670efe768f32))
* release 0.1.19 ([1f37ec4](https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))
* release 0.1.21 ([#255](https://github.com/wnfs-wg/rs-wnfs/issues/255)) ([2be9f49](https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))
* **wnfs-nameaccumulator:** Initial release at 0.1.23 ([eb17ea2](https://github.com/wnfs-wg/rs-wnfs/commit/eb17ea2fa03e248a189cb8db04a033ef542f26db))

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
