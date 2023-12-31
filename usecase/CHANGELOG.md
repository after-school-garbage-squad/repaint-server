# Changelog

## [3.6.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.5.0...repaint-server-v3.6.0) (2023-10-19)


### Features

* add reset spot palettes ([#376](https://github.com/after-school-garbage-squad/repaint-server/issues/376)) ([1e93108](https://github.com/after-school-garbage-squad/repaint-server/commit/1e93108f0714aab1a7dccb53e58194dea5611372))
* add uard for pick before scan ([#398](https://github.com/after-school-garbage-squad/repaint-server/issues/398)) ([ccce7a6](https://github.com/after-school-garbage-squad/repaint-server/commit/ccce7a65db0dd76b2aca09a6c00760457ea6c4ed))
* impl check palettes are completed endpoint ([#370](https://github.com/after-school-garbage-squad/repaint-server/issues/370)) ([d188437](https://github.com/after-school-garbage-squad/repaint-server/commit/d188437ddf19739ec2322c62da56960f97bae9b1))
* impl sending pick notification ([#379](https://github.com/after-school-garbage-squad/repaint-server/issues/379)) ([0dea0f7](https://github.com/after-school-garbage-squad/repaint-server/commit/0dea0f7e58d52eab82de6d491a4ade01c019576e))
* impl transaction to all usecase ([#393](https://github.com/after-school-garbage-squad/repaint-server/issues/393)) ([f5b748c](https://github.com/after-school-garbage-squad/repaint-server/commit/f5b748cb73b1e51d1952ad58c6873da4f3b73d4a))
* update palette structure ([#368](https://github.com/after-school-garbage-squad/repaint-server/issues/368)) ([0ecde29](https://github.com/after-school-garbage-squad/repaint-server/commit/0ecde297ff8aa35682a9245d35c6bb8b817c0e29))
* update visitors in from logic ([#377](https://github.com/after-school-garbage-squad/repaint-server/issues/377)) ([fa71e3b](https://github.com/after-school-garbage-squad/repaint-server/commit/fa71e3bfb40c604746213a130242f2ffccf8e648))


### Bug Fixes

* add auto fill flow to drop logic ([#387](https://github.com/after-school-garbage-squad/repaint-server/issues/387)) ([4093ed9](https://github.com/after-school-garbage-squad/repaint-server/commit/4093ed951220937e9c74f6a3cf208cf4c00774d4))
* **deps:** update rust crate async-trait to 0.1.74 ([#382](https://github.com/after-school-garbage-squad/repaint-server/issues/382)) ([e30f502](https://github.com/after-school-garbage-squad/repaint-server/commit/e30f50216ce72d836180146895ab72d42253e641))
* **deps:** update rust crate serde to 1.0.189 ([#371](https://github.com/after-school-garbage-squad/repaint-server/issues/371)) ([a4b6a23](https://github.com/after-school-garbage-squad/repaint-server/commit/a4b6a2310f37c10094af8b51ce56a8af48b53c26))
* fix core drop logic ([#374](https://github.com/after-school-garbage-squad/repaint-server/issues/374)) ([f22fe41](https://github.com/after-school-garbage-squad/repaint-server/commit/f22fe416410b22c19c61fcafc2b34bc172346e71))
* fix logic of is_pick ([#380](https://github.com/after-school-garbage-squad/repaint-server/issues/380)) ([cc033f6](https://github.com/after-school-garbage-squad/repaint-server/commit/cc033f61509d06ba7996418e7cbb12672d0d9a84))
* remove join_all for BUG ([#375](https://github.com/after-school-garbage-squad/repaint-server/issues/375)) ([6829e77](https://github.com/after-school-garbage-squad/repaint-server/commit/6829e77cbf829398c32e61a90dec93e2437e08b8))
* remove missed updated setter ([#390](https://github.com/after-school-garbage-squad/repaint-server/issues/390)) ([775becb](https://github.com/after-school-garbage-squad/repaint-server/commit/775becb90f1d07147a8a17e27818902263e19442))

## [3.5.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.5.0...repaint-server-v3.5.0) (2023-10-12)


### ⚠ BREAKING CHANGES

* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334))

### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl setting last_picked_at and last_droped_at in repository and impls ([#359](https://github.com/after-school-garbage-squad/repaint-server/issues/359)) ([2796c4c](https://github.com/after-school-garbage-squad/repaint-server/commit/2796c4c7588af3b5cbbb24147af52aee194a775d))
* impl traffic queue ([#338](https://github.com/after-school-garbage-squad/repaint-server/issues/338)) ([439313f](https://github.com/after-school-garbage-squad/repaint-server/commit/439313f667a9122ec4c1522e4c31910e794c344d))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* **refactor:** managing master palette on register spot ([#343](https://github.com/after-school-garbage-squad/repaint-server/issues/343)) ([33576d8](https://github.com/after-school-garbage-squad/repaint-server/commit/33576d8bc5f56ebcfb4d0a26c932080825bacb4f))
* **refactor:** remove fcm wrapper and use pubsub for notification ([#364](https://github.com/after-school-garbage-squad/repaint-server/issues/364)) ([bb49637](https://github.com/after-school-garbage-squad/repaint-server/commit/bb4963779fdf313b0e529defa06ad1ce9b16761e))
* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* change gte to lte ([#347](https://github.com/after-school-garbage-squad/repaint-server/issues/347)) ([74ce864](https://github.com/after-school-garbage-squad/repaint-server/commit/74ce864b1e25ac9b5e6fc7f858e479c39493a693))
* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* DESTROY DROP LOGIC ([#322](https://github.com/after-school-garbage-squad/repaint-server/issues/322)) ([e598344](https://github.com/after-school-garbage-squad/repaint-server/commit/e598344f21e9dbb80c5d80f1de1cd5bf01aa2516))
* fix drop logic ([#353](https://github.com/after-school-garbage-squad/repaint-server/issues/353)) ([c5ab7fd](https://github.com/after-school-garbage-squad/repaint-server/commit/c5ab7fd289feec4bcada55c28f81b9ded7669944))
* fix drop own palette ([#360](https://github.com/after-school-garbage-squad/repaint-server/issues/360)) ([68c2c18](https://github.com/after-school-garbage-squad/repaint-server/commit/68c2c18d9e970661d44ef4575ab649d72613b6e3))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))
* fix missed duration ([#344](https://github.com/after-school-garbage-squad/repaint-server/issues/344)) ([faf323f](https://github.com/after-school-garbage-squad/repaint-server/commit/faf323fdcc0ce42d3b838eb5e3e36083bdbea65c))
* fix pick core logic ([#362](https://github.com/after-school-garbage-squad/repaint-server/issues/362)) ([5f47b5d](https://github.com/after-school-garbage-squad/repaint-server/commit/5f47b5d6ed51e8ba6e024587f3bc35352df923c8))
* fix registering spot ([#349](https://github.com/after-school-garbage-squad/repaint-server/issues/349)) ([c631307](https://github.com/after-school-garbage-squad/repaint-server/commit/c631307d8820cd5bf1864ffa62a66aa6e2b883e6))
* fix spot-id typo ([#311](https://github.com/after-school-garbage-squad/repaint-server/issues/311)) ([5f78d87](https://github.com/after-school-garbage-squad/repaint-server/commit/5f78d87a164562bf597abfcde86b446f1a6f3384))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* **refactor:** fix hw-id logic ([#316](https://github.com/after-school-garbage-squad/repaint-server/issues/316)) ([2f22b36](https://github.com/after-school-garbage-squad/repaint-server/commit/2f22b368977821e3659ce257aa99aaaa14f06e6b))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334)) ([be58daf](https://github.com/after-school-garbage-squad/repaint-server/commit/be58dafcb59ad307b0ddeafa11c6ae6cffa8487c))
* remove merge publisher from set-current usecase ([#341](https://github.com/after-school-garbage-squad/repaint-server/issues/341)) ([0730667](https://github.com/after-school-garbage-squad/repaint-server/commit/07306679cb8feb4d1d66bca24966b216929c8b21))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* set update flag ([#328](https://github.com/after-school-garbage-squad/repaint-server/issues/328)) ([6c7b5f7](https://github.com/after-school-garbage-squad/repaint-server/commit/6c7b5f77d78fb6a5e4ee4c90cb3b0bbbd8ea7c9d))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))
* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))
* release 3.2.0 ([1240c24](https://github.com/after-school-garbage-squad/repaint-server/commit/1240c244f5fe03baa2d4cbcaddbc5b0382191a21))
* release 3.3.0 ([69fd163](https://github.com/after-school-garbage-squad/repaint-server/commit/69fd163e94457f29a8938a32a572d1291d9e4347))
* release 3.5.0 ([fdef4c3](https://github.com/after-school-garbage-squad/repaint-server/commit/fdef4c34599db0da4f6e79e72713b322afa36b3f))

## [3.5.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.4.0...repaint-server-v3.5.0) (2023-10-12)


### Features

* **refactor:** remove fcm wrapper and use pubsub for notification ([#364](https://github.com/after-school-garbage-squad/repaint-server/issues/364)) ([bb49637](https://github.com/after-school-garbage-squad/repaint-server/commit/bb4963779fdf313b0e529defa06ad1ce9b16761e))

## [3.4.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.2...repaint-server-v3.4.0) (2023-10-12)


### Features

* impl setting last_picked_at and last_droped_at in repository and impls ([#359](https://github.com/after-school-garbage-squad/repaint-server/issues/359)) ([2796c4c](https://github.com/after-school-garbage-squad/repaint-server/commit/2796c4c7588af3b5cbbb24147af52aee194a775d))


### Bug Fixes

* fix drop logic ([#353](https://github.com/after-school-garbage-squad/repaint-server/issues/353)) ([c5ab7fd](https://github.com/after-school-garbage-squad/repaint-server/commit/c5ab7fd289feec4bcada55c28f81b9ded7669944))
* fix drop own palette ([#360](https://github.com/after-school-garbage-squad/repaint-server/issues/360)) ([68c2c18](https://github.com/after-school-garbage-squad/repaint-server/commit/68c2c18d9e970661d44ef4575ab649d72613b6e3))
* fix pick core logic ([#362](https://github.com/after-school-garbage-squad/repaint-server/issues/362)) ([5f47b5d](https://github.com/after-school-garbage-squad/repaint-server/commit/5f47b5d6ed51e8ba6e024587f3bc35352df923c8))

## [3.3.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.1...repaint-server-v3.3.2) (2023-10-12)


### Bug Fixes

* fix registering spot ([#349](https://github.com/after-school-garbage-squad/repaint-server/issues/349)) ([c631307](https://github.com/after-school-garbage-squad/repaint-server/commit/c631307d8820cd5bf1864ffa62a66aa6e2b883e6))

## [3.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.0...repaint-server-v3.3.1) (2023-10-12)


### Bug Fixes

* change gte to lte ([#347](https://github.com/after-school-garbage-squad/repaint-server/issues/347)) ([74ce864](https://github.com/after-school-garbage-squad/repaint-server/commit/74ce864b1e25ac9b5e6fc7f858e479c39493a693))

## [3.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.0...repaint-server-v3.3.0) (2023-10-12)


### Features

* **refactor:** managing master palette on register spot ([#343](https://github.com/after-school-garbage-squad/repaint-server/issues/343)) ([33576d8](https://github.com/after-school-garbage-squad/repaint-server/commit/33576d8bc5f56ebcfb4d0a26c932080825bacb4f))


### Bug Fixes

* fix missed duration ([#344](https://github.com/after-school-garbage-squad/repaint-server/issues/344)) ([faf323f](https://github.com/after-school-garbage-squad/repaint-server/commit/faf323fdcc0ce42d3b838eb5e3e36083bdbea65c))
* remove merge publisher from set-current usecase ([#341](https://github.com/after-school-garbage-squad/repaint-server/issues/341)) ([0730667](https://github.com/after-school-garbage-squad/repaint-server/commit/07306679cb8feb4d1d66bca24966b216929c8b21))


### Miscellaneous Chores

* release 3.3.0 ([69fd163](https://github.com/after-school-garbage-squad/repaint-server/commit/69fd163e94457f29a8938a32a572d1291d9e4347))

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.0...repaint-server-v3.2.0) (2023-10-12)


### ⚠ BREAKING CHANGES

* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334))

### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl traffic queue ([#338](https://github.com/after-school-garbage-squad/repaint-server/issues/338)) ([439313f](https://github.com/after-school-garbage-squad/repaint-server/commit/439313f667a9122ec4c1522e4c31910e794c344d))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* **refactor:** managing master palette on register spot ([#343](https://github.com/after-school-garbage-squad/repaint-server/issues/343)) ([33576d8](https://github.com/after-school-garbage-squad/repaint-server/commit/33576d8bc5f56ebcfb4d0a26c932080825bacb4f))
* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* DESTROY DROP LOGIC ([#322](https://github.com/after-school-garbage-squad/repaint-server/issues/322)) ([e598344](https://github.com/after-school-garbage-squad/repaint-server/commit/e598344f21e9dbb80c5d80f1de1cd5bf01aa2516))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))
* fix missed duration ([#344](https://github.com/after-school-garbage-squad/repaint-server/issues/344)) ([faf323f](https://github.com/after-school-garbage-squad/repaint-server/commit/faf323fdcc0ce42d3b838eb5e3e36083bdbea65c))
* fix spot-id typo ([#311](https://github.com/after-school-garbage-squad/repaint-server/issues/311)) ([5f78d87](https://github.com/after-school-garbage-squad/repaint-server/commit/5f78d87a164562bf597abfcde86b446f1a6f3384))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* **refactor:** fix hw-id logic ([#316](https://github.com/after-school-garbage-squad/repaint-server/issues/316)) ([2f22b36](https://github.com/after-school-garbage-squad/repaint-server/commit/2f22b368977821e3659ce257aa99aaaa14f06e6b))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334)) ([be58daf](https://github.com/after-school-garbage-squad/repaint-server/commit/be58dafcb59ad307b0ddeafa11c6ae6cffa8487c))
* remove merge publisher from set-current usecase ([#341](https://github.com/after-school-garbage-squad/repaint-server/issues/341)) ([0730667](https://github.com/after-school-garbage-squad/repaint-server/commit/07306679cb8feb4d1d66bca24966b216929c8b21))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* set update flag ([#328](https://github.com/after-school-garbage-squad/repaint-server/issues/328)) ([6c7b5f7](https://github.com/after-school-garbage-squad/repaint-server/commit/6c7b5f77d78fb6a5e4ee4c90cb3b0bbbd8ea7c9d))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))
* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))
* release 3.2.0 ([1240c24](https://github.com/after-school-garbage-squad/repaint-server/commit/1240c244f5fe03baa2d4cbcaddbc5b0382191a21))

## [3.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.0...repaint-server-v3.3.0) (2023-10-12)


### Features

* **refactor:** managing master palette on register spot ([#343](https://github.com/after-school-garbage-squad/repaint-server/issues/343)) ([33576d8](https://github.com/after-school-garbage-squad/repaint-server/commit/33576d8bc5f56ebcfb4d0a26c932080825bacb4f))


### Bug Fixes

* remove merge publisher from set-current usecase ([#341](https://github.com/after-school-garbage-squad/repaint-server/issues/341)) ([0730667](https://github.com/after-school-garbage-squad/repaint-server/commit/07306679cb8feb4d1d66bca24966b216929c8b21))

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.0.0...repaint-server-v3.2.0) (2023-10-11)


### ⚠ BREAKING CHANGES

* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334))

### Features

* impl traffic queue ([#338](https://github.com/after-school-garbage-squad/repaint-server/issues/338)) ([439313f](https://github.com/after-school-garbage-squad/repaint-server/commit/439313f667a9122ec4c1522e4c31910e794c344d))
* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))


### Bug Fixes

* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334)) ([be58daf](https://github.com/after-school-garbage-squad/repaint-server/commit/be58dafcb59ad307b0ddeafa11c6ae6cffa8487c))


### Miscellaneous Chores

* release 3.2.0 ([1240c24](https://github.com/after-school-garbage-squad/repaint-server/commit/1240c244f5fe03baa2d4cbcaddbc5b0382191a21))

## [3.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v4.0.0...repaint-server-v3.0.0) (2023-10-11)


### ⚠ BREAKING CHANGES

* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334))

### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl traffic queue ([#338](https://github.com/after-school-garbage-squad/repaint-server/issues/338)) ([439313f](https://github.com/after-school-garbage-squad/repaint-server/commit/439313f667a9122ec4c1522e4c31910e794c344d))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* DESTROY DROP LOGIC ([#322](https://github.com/after-school-garbage-squad/repaint-server/issues/322)) ([e598344](https://github.com/after-school-garbage-squad/repaint-server/commit/e598344f21e9dbb80c5d80f1de1cd5bf01aa2516))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))
* fix spot-id typo ([#311](https://github.com/after-school-garbage-squad/repaint-server/issues/311)) ([5f78d87](https://github.com/after-school-garbage-squad/repaint-server/commit/5f78d87a164562bf597abfcde86b446f1a6f3384))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* **refactor:** fix hw-id logic ([#316](https://github.com/after-school-garbage-squad/repaint-server/issues/316)) ([2f22b36](https://github.com/after-school-garbage-squad/repaint-server/commit/2f22b368977821e3659ce257aa99aaaa14f06e6b))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334)) ([be58daf](https://github.com/after-school-garbage-squad/repaint-server/commit/be58dafcb59ad307b0ddeafa11c6ae6cffa8487c))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* set update flag ([#328](https://github.com/after-school-garbage-squad/repaint-server/issues/328)) ([6c7b5f7](https://github.com/after-school-garbage-squad/repaint-server/commit/6c7b5f77d78fb6a5e4ee4c90cb3b0bbbd8ea7c9d))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))
* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))

## [4.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.0.0...repaint-server-v4.0.0) (2023-10-11)


### ⚠ BREAKING CHANGES

* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334))

### Features

* update scanned endpoint ([#331](https://github.com/after-school-garbage-squad/repaint-server/issues/331)) ([dc56a16](https://github.com/after-school-garbage-squad/repaint-server/commit/dc56a16fe5a460ad5653542e1e55a0ab00c16a4b))


### Bug Fixes

* change HTTP response type to Conflict ([#333](https://github.com/after-school-garbage-squad/repaint-server/issues/333)) ([321b2e0](https://github.com/after-school-garbage-squad/repaint-server/commit/321b2e077bb92fc6f811d90c3253e5e2f698d0ae))
* remove infinity loop ([#334](https://github.com/after-school-garbage-squad/repaint-server/issues/334)) ([be58daf](https://github.com/after-school-garbage-squad/repaint-server/commit/be58dafcb59ad307b0ddeafa11c6ae6cffa8487c))

## [3.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.16.1...repaint-server-v3.0.0) (2023-10-11)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* DESTROY DROP LOGIC ([#322](https://github.com/after-school-garbage-squad/repaint-server/issues/322)) ([e598344](https://github.com/after-school-garbage-squad/repaint-server/commit/e598344f21e9dbb80c5d80f1de1cd5bf01aa2516))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))
* fix spot-id typo ([#311](https://github.com/after-school-garbage-squad/repaint-server/issues/311)) ([5f78d87](https://github.com/after-school-garbage-squad/repaint-server/commit/5f78d87a164562bf597abfcde86b446f1a6f3384))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* **refactor:** fix hw-id logic ([#316](https://github.com/after-school-garbage-squad/repaint-server/issues/316)) ([2f22b36](https://github.com/after-school-garbage-squad/repaint-server/commit/2f22b368977821e3659ce257aa99aaaa14f06e6b))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* set update flag ([#328](https://github.com/after-school-garbage-squad/repaint-server/issues/328)) ([6c7b5f7](https://github.com/after-school-garbage-squad/repaint-server/commit/6c7b5f77d78fb6a5e4ee4c90cb3b0bbbd8ea7c9d))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))
* release 3.0.0 ([24a6ff2](https://github.com/after-school-garbage-squad/repaint-server/commit/24a6ff2b51f98e9a69c55e70c7bcf592ae6b21a2))

## [2.16.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.16.0...repaint-server-v2.16.1) (2023-10-11)


### Bug Fixes

* DESTROY DROP LOGIC ([#322](https://github.com/after-school-garbage-squad/repaint-server/issues/322)) ([e598344](https://github.com/after-school-garbage-squad/repaint-server/commit/e598344f21e9dbb80c5d80f1de1cd5bf01aa2516))

## [2.16.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.15.1...repaint-server-v2.16.0) (2023-10-10)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))
* fix spot-id typo ([#311](https://github.com/after-school-garbage-squad/repaint-server/issues/311)) ([5f78d87](https://github.com/after-school-garbage-squad/repaint-server/commit/5f78d87a164562bf597abfcde86b446f1a6f3384))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* **refactor:** fix hw-id logic ([#316](https://github.com/after-school-garbage-squad/repaint-server/issues/316)) ([2f22b36](https://github.com/after-school-garbage-squad/repaint-server/commit/2f22b368977821e3659ce257aa99aaaa14f06e6b))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.15.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.15.0...repaint-server-v2.15.1) (2023-10-10)


### Bug Fixes

* **refactor:** fix hw-id logic ([#316](https://github.com/after-school-garbage-squad/repaint-server/issues/316)) ([2f22b36](https://github.com/after-school-garbage-squad/repaint-server/commit/2f22b368977821e3659ce257aa99aaaa14f06e6b))

## [2.15.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.14.0...repaint-server-v2.15.0) (2023-10-10)


### Features

* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))
* fix spot-id typo ([#311](https://github.com/after-school-garbage-squad/repaint-server/issues/311)) ([5f78d87](https://github.com/after-school-garbage-squad/repaint-server/commit/5f78d87a164562bf597abfcde86b446f1a6f3384))

## [2.14.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.15.0...repaint-server-v2.14.0) (2023-10-10)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.15.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.14.0...repaint-server-v2.15.0) (2023-10-10)


### Features

* refactor listing visitor images ([#306](https://github.com/after-school-garbage-squad/repaint-server/issues/306)) ([bf3b56f](https://github.com/after-school-garbage-squad/repaint-server/commit/bf3b56f9bff220498b9fd1177ad42466bfef6526))


### Bug Fixes

* fix hw_id logic ([#304](https://github.com/after-school-garbage-squad/repaint-server/issues/304)) ([54effb3](https://github.com/after-school-garbage-squad/repaint-server/commit/54effb38d269badbf2b6dbf18ed0733d3b1069dc))

## [2.14.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.12.0...repaint-server-v2.14.0) (2023-10-09)


### Miscellaneous Chores

* release ([f3c39c7](https://github.com/after-school-garbage-squad/repaint-server/commit/f3c39c76f8b938a2556341d94f0ebb2d26d2d2ad))

## [2.12.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.12.0...repaint-server-v2.12.0) (2023-10-09)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
* update usecase response ([#228](https://github.com/after-school-garbage-squad/repaint-server/issues/228)) ([86ccd82](https://github.com/after-school-garbage-squad/repaint-server/commit/86ccd8221541c7989ee2c7d0423b514c33d0baca))


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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.12.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.11.0...repaint-server-v2.12.0) (2023-10-09)


### Features

* impl gray scale otp to usecase ([#295](https://github.com/after-school-garbage-squad/repaint-server/issues/295)) ([7c16a32](https://github.com/after-school-garbage-squad/repaint-server/commit/7c16a32ed3ad819706a890d0539df59b5757db6c))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* refactor visitor image list usecase ([adad127](https://github.com/after-school-garbage-squad/repaint-server/commit/adad1274b653daace866e07f8fb7aa930f7b1ff6))

## [2.11.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.10.0...repaint-server-v2.11.0) (2023-10-08)


### Features

* impl pubsub to listing images for visitor ([#291](https://github.com/after-school-garbage-squad/repaint-server/issues/291)) ([591099a](https://github.com/after-school-garbage-squad/repaint-server/commit/591099af6ea6018851325136845ff7db8c7ff7e9))

## [2.10.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.10.0...repaint-server-v2.10.0) (2023-10-08)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))
* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* fix drop palette endpoint ([#210](https://github.com/after-school-garbage-squad/repaint-server/issues/210)) ([449b9d7](https://github.com/after-school-garbage-squad/repaint-server/commit/449b9d70151a0f32dc74585ffca6da2c5e5fb0c4))
* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* impl query struct ([#223](https://github.com/after-school-garbage-squad/repaint-server/issues/223)) ([4a4b16e](https://github.com/after-school-garbage-squad/repaint-server/commit/4a4b16eda979619a09cc0a3fc83aa43df8737f14))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* remove bad request body ([#221](https://github.com/after-school-garbage-squad/repaint-server/issues/221)) ([890b9f1](https://github.com/after-school-garbage-squad/repaint-server/commit/890b9f176ee3a5b195b844883a0eeecf584589c0))
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
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
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.10.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.9.0...repaint-server-v2.10.0) (2023-10-08)


### Features

* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))

## [2.9.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.1...repaint-server-v2.9.0) (2023-10-07)


### Features

* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))

## [2.8.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.0...repaint-server-v2.8.1) (2023-10-06)


### Miscellaneous Chores

* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.8.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.7.0...repaint-server-v2.8.0) (2023-10-06)


### Miscellaneous Chores

* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))

## [2.7.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.6.0...repaint-server-v2.7.0) (2023-10-06)


### Features

* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))

## [2.6.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.5.0...repaint-server-v2.6.0) (2023-10-06)


### Features

* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))

## [2.5.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.4.0...repaint-server-v2.5.0) (2023-10-05)


### Miscellaneous Chores

* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))

## [2.4.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.3.1...repaint-server-v2.4.0) (2023-10-05)


### Features

* add publisher to join-visitor usecase ([#255](https://github.com/after-school-garbage-squad/repaint-server/issues/255)) ([c1e9ae9](https://github.com/after-school-garbage-squad/repaint-server/commit/c1e9ae9188050d684dd49c26262de5168d493211))

## [2.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.3.0...repaint-server-v2.3.1) (2023-10-05)


### Miscellaneous Chores

* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))

## [2.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.7...repaint-server-v2.3.0) (2023-10-05)


### Features

* impl update-notificaton adn check-update endpoints ([#249](https://github.com/after-school-garbage-squad/repaint-server/issues/249)) ([349466a](https://github.com/after-school-garbage-squad/repaint-server/commit/349466a6c23ed073882ad46fe12b1ad39eda322d))

## [2.2.7](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.6...repaint-server-v2.2.7) (2023-10-05)


### Bug Fixes

* fix typo ([#243](https://github.com/after-school-garbage-squad/repaint-server/issues/243)) ([ca75143](https://github.com/after-school-garbage-squad/repaint-server/commit/ca75143a05a824b8537e1d326d7b5d8d27203f71))

## [2.2.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.0...repaint-server-v2.2.6) (2023-10-05)


### Features

* change id field type to Option&lt;T&gt; ([#239](https://github.com/after-school-garbage-squad/repaint-server/issues/239)) ([d365e63](https://github.com/after-school-garbage-squad/repaint-server/commit/d365e6329a540457bf96b3f5f8d4a20d357c3c7d))
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
* remove unneeded serde and camelCase serialize ([#233](https://github.com/after-school-garbage-squad/repaint-server/issues/233)) ([6d9d53f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d9d53ff1ba373cb9148a7913ee107455f3082bd))
* rename missed field ([#231](https://github.com/after-school-garbage-squad/repaint-server/issues/231)) ([3737331](https://github.com/after-school-garbage-squad/repaint-server/commit/3737331081520ad29dcdd8f2d93d1adf8d60407b))
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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))

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
