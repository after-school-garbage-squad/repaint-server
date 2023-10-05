# Changelog

## [2.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.5...repaint-server-v2.2.0) (2023-10-05)


### Features

* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))

## [2.1.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.4...repaint-server-v2.1.5) (2023-10-04)


### Bug Fixes

* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))

## [2.1.4](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.3...repaint-server-v2.1.4) (2023-10-04)


### Bug Fixes

* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))

## [2.1.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.3...repaint-server-v2.1.3) (2023-10-04)


### Features

* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))


### Bug Fixes

* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


### Miscellaneous Chores

* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))
* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))
* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))
* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))
* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))
* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))
* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))

## [2.1.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.2...repaint-server-v2.1.3) (2023-10-04)


### Bug Fixes

* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))

## [2.1.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.1...repaint-server-v2.1.2) (2023-10-04)


### Bug Fixes

* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))

## [2.1.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.6...repaint-server-v2.1.1) (2023-10-04)


### Features

* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))


### Bug Fixes

* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))


### Miscellaneous Chores

* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))
* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))
* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))
* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))
* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))
* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))
* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))

## [2.0.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.5...repaint-server-v2.0.6) (2023-10-04)


### Bug Fixes

* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))

## [2.0.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.2...repaint-server-v2.0.5) (2023-10-03)


### Miscellaneous Chores

* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))

## [2.0.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.1...repaint-server-v2.0.2) (2023-10-03)


### Bug Fixes

* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))

## [2.0.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.0...repaint-server-v2.0.1) (2023-10-03)


### Miscellaneous Chores

* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))

## [2.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.6...repaint-server-v2.0.0) (2023-10-03)


### Miscellaneous Chores

* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))

## [1.3.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.3...repaint-server-v1.3.6) (2023-10-01)


### Miscellaneous Chores

* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))

## [1.3.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.2...repaint-server-v1.3.3) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))

## [1.3.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.1...repaint-server-v1.3.2) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))

## [1.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.0...repaint-server-v1.3.1) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))

## [1.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.0...repaint-server-v1.3.0) (2023-09-30)


### Features

* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))

## 1.2.0 (2023-09-29)


### Features

* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))


### Bug Fixes

* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))


### Miscellaneous Chores

* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
