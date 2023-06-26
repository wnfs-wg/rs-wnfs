# Changelog

## [0.1.22](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.21...wnfs-v0.1.22) (2023-06-23)


### Features

* make changes to BlockStore trait based on feedback ([#286](https://github.com/wnfs-wg/rs-wnfs/issues/286)) ([085242d](https://github.com/wnfs-wg/rs-wnfs/commit/085242d15aa48db17d77ed45e1c7717d13ed105f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.21 to 0.1.22
    * wnfs-hamt bumped from 0.1.21 to 0.1.22
    * wnfs-namefilter bumped from 0.1.21 to 0.1.22

## [0.1.21](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.20...wnfs-v0.1.21) (2023-05-22)


### ⚠ BREAKING CHANGES

* get_node should return null on missing path ([#253](https://github.com/wnfs-wg/rs-wnfs/issues/253))

### Features

* Add `PrivateDirectory::entires`, `PrivateFile::read_at` and make `PrivateFile::get_content_size_upper_bound` public ([#237](https://github.com/wnfs-wg/rs-wnfs/issues/237)) ([1572f43](https://github.com/wnfs-wg/rs-wnfs/commit/1572f432b6ae5366436cdefda7defd71c23b0ca7))


### Bug Fixes

* get_node should return null on missing path ([#253](https://github.com/wnfs-wg/rs-wnfs/issues/253)) ([5ed87fe](https://github.com/wnfs-wg/rs-wnfs/commit/5ed87fe6359a19abdea5f34dd0537fd5d62c98a8))
* propagate missing chunk error ([#252](https://github.com/wnfs-wg/rs-wnfs/issues/252)) ([4c16dcd](https://github.com/wnfs-wg/rs-wnfs/commit/4c16dcd4725c8b499a01184530e0e95ed8f4a9d5))


### Miscellaneous Chores

* release 0.1.21 ([#255](https://github.com/wnfs-wg/rs-wnfs/issues/255)) ([2be9f49](https://github.com/wnfs-wg/rs-wnfs/commit/2be9f4999d279acccfcda3b690d69dcbcdf8e60b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.20 to 0.1.21
    * wnfs-hamt bumped from 0.1.20 to 0.1.21
    * wnfs-namefilter bumped from 0.1.20 to 0.1.21

## [0.1.20](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.19...wnfs-v0.1.20) (2023-03-30)


### Features

* `open_file_mut` function for getting `&mut PrivateFile` references ([#218](https://github.com/wnfs-wg/rs-wnfs/issues/218)) ([f80dbb1](https://github.com/wnfs-wg/rs-wnfs/commit/f80dbb19cee471447145245b8c0285608a25ebcc))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.19 to 0.1.20
    * wnfs-hamt bumped from 0.1.19 to 0.1.20
    * wnfs-namefilter bumped from 0.1.19 to 0.1.20

## [0.1.19](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.18...wnfs-v0.1.19) (2023-03-23)


### Miscellaneous Chores

* release 0.1.19 ([1f37ec4](https://github.com/wnfs-wg/rs-wnfs/commit/1f37ec4d706b9bcb4305128451cc77063b4f211d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * wnfs-common bumped from 0.1.18 to 0.1.19
    * wnfs-hamt bumped from 0.1.18 to 0.1.19
    * wnfs-namefilter bumped from 0.1.18 to 0.1.19

## [0.1.18](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.17...wnfs-v0.1.18) (2023-03-23)


### Features

* Make log optional ([#189](https://github.com/wnfs-wg/rs-wnfs/issues/189)) ([12cd842](https://github.com/wnfs-wg/rs-wnfs/commit/12cd8428514d7c145b443a78e279dc468fa01a91))
* Redundant sha2 ([#191](https://github.com/wnfs-wg/rs-wnfs/issues/191)) ([231f4e9](https://github.com/wnfs-wg/rs-wnfs/commit/231f4e929378d7a02c9f7f8b095f1c2b1175ec2e))


### Bug Fixes

* `find_latest_share_counter` finds the last share count ([#197](https://github.com/wnfs-wg/rs-wnfs/issues/197)) ([69ffeec](https://github.com/wnfs-wg/rs-wnfs/commit/69ffeeca20cc3106e6d733e2d5adf5f87987630c))

## [0.1.17](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.16...wnfs-v0.1.17) (2023-02-24)


### Features

* **api:** add privateforest merge and diff bindings ([#181](https://github.com/wnfs-wg/rs-wnfs/issues/181)) ([231ece4](https://github.com/wnfs-wg/rs-wnfs/commit/231ece4309cab86d4682693e8e31f8ed99478a1f))
* PrivateLink abstraction ([#172](https://github.com/wnfs-wg/rs-wnfs/issues/172)) ([f04fa89](https://github.com/wnfs-wg/rs-wnfs/commit/f04fa89738e19a095d177e18b35d7e153c380833))
* Remove `base_history_on` and auto-track history instead ([#174](https://github.com/wnfs-wg/rs-wnfs/issues/174)) ([806bbb9](https://github.com/wnfs-wg/rs-wnfs/commit/806bbb93b1f03983165375005e14a9b63ebe67c2))

## [0.1.16](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.15...wnfs-v0.1.16) (2023-02-22)


### ⚠ BREAKING CHANGES

* **exports:** make re-exports more flexible ([#167](https://github.com/wnfs-wg/rs-wnfs/issues/167))

### Features

* Streaming write for PrivateFile ([#163](https://github.com/wnfs-wg/rs-wnfs/issues/163)) ([1bfe89b](https://github.com/wnfs-wg/rs-wnfs/commit/1bfe89bcaabdf679a5338a2c9aa97b76deb00b03))


### Miscellaneous Chores

* **exports:** make re-exports more flexible ([#167](https://github.com/wnfs-wg/rs-wnfs/issues/167)) ([d7870bc](https://github.com/wnfs-wg/rs-wnfs/commit/d7870bc78660458fe9c5252c551a474dcdd045f2))
* release 0.1.16 ([#178](https://github.com/wnfs-wg/rs-wnfs/issues/178)) ([89e4d36](https://github.com/wnfs-wg/rs-wnfs/commit/89e4d36dc9b27ec1ab67db6fc214670efe768f32))

## [0.1.15](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.14...wnfs-v0.1.15) (2023-02-16)


### Features

* **private:** shared private data ([#148](https://github.com/wnfs-wg/rs-wnfs/issues/148)) ([c210067](https://github.com/wnfs-wg/rs-wnfs/commit/c2100679acb1d16d98cb9a2e6aa6e9abc5a8eff2))

## [0.1.14](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.13...wnfs-v0.1.14) (2023-01-16)


### Features

* **api:** adds missing metadata functions for the private side ([#144](https://github.com/wnfs-wg/rs-wnfs/issues/144)) ([7588f69](https://github.com/wnfs-wg/rs-wnfs/commit/7588f69440bfec14b8959f6aecd35eb5f848dacc))

## [0.1.13](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.12...wnfs-v0.1.13) (2023-01-12)


### Features

* **api:** self lookup & store at construction ([#138](https://github.com/wnfs-wg/rs-wnfs/issues/138)) ([228d326](https://github.com/wnfs-wg/rs-wnfs/commit/228d326291926c7e4b593ef66ebb089ce220dacb))

## [0.1.12](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.11...wnfs-v0.1.12) (2023-01-11)


### Features

* private backpointer ([#90](https://github.com/wnfs-wg/rs-wnfs/issues/90)) ([e38d039](https://github.com/wnfs-wg/rs-wnfs/commit/e38d039d3886f8590e00c7f87a530ca207f8a713))

## [0.1.11](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.10...wnfs-v0.1.11) (2023-01-06)


### Features

* **hamt:** diff and merge implementation ([#94](https://github.com/wnfs-wg/rs-wnfs/issues/94)) ([883b3ab](https://github.com/wnfs-wg/rs-wnfs/commit/883b3ab7f9c0ec4c086e83afe7f0510c448f6bbb))

## [0.1.10](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.9...wnfs-v0.1.10) (2022-12-06)


### Miscellaneous Chores

* release 0.1.10 ([#114](https://github.com/wnfs-wg/rs-wnfs/issues/114)) ([9cbc320](https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
* rename to wnfs-wasm and actions fix *maybe* ([#116](https://github.com/wnfs-wg/rs-wnfs/issues/116)) ([9ffad56](https://github.com/wnfs-wg/rs-wnfs/commit/9ffad56e6ab402c8636b13563a5bf516fb962037))

## [0.1.10](https://github.com/wnfs-wg/rs-wnfs/compare/wnfs-v0.1.9...wnfs-v0.1.10) (2022-12-06)


### Miscellaneous Chores

* release 0.1.10 ([#114](https://github.com/wnfs-wg/rs-wnfs/issues/114)) ([9cbc320](https://github.com/wnfs-wg/rs-wnfs/commit/9cbc32076d80a5b7d3138ea891180c689411123f))
