# Changelog













































## [3.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.3...repaint-server-v3.3.0) (2023-10-12)


### Features

* add filtering with binary head ([#314](https://github.com/after-school-garbage-squad/repaint-server/issues/314)) ([ef67900](https://github.com/after-school-garbage-squad/repaint-server/commit/ef679009d38149c5feecaae9cc77be870fc56c00))
* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* refactor visitor_images logic ([#324](https://github.com/after-school-garbage-squad/repaint-server/issues/324)) ([7bcd042](https://github.com/after-school-garbage-squad/repaint-server/commit/7bcd0425e3f24b359feac592bd8e4cd78ee46f37))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))
* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))
* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


### Miscellaneous Chores

* 2.12.0 ([27e2b30](https://github.com/after-school-garbage-squad/repaint-server/commit/27e2b301c782efd59201c55ac135e84f19432bfa))
* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))
* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))
* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))
* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))
* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
* release ([f3c39c7](https://github.com/after-school-garbage-squad/repaint-server/commit/f3c39c76f8b938a2556341d94f0ebb2d26d2d2ad))
* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))
* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))
* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.16.0 ([3d97c7e](https://github.com/after-school-garbage-squad/repaint-server/commit/3d97c7e7b9b02e1eba25c442627342886c6a3af3))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))
* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))
* release 3.2.0 ([1240c24](https://github.com/after-school-garbage-squad/repaint-server/commit/1240c244f5fe03baa2d4cbcaddbc5b0382191a21))
* release 3.3.0 ([69fd163](https://github.com/after-school-garbage-squad/repaint-server/commit/69fd163e94457f29a8938a32a572d1291d9e4347))

## [3.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.3.0) (2023-10-12)


### Features

* add filtering with binary head ([#314](https://github.com/after-school-garbage-squad/repaint-server/issues/314)) ([ef67900](https://github.com/after-school-garbage-squad/repaint-server/commit/ef679009d38149c5feecaae9cc77be870fc56c00))
* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* refactor visitor_images logic ([#324](https://github.com/after-school-garbage-squad/repaint-server/issues/324)) ([7bcd042](https://github.com/after-school-garbage-squad/repaint-server/commit/7bcd0425e3f24b359feac592bd8e4cd78ee46f37))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))
* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))
* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


### Miscellaneous Chores

* 2.12.0 ([27e2b30](https://github.com/after-school-garbage-squad/repaint-server/commit/27e2b301c782efd59201c55ac135e84f19432bfa))
* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))
* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))
* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))
* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))
* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
* release ([f3c39c7](https://github.com/after-school-garbage-squad/repaint-server/commit/f3c39c76f8b938a2556341d94f0ebb2d26d2d2ad))
* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))
* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))
* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.16.0 ([3d97c7e](https://github.com/after-school-garbage-squad/repaint-server/commit/3d97c7e7b9b02e1eba25c442627342886c6a3af3))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))
* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))
* release 3.2.0 ([1240c24](https://github.com/after-school-garbage-squad/repaint-server/commit/1240c244f5fe03baa2d4cbcaddbc5b0382191a21))
* release 3.3.0 ([69fd163](https://github.com/after-school-garbage-squad/repaint-server/commit/69fd163e94457f29a8938a32a572d1291d9e4347))

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.1.2...repaint-server-v3.2.0) (2023-10-12)


### Features

* add filtering with binary head ([#314](https://github.com/after-school-garbage-squad/repaint-server/issues/314)) ([ef67900](https://github.com/after-school-garbage-squad/repaint-server/commit/ef679009d38149c5feecaae9cc77be870fc56c00))
* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* refactor visitor_images logic ([#324](https://github.com/after-school-garbage-squad/repaint-server/issues/324)) ([7bcd042](https://github.com/after-school-garbage-squad/repaint-server/commit/7bcd0425e3f24b359feac592bd8e4cd78ee46f37))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))
* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))
* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


### Miscellaneous Chores

* 2.12.0 ([27e2b30](https://github.com/after-school-garbage-squad/repaint-server/commit/27e2b301c782efd59201c55ac135e84f19432bfa))
* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))
* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))
* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))
* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))
* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
* release ([f3c39c7](https://github.com/after-school-garbage-squad/repaint-server/commit/f3c39c76f8b938a2556341d94f0ebb2d26d2d2ad))
* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))
* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))
* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.16.0 ([3d97c7e](https://github.com/after-school-garbage-squad/repaint-server/commit/3d97c7e7b9b02e1eba25c442627342886c6a3af3))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))
* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))
* release 3.2.0 ([1240c24](https://github.com/after-school-garbage-squad/repaint-server/commit/1240c244f5fe03baa2d4cbcaddbc5b0382191a21))

## [3.1.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.1.0...repaint-server-v3.1.1) (2023-10-11)


### Bug Fixes

* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))

