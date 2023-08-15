# Changelog

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.10 to 0.1.11

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.12 to 0.1.13

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.19 to 0.1.20

## [0.1.24](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.23...wnfs-wasm-v0.1.24) (2023-08-15)


### Features

* Implement public directory cp & more efficient copy for `PrivateFile` ([#319](https://github.com/wnfs-wg/rs-wnfs/issues/319)) ([cebb956](https://github.com/wnfs-wg/rs-wnfs/commit/cebb956cdaf88ed6e2eb09b784eeec5d61bdf4c8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.23 to 0.1.24

## [0.1.23](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.22...wnfs-wasm-v0.1.23) (2023-07-21)


### Features

* expose AccessKey encode/decode api ([#296](https://github.com/wnfs-wg/rs-wnfs/issues/296)) ([982feff](https://github.com/wnfs-wg/rs-wnfs/commit/982feff849a3f42bb859636a68324b3c6a550a91))
* Switch from AES-GCM to XChaCha20-Poly1305 ([#305](https://github.com/wnfs-wg/rs-wnfs/issues/305)) ([c17f6bb](https://github.com/wnfs-wg/rs-wnfs/commit/c17f6bb5bc9369d94d1c57cfa66c6cc2adf8174b))
* Switch from Namefilter to Name Accumulators ([#247](https://github.com/wnfs-wg/rs-wnfs/issues/247)) ([7026a37](https://github.com/wnfs-wg/rs-wnfs/commit/7026a379443038fa1b0410df1c7d0bc23649f17a))
* Switch from SHA3-256 to BLAKE3-256 ([#306](https://github.com/wnfs-wg/rs-wnfs/issues/306)) ([e164a1f](https://github.com/wnfs-wg/rs-wnfs/commit/e164a1fc80c30d9446404a61b05fd995d7d88c0e))


### Miscellaneous Chores

* **wnfs-nameaccumulator:** Initial release at 0.1.23 ([eb17ea2](https://github.com/wnfs-wg/rs-wnfs/commit/eb17ea2fa03e248a189cb8db04a033ef542f26db))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.22 to 0.1.23
    * wnfs-nameaccumulator bumped from 0.1.22 to 0.1.23

## [0.1.22](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.21...wnfs-wasm-v0.1.22) (2023-06-23)


### Features

* make changes to BlockStore trait based on feedback ([#286](https://github.com/wnfs-wg/rs-wnfs/issues/286)) ([085242d](https://github.com/wnfs-wg/rs-wnfs/commit/085242d15aa48db17d77ed45e1c7717d13ed105f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.21 to 0.1.22

## [0.1.21](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.20...wnfs-wasm-v0.1.21) (2023-05-22)


### ⚠ BREAKING CHANGES

* get_node should return null on missing path ([#253](https://github.com/wnfs-wg/rs-wnfs/issues/253))

### Bug Fixes

* get_node should return null on missing path ([#253](https://github.com/wnfs-wg/rs-wnfs/issues/253)) ([5ed87fe](https://github.com/wnfs-wg/rs-wnfs/commit/5ed87fe6359a19abdea5f34dd0537fd5d62c98a8))


### Miscellaneous Chores

* release 0.1.21 ([#255](https://github.com/wnfs-wg/rs-wnfs/issues/255)) ([2be9f49](https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.20 to 0.1.21

## [0.1.19](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.18...wnfs-wasm-v0.1.19) (2023-03-23)


### Miscellaneous Chores

* release 0.1.19 ([1f37ec4](https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.18 to 0.1.19

## [0.1.18](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.17...wnfs-wasm-v0.1.18) (2023-03-23)


### Bug Fixes

* `find_latest_share_counter` finds the last share count ([#197](https://github.com/wnfs-wg/rs-wnfs/issues/197)) ([69ffeec](https://github.com/wnfs-wg/rs-wnfs/commit/69ffeeca20cc3106e6d733e2d5adf5f87987630c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.17 to 0.1.18

## [0.1.17](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.16...wnfs-wasm-v0.1.17) (2023-02-24)


### Features

* **api:** add privateforest merge and diff bindings ([#181](https://github.com/wnfs-wg/rs-wnfs/issues/181)) ([231ece4](https://github.com/wnfs-wg/rs-wnfs/commit/231ece4309cab86d4682693e8e31f8ed99478a1f))
* PrivateLink abstraction ([#172](https://github.com/wnfs-wg/rs-wnfs/issues/172)) ([f04fa89](https://github.com/wnfs-wg/rs-wnfs/commit/f04fa89738e19a095d177e18b35d7e153c380833))
* Remove `base_history_on` and auto-track history instead ([#174](https://github.com/wnfs-wg/rs-wnfs/issues/174)) ([806bbb9](https://github.com/wnfs-wg/rs-wnfs/commit/806bbb93b1f03983165375005e14a9b63ebe67c2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.16 to 0.1.17

## [0.1.16](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.15...wnfs-wasm-v0.1.16) (2023-02-22)


### ⚠ BREAKING CHANGES

* **exports:** make re-exports more flexible ([#167](https://github.com/wnfs-wg/rs-wnfs/issues/167))

### Miscellaneous Chores

* **exports:** make re-exports more flexible ([#167](https://github.com/wnfs-wg/rs-wnfs/issues/167)) ([d7870bc](https://github.com/wnfs-wg/rs-wnfs/commit/d7870bc78660458fe9c5252c551a474dcdd045f2))
* release 0.1.16 ([#178](https://github.com/wnfs-wg/rs-wnfs/issues/178)) ([89e4d36](https://github.com/wnfs-wg/rs-wnfs/commit/89e4d36dc9b27ec1ab67db6fc214670efe768f32))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.15 to 0.1.16

## [0.1.15](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.14...wnfs-wasm-v0.1.15) (2023-02-16)


### Features

* **private:** shared private data ([#148](https://github.com/wnfs-wg/rs-wnfs/issues/148)) ([c210067](https://github.com/wnfs-wg/rs-wnfs/commit/c2100679acb1d16d98cb9a2e6aa6e9abc5a8eff2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.14 to 0.1.15

## [0.1.14](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.13...wnfs-wasm-v0.1.14) (2023-01-16)


### Features

* **api:** adds missing metadata functions for the private side ([#144](https://github.com/wnfs-wg/rs-wnfs/issues/144)) ([7588f69](https://github.com/wnfs-wg/rs-wnfs/commit/7588f69440bfec14b8959f6aecd35eb5f848dacc))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.13 to 0.1.14

## [0.1.12](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.11...wnfs-wasm-v0.1.12) (2023-01-11)


### Features

* Add as_file and is_file to PrivateNode (wasm) ([#136](https://github.com/wnfs-wg/rs-wnfs/issues/136)) ([f02658b](https://github.com/wnfs-wg/rs-wnfs/commit/f02658b07b84e391a0984046d4e2fc4b949056a1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.11 to 0.1.12

## [0.1.10](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-wasm-v0.1.9...wnfs-wasm-v0.1.10) (2022-12-06)


### Miscellaneous Chores

* release 0.1.10 ([#114](https://github.com/wnfs-wg/rs-wnfs/issues/114)) ([9cbc320](https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
* rename to wnfs-wasm and actions fix *maybe* ([#116](https://github.com/wnfs-wg/rs-wnfs/issues/116)) ([9ffad56](https://github.com/wnfs-wg/rs-wnfs/commit/9ffad56e6ab402c8636b13563a5bf516fb962037))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs bumped from 0.1.9 to 0.1.10
