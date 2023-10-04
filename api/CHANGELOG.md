# Changelog







## [2.1.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.1...repaint-server-v2.1.2) (2023-10-04)


### Bug Fixes

* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))

## [2.1.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.5...repaint-server-v2.1.1) (2023-10-04)


### Bug Fixes

* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))


### Miscellaneous Chores

* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))

## [2.0.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.6...repaint-server-v2.0.5) (2023-10-04)


### Features

* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))


### Bug Fixes

* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))


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

## [2.0.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.5...repaint-server-v2.0.5) (2023-10-03)


### Features

* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))


### Bug Fixes

* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))


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

## [2.0.4](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.3...repaint-server-v2.0.4) (2023-10-03)


### Bug Fixes

* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))

## [2.0.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.2...repaint-server-v2.0.3) (2023-10-03)


### Bug Fixes

* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))

## [2.0.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.1...repaint-server-v2.0.2) (2023-10-03)


### Bug Fixes

* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))

## [2.0.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.0...repaint-server-v2.0.1) (2023-10-03)


### Miscellaneous Chores

* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))

## [2.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.5.0...repaint-server-v2.0.0) (2023-10-03)


### Bug Fixes

* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))


### Miscellaneous Chores

* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))

## [1.5.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.4.0...repaint-server-v1.5.0) (2023-10-02)


### Features

* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))

## [1.4.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.6...repaint-server-v1.4.0) (2023-10-01)


### Features

* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))

## [1.3.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.6...repaint-server-v1.3.6) (2023-10-01)


### Features

* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))


### Bug Fixes

* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))


### Miscellaneous Chores

* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))
* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))
* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))
* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))
* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))

## [1.3.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.4...repaint-server-v1.3.5) (2023-10-01)


### Bug Fixes

* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))

## [1.3.4](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.3...repaint-server-v1.3.4) (2023-10-01)


### Bug Fixes

* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))

## [1.3.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.2...repaint-server-v1.3.3) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))

## [1.3.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.1...repaint-server-v1.3.2) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))

## [1.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.0...repaint-server-v1.3.1) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))

## [1.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.3...repaint-server-v1.3.0) (2023-09-30)


### Features

* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))

## [1.2.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.2...repaint-server-v1.2.3) (2023-09-29)


### Bug Fixes

* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))

## [1.2.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.1...repaint-server-v1.2.2) (2023-09-29)


### Bug Fixes

* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))

## [1.2.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.0...repaint-server-v1.2.1) (2023-09-29)


### Bug Fixes

* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))

## 1.2.0 (2023-09-29)


### Features

* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))


### Bug Fixes

* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))


### Miscellaneous Chores

* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