## [3.1.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.0.1...repaint-server-v3.1.0) (2023-10-11)


### Features

* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))


### Bug Fixes

* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))

## [3.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.17.0...repaint-server-v3.0.0) (2023-10-11)


### Features

* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))


### Miscellaneous Chores

* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))

## [2.17.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.16.0...repaint-server-v2.17.0) (2023-10-11)


### Features

* refactor visitor_images logic ([#324](https://github.com/after-school-garbage-squad/repaint-server/issues/324)) ([7bcd042](https://github.com/after-school-garbage-squad/repaint-server/commit/7bcd0425e3f24b359feac592bd8e4cd78ee46f37))

## [2.16.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.14.0...repaint-server-v2.16.0) (2023-10-10)


### Features

* add filtering with binary head ([#314](https://github.com/after-school-garbage-squad/repaint-server/issues/314)) ([ef67900](https://github.com/after-school-garbage-squad/repaint-server/commit/ef679009d38149c5feecaae9cc77be870fc56c00))


### Bug Fixes

* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))


### Miscellaneous Chores

* release 2.16.0 ([3d97c7e](https://github.com/after-school-garbage-squad/repaint-server/commit/3d97c7e7b9b02e1eba25c442627342886c6a3af3))

## [2.14.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.14.3...repaint-server-v2.14.0) (2023-10-10)


### Features

* add filtering with binary head ([#314](https://github.com/after-school-garbage-squad/repaint-server/issues/314)) ([ef67900](https://github.com/after-school-garbage-squad/repaint-server/commit/ef679009d38149c5feecaae9cc77be870fc56c00))
* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


### Miscellaneous Chores

* 2.12.0 ([27e2b30](https://github.com/after-school-garbage-squad/repaint-server/commit/27e2b301c782efd59201c55ac135e84f19432bfa))
* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))
* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))
* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))
* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))
* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
* release ([f3c39c7](https://github.com/after-school-garbage-squad/repaint-server/commit/f3c39c76f8b938a2556341d94f0ebb2d26d2d2ad))
* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))
* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))
* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.14.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.14.0...repaint-server-v2.14.1) (2023-10-10)


### Bug Fixes

* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))

## [2.14.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.12.0...repaint-server-v2.14.0) (2023-10-09)


### Miscellaneous Chores

* release ([f3c39c7](https://github.com/after-school-garbage-squad/repaint-server/commit/f3c39c76f8b938a2556341d94f0ebb2d26d2d2ad))

## [2.12.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.12.1...repaint-server-v2.12.0) (2023-10-09)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


### Miscellaneous Chores

* 2.12.0 ([27e2b30](https://github.com/after-school-garbage-squad/repaint-server/commit/27e2b301c782efd59201c55ac135e84f19432bfa))
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
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.12.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.10.0...repaint-server-v2.12.0) (2023-10-09)


### Bug Fixes

* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))


### Miscellaneous Chores

* 2.12.0 ([27e2b30](https://github.com/after-school-garbage-squad/repaint-server/commit/27e2b301c782efd59201c55ac135e84f19432bfa))

## [2.10.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.10.2...repaint-server-v2.10.0) (2023-10-09)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


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
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.10.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.10.0...repaint-server-v2.10.1) (2023-10-08)


### Bug Fixes

* add ext filter to uploading default image ([#289](https://github.com/after-school-garbage-squad/repaint-server/issues/289)) ([c774777](https://github.com/after-school-garbage-squad/repaint-server/commit/c774777be6166550cac11974bc2e5e5f917af24a))

## [2.10.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.1...repaint-server-v2.10.0) (2023-10-08)


### Features

* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Miscellaneous Chores

* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))

## [2.8.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.2...repaint-server-v2.8.1) (2023-10-08)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


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
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.8.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.0...repaint-server-v2.8.1) (2023-10-06)


### Miscellaneous Chores

* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.8.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.7.1...repaint-server-v2.8.0) (2023-10-06)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))
* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))
* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))


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
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))

## [2.7.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.6.0...repaint-server-v2.7.0) (2023-10-06)


### Features

* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))


### Bug Fixes

