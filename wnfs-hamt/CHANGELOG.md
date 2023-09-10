# Changelog

## 0.1.25 (2023-09-04)

* Fixed a bug causing dropped updates when doing serialization, then continuing writes, then serializing again and loading from that serialized state [#348](https://github.com/wnfs-wg/rs-wnfs/pull/348)
* Dependency wnfs-common bumped from 0.1.24 to 0.1.25

## [0.1.23](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-hamt-v0.1.22...wnfs-hamt-v0.1.23) (2023-07-21)


### Features

* Switch from Namefilter to Name Accumulators ([#247](https://github.com/wnfs-wg/rs-wnfs/issues/247)) ([7026a37](https://github.com/wnfs-wg/rs-wnfs/commit/7026a379443038fa1b0410df1c7d0bc23649f17a))
* Switch from SHA3-256 to BLAKE3-256 ([#306](https://github.com/wnfs-wg/rs-wnfs/issues/306)) ([e164a1f](https://github.com/wnfs-wg/rs-wnfs/commit/e164a1fc80c30d9446404a61b05fd995d7d88c0e))


### Miscellaneous Chores

* **wnfs-nameaccumulator:** Initial release at 0.1.23 ([eb17ea2](https://github.com/wnfs-wg/rs-wnfs/commit/eb17ea2fa03e248a189cb8db04a033ef542f26db))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.22 to 0.1.23
  * dev-dependencies
    * wnfs-common bumped from 0.1.22 to 0.1.23

## [0.1.22](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-hamt-v0.1.21...wnfs-hamt-v0.1.22) (2023-06-23)


### Features

* make changes to BlockStore trait based on feedback ([#286](https://github.com/wnfs-wg/rs-wnfs/issues/286)) ([085242d](https://github.com/wnfs-wg/rs-wnfs/commit/085242d15aa48db17d77ed45e1c7717d13ed105f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.21 to 0.1.22
  * dev-dependencies
    * wnfs-common bumped from 0.1.21 to 0.1.22

## [0.1.21](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-hamt-v0.1.20...wnfs-hamt-v0.1.21) (2023-05-22)


### Miscellaneous Chores

* release 0.1.21 ([#255](https://github.com/wnfs-wg/rs-wnfs/issues/255)) ([2be9f49](https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.20 to 0.1.21
  * dev-dependencies
    * wnfs-common bumped from 0.1.20 to 0.1.21

## [0.1.19](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-hamt-v0.1.18...wnfs-hamt-v0.1.19) (2023-03-23)


### Miscellaneous Chores

* release 0.1.19 ([1f37ec4](https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.18 to 0.1.19
  * dev-dependencies
    * wnfs-common bumped from 0.1.18 to 0.1.19
