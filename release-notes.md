:robot: I have created a release *beep* *boop*
---


<details><summary>repaint-server-api: 3.2.1</summary>

### Dependencies


</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.0...repaint-server-v3.2.0) (2023-10-12)


### Features

* fix hw_id ([#297](https://github.com/after-school-garbage-squad/repaint-server/issues/297)) ([cb248fd](https://github.com/after-school-garbage-squad/repaint-server/commit/cb248fde556ef0decbccf329c7dc60aaa13dcc71))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl download flag ([#265](https://github.com/after-school-garbage-squad/repaint-server/issues/265)) ([bb535bb](https://github.com/after-school-garbage-squad/repaint-server/commit/bb535bb72708232ef806c71d5d56e1a6d50bebe9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl traffic queue ([#338](https://github.com/after-school-garbage-squad/repaint-server/issues/338)) ([439313f](https://github.com/after-school-garbage-squad/repaint-server/commit/439313f667a9122ec4c1522e4c31910e794c344d))
* impl visitor_spots and refactor logics ([#320](https://github.com/after-school-garbage-squad/repaint-server/issues/320)) ([211f553](https://github.com/after-school-garbage-squad/repaint-server/commit/211f553bc7f6b30beb01c865035c7eeccff4f3d2))
* implement entity ([#14](https://github.com/after-school-garbage-squad/repaint-server/issues/14)) ([8b4d4f4](https://github.com/after-school-garbage-squad/repaint-server/commit/8b4d4f448fe75a3aa7d9031e83bf17b3aefa3ece))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))
* refactor visitor_images logic ([#324](https://github.com/after-school-garbage-squad/repaint-server/issues/324)) ([7bcd042](https://github.com/after-school-garbage-squad/repaint-server/commit/7bcd0425e3f24b359feac592bd8e4cd78ee46f37))
* **refactor:** fix visitor state bug with moving logic and table to visitor ([#280](https://github.com/after-school-garbage-squad/repaint-server/issues/280)) ([62fc7cf](https://github.com/after-school-garbage-squad/repaint-server/commit/62fc7cfa538dd32a71e9c8c4d67773dffff9c152))
* **refactor:** managing master palette on register spot ([#343](https://github.com/after-school-garbage-squad/repaint-server/issues/343)) ([33576d8](https://github.com/after-school-garbage-squad/repaint-server/commit/33576d8bc5f56ebcfb4d0a26c932080825bacb4f))
* update event delete query ([#300](https://github.com/after-school-garbage-squad/repaint-server/issues/300)) ([2a5a0cd](https://github.com/after-school-garbage-squad/repaint-server/commit/2a5a0cdb061923c86eed4a2be42ed6c10e6edd4f))
* update spot schema ([#225](https://github.com/after-school-garbage-squad/repaint-server/issues/225)) ([831255f](https://github.com/after-school-garbage-squad/repaint-server/commit/831255f705b75082ef9146dd405f9f0512c1aaa2))


### Bug Fixes

* add unset update to repository and usecase ([#308](https://github.com/after-school-garbage-squad/repaint-server/issues/308)) ([1b2a558](https://github.com/after-school-garbage-squad/repaint-server/commit/1b2a558a4989d49288aadde478cff97e698e8d6e))
* **deps:** update rust crate sea-orm to 0.12.1 ([#29](https://github.com/after-school-garbage-squad/repaint-server/issues/29)) ([8967087](https://github.com/after-school-garbage-squad/repaint-server/commit/896708784c2df32aa0bfb565ecdc795dfca65246))
* **deps:** update rust crate sea-orm to 0.12.2 ([#45](https://github.com/after-school-garbage-squad/repaint-server/issues/45)) ([132dfcd](https://github.com/after-school-garbage-squad/repaint-server/commit/132dfcd20b240551a0c7ef8fdde53294701765a8))
* **deps:** update rust crate sea-orm to 0.12.3 ([#124](https://github.com/after-school-garbage-squad/repaint-server/issues/124)) ([21ae02e](https://github.com/after-school-garbage-squad/repaint-server/commit/21ae02e71a4b454d22a49bc34a3d81de77ee5b0e))
* **deps:** update rust crate serde to 1.0.173 ([#17](https://github.com/after-school-garbage-squad/repaint-server/issues/17)) ([dde258f](https://github.com/after-school-garbage-squad/repaint-server/commit/dde258f6423c42930f88b5834931fea1de0f9639))
* **deps:** update rust crate serde to 1.0.177 ([#22](https://github.com/after-school-garbage-squad/repaint-server/issues/22)) ([cde0f06](https://github.com/after-school-garbage-squad/repaint-server/commit/cde0f06ec09cbb0999e388afe94f14a14c218f45))
* **deps:** update rust crate serde to 1.0.178 ([#34](https://github.com/after-school-garbage-squad/repaint-server/issues/34)) ([184316d](https://github.com/after-school-garbage-squad/repaint-server/commit/184316d2456c2bc8a53865c24532ab29c222221b))
* **deps:** update rust crate serde to 1.0.180 ([#36](https://github.com/after-school-garbage-squad/repaint-server/issues/36)) ([c6fc7da](https://github.com/after-school-garbage-squad/repaint-server/commit/c6fc7daab951143bd26018296bf8a5b4cf68558a))
* **deps:** update rust crate serde to 1.0.183 ([#43](https://github.com/after-school-garbage-squad/repaint-server/issues/43)) ([30a0505](https://github.com/after-school-garbage-squad/repaint-server/commit/30a05059162412a65f360584b2dc30bdfc4b1ca1))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))
* **deps:** update rust crate ulid to 1.0.1 ([#88](https://github.com/after-school-garbage-squad/repaint-server/issues/88)) ([ee3bd42](https://github.com/after-school-garbage-squad/repaint-server/commit/ee3bd42153c8ac2912ce92f3dc1abed447cf9f44))
* **deps:** update rust crate ulid to 1.1.0 ([#109](https://github.com/after-school-garbage-squad/repaint-server/issues/109)) ([1c5dd41](https://github.com/after-school-garbage-squad/repaint-server/commit/1c5dd41e519df2bbdba4101e44b8f754f3406354))
* fix initialize palettes ([#329](https://github.com/after-school-garbage-squad/repaint-server/issues/329)) ([222efe9](https://github.com/after-school-garbage-squad/repaint-server/commit/222efe9941758c2bdde5534ea81a5bbea15f1cbd))
* fix missed duration ([#344](https://github.com/after-school-garbage-squad/repaint-server/issues/344)) ([faf323f](https://github.com/after-school-garbage-squad/repaint-server/commit/faf323fdcc0ce42d3b838eb5e3e36083bdbea65c))
* fix usecase ([#263](https://github.com/after-school-garbage-squad/repaint-server/issues/263)) ([a3c3a98](https://github.com/after-school-garbage-squad/repaint-server/commit/a3c3a989d663f75b01f7241297709fc905ff7d1e))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))
* **refactor:** fix hw-id logic ([#316](https://github.com/after-school-garbage-squad/repaint-server/issues/316)) ([2f22b36](https://github.com/after-school-garbage-squad/repaint-server/commit/2f22b368977821e3659ce257aa99aaaa14f06e6b))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.2.0) (2023-10-12)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl authorization for fcm ([#137](https://github.com/after-school-garbage-squad/repaint-server/issues/137)) ([5c75906](https://github.com/after-school-garbage-squad/repaint-server/commit/5c75906a2c83ddd6257e83b0026396b1eb4dc625))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))


### Bug Fixes

* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.2.0) (2023-10-12)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* impl traffic queue ([#338](https://github.com/after-school-garbage-squad/repaint-server/issues/338)) ([439313f](https://github.com/after-school-garbage-squad/repaint-server/commit/439313f667a9122ec4c1522e4c31910e794c344d))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* **deps:** update rust crate firestore to 0.37.1 ([#283](https://github.com/after-school-garbage-squad/repaint-server/issues/283)) ([5708b17](https://github.com/after-school-garbage-squad/repaint-server/commit/5708b176687ba58077a9152d073cdd78ea858954))
* **deps:** update rust crate firestore to 0.37.2 ([#287](https://github.com/after-school-garbage-squad/repaint-server/issues/287)) ([a66d357](https://github.com/after-school-garbage-squad/repaint-server/commit/a66d357ea954dc69926a2f907e8dc17a3aa916f7))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.2.0) (2023-10-12)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))


### Bug Fixes

* fix typo ([#252](https://github.com/after-school-garbage-squad/repaint-server/issues/252)) ([c79c6e2](https://github.com/after-school-garbage-squad/repaint-server/commit/c79c6e20ac3f84513c59431ed2f23386858c754a))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* remove `$` in path ([#204](https://github.com/after-school-garbage-squad/repaint-server/issues/204)) ([d45e97c](https://github.com/after-school-garbage-squad/repaint-server/commit/d45e97c5eb4ffbb82d20889159bf4ba186e51c24))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.2.0) (2023-10-12)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))


### Bug Fixes

* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.2.0) (2023-10-12)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* change image url ([#235](https://github.com/after-school-garbage-squad/repaint-server/issues/235)) ([a79b544](https://github.com/after-school-garbage-squad/repaint-server/commit/a79b5448ad150e149e99c38743b0dc137e493dc3))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl image proxy for event image ([#178](https://github.com/after-school-garbage-squad/repaint-server/issues/178)) ([2c7f82b](https://github.com/after-school-garbage-squad/repaint-server/commit/2c7f82b6ffbbcf5656656a284b89c9cdee4ba74b))
* impl verify gray scale image ([2b0956a](https://github.com/after-school-garbage-squad/repaint-server/commit/2b0956a9464d0c2f9ba97b02bee2ded4402b95b1))
* update spot schema ([#225](https://github.com/after-school-garbage-squad/repaint-server/issues/225)) ([831255f](https://github.com/after-school-garbage-squad/repaint-server/commit/831255f705b75082ef9146dd405f9f0512c1aaa2))


### Bug Fixes

* add bucket to otp url ([#245](https://github.com/after-school-garbage-squad/repaint-server/issues/245)) ([ef04a8c](https://github.com/after-school-garbage-squad/repaint-server/commit/ef04a8ce3a1c5c5a32e2319c61de0a71ce1e10eb))
* **deps:** update rust crate reqwest to 0.11.21 ([#202](https://github.com/after-school-garbage-squad/repaint-server/issues/202)) ([4b4babc](https://github.com/after-school-garbage-squad/repaint-server/commit/4b4babcf075af7c9e87856cfb39782c4d3e9cd58))
* **deps:** update rust crate reqwest to 0.11.22 ([#213](https://github.com/after-school-garbage-squad/repaint-server/issues/213)) ([bffee73](https://github.com/after-school-garbage-squad/repaint-server/commit/bffee7338717dcfaa0143738e79374ddc4c3d0b5))
* fix forget bucket ([#247](https://github.com/after-school-garbage-squad/repaint-server/issues/247)) ([506eb2d](https://github.com/after-school-garbage-squad/repaint-server/commit/506eb2d6588dd9f084fd1cf438b671aad76f457d))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.2.0) (2023-10-12)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))


### Bug Fixes

* **deps:** update rust crate tokio to 1.33.0 ([#303](https://github.com/after-school-garbage-squad/repaint-server/issues/303)) ([bf1da4b](https://github.com/after-school-garbage-squad/repaint-server/commit/bf1da4be4772ae5b190b355b04e51918053f1dd3))
* fix logic of publish ([#212](https://github.com/after-school-garbage-squad/repaint-server/issues/212)) ([a848262](https://github.com/after-school-garbage-squad/repaint-server/commit/a84826216646f0b3801d8fed55f33d89717a75e3))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.1...repaint-server-v3.2.0) (2023-10-12)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* implement send-grid ([#103](https://github.com/after-school-garbage-squad/repaint-server/issues/103)) ([b06a53f](https://github.com/after-school-garbage-squad/repaint-server/commit/b06a53fb05930e762a1283c85e6af93f803024aa))


### Bug Fixes

* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))


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
</details>

<details><summary>repaint-server: 3.2.0</summary>

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.0...repaint-server-v3.2.0) (2023-10-12)


###   BREAKING CHANGES

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
</details>

---
This PR was generated with [Release Please](https://github.com/googleapis/release-please). See [documentation](https://github.com/googleapis/release-please#release-please).