* fix unneeded auth layer and feed body limit ([#268](https://github.com/after-school-garbage-squad/repaint-server/issues/268)) ([8b0b868](https://github.com/after-school-garbage-squad/repaint-server/commit/8b0b868625f3fcc1a4414576575d0dbdb74f7fba))

## [2.6.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.5.1...repaint-server-v2.6.0) (2023-10-05)


### Features

* update body limit to 128MB ([#261](https://github.com/after-school-garbage-squad/repaint-server/issues/261)) ([c4e3d24](https://github.com/after-school-garbage-squad/repaint-server/commit/c4e3d24b57d05fe1aa31c8141ca0f4717a69b5c0))

## [2.5.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.5.0...repaint-server-v2.5.1) (2023-10-05)


### Bug Fixes

* throw unauthorized status when token was expired ([#259](https://github.com/after-school-garbage-squad/repaint-server/issues/259)) ([46df0b5](https://github.com/after-school-garbage-squad/repaint-server/commit/46df0b5bd2fe7a205bbbe63643fe5ef4af983715))

## [2.5.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.4.1...repaint-server-v2.5.0) (2023-10-05)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
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
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))

## [2.4.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.3.1...repaint-server-v2.4.0) (2023-10-05)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))

## [2.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.3.1...repaint-server-v2.3.1) (2023-10-05)


### Features

* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl CORS ([#200](https://github.com/after-school-garbage-squad/repaint-server/issues/200)) ([e734ba0](https://github.com/after-school-garbage-squad/repaint-server/commit/e734ba06a567bd369bb25c041b7a3de9568d7b4b))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl prometheus endpoint ([#196](https://github.com/after-school-garbage-squad/repaint-server/issues/196)) ([782b152](https://github.com/after-school-garbage-squad/repaint-server/commit/782b152eb4bbc191d7f213372bc3de7e58974320))
* impl sentry ([#154](https://github.com/after-school-garbage-squad/repaint-server/issues/154)) ([47a7e9c](https://github.com/after-school-garbage-squad/repaint-server/commit/47a7e9ce5d44297599662438ccd67c32009fda63))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* switch license to Apache-2.0 ([#160](https://github.com/after-school-garbage-squad/repaint-server/issues/160)) ([5fea742](https://github.com/after-school-garbage-squad/repaint-server/commit/5fea7429a541948f455203551ade0f23ba8ab83f))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix credential.json import ([#170](https://github.com/after-school-garbage-squad/repaint-server/issues/170)) ([4299b73](https://github.com/after-school-garbage-squad/repaint-server/commit/4299b7372a956253145d9793e594e84034fdb5eb))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix file path to license.html ([#192](https://github.com/after-school-garbage-squad/repaint-server/issues/192)) ([52689b7](https://github.com/after-school-garbage-squad/repaint-server/commit/52689b70afedb9db25c10983bb451123f5142825))
* fix initialize sentry instance ([#190](https://github.com/after-school-garbage-squad/repaint-server/issues/190)) ([4a7ec68](https://github.com/after-school-garbage-squad/repaint-server/commit/4a7ec68894f0e2c9b0cad3f2f0a495a0525ee8a0))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
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
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))

## [2.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.9...repaint-server-v2.3.0) (2023-10-05)


### Features

* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))

## [2.2.8](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.7...repaint-server-v2.2.8) (2023-10-05)


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))

## [2.2.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.6...repaint-server-v2.2.6) (2023-10-05)


### Features

* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
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
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove sentry-trace crate and impl catch panic layer to Service ([#188](https://github.com/after-school-garbage-squad/repaint-server/issues/188)) ([14c1701](https://github.com/after-school-garbage-squad/repaint-server/commit/14c1701e303eda8717264046d8e130adcb8b5ce7))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
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
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))
* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))

## [2.2.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.0...repaint-server-v2.2.5) (2023-10-04)


### Features

* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))


### Bug Fixes

* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))


### Miscellaneous Chores

* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))
* release 2.2.5 ([8c42acf](https://github.com/after-school-garbage-squad/repaint-server/commit/8c42acfd255fd44c3ed82600d82b8aefcfe6aecb))

## [2.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.5...repaint-server-v2.2.0) (2023-10-04)


### Features

* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))

## [2.1.4](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.3...repaint-server-v2.1.4) (2023-10-04)


### Bug Fixes

* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))

## [2.1.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.1...repaint-server-v2.1.3) (2023-10-04)


### Bug Fixes

* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))


### Miscellaneous Chores

* release 2.1.3 ([66708c6](https://github.com/after-school-garbage-squad/repaint-server/commit/66708c6c31e3f803695962c928fa3e1556c9a919))

## [2.1.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.3...repaint-server-v2.1.1) (2023-10-04)


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
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
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
* release 2.1.1 ([2120122](https://github.com/after-school-garbage-squad/repaint-server/commit/212012202f82ce98a3338c9c566125b57b8dd717))

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
