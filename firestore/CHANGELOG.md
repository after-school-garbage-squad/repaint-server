# Changelog













































## [3.6.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.5.0...repaint-server-v3.6.0) (2023-10-19)


### Features

* update palette structure ([#368](https://github.com/after-school-garbage-squad/repaint-server/issues/368)) ([0ecde29](https://github.com/after-school-garbage-squad/repaint-server/commit/0ecde297ff8aa35682a9245d35c6bb8b817c0e29))


### Bug Fixes

* **deps:** update rust crate async-trait to 0.1.74 ([#382](https://github.com/after-school-garbage-squad/repaint-server/issues/382)) ([e30f502](https://github.com/after-school-garbage-squad/repaint-server/commit/e30f50216ce72d836180146895ab72d42253e641))
* **deps:** update rust crate serde to 1.0.189 ([#371](https://github.com/after-school-garbage-squad/repaint-server/issues/371)) ([a4b6a23](https://github.com/after-school-garbage-squad/repaint-server/commit/a4b6a2310f37c10094af8b51ce56a8af48b53c26))
* **deps:** update rust crate tracing to 0.1.39 ([#381](https://github.com/after-school-garbage-squad/repaint-server/issues/381)) ([68a0bf7](https://github.com/after-school-garbage-squad/repaint-server/commit/68a0bf7490185b539658ec2392373f2691e61d13))
* **deps:** update rust crate tracing to 0.1.40 ([#397](https://github.com/after-school-garbage-squad/repaint-server/issues/397)) ([2f96d92](https://github.com/after-school-garbage-squad/repaint-server/commit/2f96d9284cd400d436dc9f1d8510e07f0f720fdc))
* fix subscribe_palette logic ([#373](https://github.com/after-school-garbage-squad/repaint-server/issues/373)) ([10f1f39](https://github.com/after-school-garbage-squad/repaint-server/commit/10f1f39c5bcf1321db6c0e67fbb0ea5051b714a9))

## [3.5.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.0...repaint-server-v3.5.0) (2023-10-12)


### Bug Fixes

* fix drop logic ([#353](https://github.com/after-school-garbage-squad/repaint-server/issues/353)) ([c5ab7fd](https://github.com/after-school-garbage-squad/repaint-server/commit/c5ab7fd289feec4bcada55c28f81b9ded7669944))
* fix drop own palette ([#360](https://github.com/after-school-garbage-squad/repaint-server/issues/360)) ([68c2c18](https://github.com/after-school-garbage-squad/repaint-server/commit/68c2c18d9e970661d44ef4575ab649d72613b6e3))


### Miscellaneous Chores

* release 3.5.0 ([fdef4c3](https://github.com/after-school-garbage-squad/repaint-server/commit/fdef4c34599db0da4f6e79e72713b322afa36b3f))

## [3.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.3...repaint-server-v3.3.0) (2023-10-12)


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
* fix drop logic ([#353](https://github.com/after-school-garbage-squad/repaint-server/issues/353)) ([c5ab7fd](https://github.com/after-school-garbage-squad/repaint-server/commit/c5ab7fd289feec4bcada55c28f81b9ded7669944))
* fix drop own palette ([#360](https://github.com/after-school-garbage-squad/repaint-server/issues/360)) ([68c2c18](https://github.com/after-school-garbage-squad/repaint-server/commit/68c2c18d9e970661d44ef4575ab649d72613b6e3))
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
* release 3.3.0 ([69fd163](https://github.com/after-school-garbage-squad/repaint-server/commit/69fd163e94457f29a8938a32a572d1291d9e4347))

## [3.3.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.3.2...repaint-server-v3.3.3) (2023-10-12)


### Bug Fixes

* fix drop logic ([#353](https://github.com/after-school-garbage-squad/repaint-server/issues/353)) ([c5ab7fd](https://github.com/after-school-garbage-squad/repaint-server/commit/c5ab7fd289feec4bcada55c28f81b9ded7669944))
* fix drop own palette ([#360](https://github.com/after-school-garbage-squad/repaint-server/issues/360)) ([68c2c18](https://github.com/after-school-garbage-squad/repaint-server/commit/68c2c18d9e970661d44ef4575ab649d72613b6e3))

## [3.3.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.2.0...repaint-server-v3.3.0) (2023-10-12)


### Miscellaneous Chores

* release 3.3.0 ([69fd163](https://github.com/after-school-garbage-squad/repaint-server/commit/69fd163e94457f29a8938a32a572d1291d9e4347))

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

## [3.2.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.0.0...repaint-server-v3.2.0) (2023-10-11)


### Features

* impl traffic queue ([#338](https://github.com/after-school-garbage-squad/repaint-server/issues/338)) ([439313f](https://github.com/after-school-garbage-squad/repaint-server/commit/439313f667a9122ec4c1522e4c31910e794c344d))


### Bug Fixes

* **refactor:** remove unneeded firestore ([#336](https://github.com/after-school-garbage-squad/repaint-server/issues/336)) ([39f39d4](https://github.com/after-school-garbage-squad/repaint-server/commit/39f39d4e3433759251e6cd539417e193821ab1f1))


### Miscellaneous Chores

* release 3.2.0 ([1240c24](https://github.com/after-school-garbage-squad/repaint-server/commit/1240c244f5fe03baa2d4cbcaddbc5b0382191a21))

## [3.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v3.0.1...repaint-server-v3.0.0) (2023-10-11)


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

## [3.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.16.1...repaint-server-v3.0.0) (2023-10-11)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* **deps:** update rust crate firestore to 0.37.1 ([#283](https://github.com/after-school-garbage-squad/repaint-server/issues/283)) ([5708b17](https://github.com/after-school-garbage-squad/repaint-server/commit/5708b176687ba58077a9152d073cdd78ea858954))
* **deps:** update rust crate firestore to 0.37.2 ([#287](https://github.com/after-school-garbage-squad/repaint-server/issues/287)) ([a66d357](https://github.com/after-school-garbage-squad/repaint-server/commit/a66d357ea954dc69926a2f907e8dc17a3aa916f7))
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

## [2.16.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.14.0...repaint-server-v2.16.0) (2023-10-10)


### Miscellaneous Chores

* release 2.16.0 ([3d97c7e](https://github.com/after-school-garbage-squad/repaint-server/commit/3d97c7e7b9b02e1eba25c442627342886c6a3af3))

## [2.14.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.14.3...repaint-server-v2.14.0) (2023-10-10)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* **deps:** update rust crate firestore to 0.37.1 ([#283](https://github.com/after-school-garbage-squad/repaint-server/issues/283)) ([5708b17](https://github.com/after-school-garbage-squad/repaint-server/commit/5708b176687ba58077a9152d073cdd78ea858954))
* **deps:** update rust crate firestore to 0.37.2 ([#287](https://github.com/after-school-garbage-squad/repaint-server/issues/287)) ([a66d357](https://github.com/after-school-garbage-squad/repaint-server/commit/a66d357ea954dc69926a2f907e8dc17a3aa916f7))
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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.14.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.12.0...repaint-server-v2.14.0) (2023-10-09)


### Miscellaneous Chores

* release ([f3c39c7](https://github.com/after-school-garbage-squad/repaint-server/commit/f3c39c76f8b938a2556341d94f0ebb2d26d2d2ad))

## [2.12.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.10.0...repaint-server-v2.12.0) (2023-10-09)


### Miscellaneous Chores

* 2.12.0 ([27e2b30](https://github.com/after-school-garbage-squad/repaint-server/commit/27e2b301c782efd59201c55ac135e84f19432bfa))

## [2.10.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.10.1...repaint-server-v2.10.0) (2023-10-09)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* **deps:** update rust crate firestore to 0.37.1 ([#283](https://github.com/after-school-garbage-squad/repaint-server/issues/283)) ([5708b17](https://github.com/after-school-garbage-squad/repaint-server/commit/5708b176687ba58077a9152d073cdd78ea858954))
* **deps:** update rust crate firestore to 0.37.2 ([#287](https://github.com/after-school-garbage-squad/repaint-server/issues/287)) ([a66d357](https://github.com/after-school-garbage-squad/repaint-server/commit/a66d357ea954dc69926a2f907e8dc17a3aa916f7))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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
* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.10.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.1...repaint-server-v2.10.0) (2023-10-08)


### Features

* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate firestore to 0.37.1 ([#283](https://github.com/after-school-garbage-squad/repaint-server/issues/283)) ([5708b17](https://github.com/after-school-garbage-squad/repaint-server/commit/5708b176687ba58077a9152d073cdd78ea858954))
* **deps:** update rust crate firestore to 0.37.2 ([#287](https://github.com/after-school-garbage-squad/repaint-server/issues/287)) ([a66d357](https://github.com/after-school-garbage-squad/repaint-server/commit/a66d357ea954dc69926a2f907e8dc17a3aa916f7))


### Miscellaneous Chores

* release 2.10.0 ([8b2fb2b](https://github.com/after-school-garbage-squad/repaint-server/commit/8b2fb2b172fdaf19597b261a4de03ecd31a6bc3e))

## [2.8.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.2...repaint-server-v2.8.1) (2023-10-08)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))
* update traffic logic ([#285](https://github.com/after-school-garbage-squad/repaint-server/issues/285)) ([19dbcb8](https://github.com/after-school-garbage-squad/repaint-server/commit/19dbcb8c18cd6c788eb77696256f08c4a3a4c0c3))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* **deps:** update rust crate firestore to 0.37.1 ([#283](https://github.com/after-school-garbage-squad/repaint-server/issues/283)) ([5708b17](https://github.com/after-school-garbage-squad/repaint-server/commit/5708b176687ba58077a9152d073cdd78ea858954))
* **deps:** update rust crate firestore to 0.37.2 ([#287](https://github.com/after-school-garbage-squad/repaint-server/issues/287)) ([a66d357](https://github.com/after-school-garbage-squad/repaint-server/commit/a66d357ea954dc69926a2f907e8dc17a3aa916f7))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))
* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.8.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.8.0...repaint-server-v2.8.1) (2023-10-06)


### Miscellaneous Chores

* release 2.8.1 ([7566229](https://github.com/after-school-garbage-squad/repaint-server/commit/7566229587164ebc33f13c99cbfed9ac6d9bd30c))

## [2.8.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.5.2...repaint-server-v2.8.0) (2023-10-06)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))
* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))
* release 2.8.0 ([a21c2a3](https://github.com/after-school-garbage-squad/repaint-server/commit/a21c2a313c4cc608228ed264779ed3e859632aef))

## [2.5.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.3.1...repaint-server-v2.5.0) (2023-10-05)


### Miscellaneous Chores

* release 2.5.0 ([d7465e8](https://github.com/after-school-garbage-squad/repaint-server/commit/d7465e88fe2707c2d70a23972b72a82e1c2901d2))

## [2.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.3.2...repaint-server-v2.3.1) (2023-10-05)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))
* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))

## [2.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.6...repaint-server-v2.3.1) (2023-10-05)


### Miscellaneous Chores

* release 2.3.1 ([aa54a58](https://github.com/after-school-garbage-squad/repaint-server/commit/aa54a5893a362055ebeac32d64a5a601e028d989))

## [2.2.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.2.8...repaint-server-v2.2.6) (2023-10-05)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))

## [2.2.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.6...repaint-server-v2.2.6) (2023-10-05)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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
* release 2.2.6 ([39cb15e](https://github.com/after-school-garbage-squad/repaint-server/commit/39cb15e5a346d8e8c48fabccb6eb140c99c0d8ee))

## [2.1.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.3...repaint-server-v2.1.3) (2023-10-04)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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

## [2.1.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.1.1...repaint-server-v2.1.1) (2023-10-04)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))
* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))
* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
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

## [2.1.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.5...repaint-server-v2.1.0) (2023-10-03)


### Features

* add tracing ([#218](https://github.com/after-school-garbage-squad/repaint-server/issues/218)) ([539b380](https://github.com/after-school-garbage-squad/repaint-server/commit/539b380a56e28731a550c2c35dd0adc6771234cd))

## [2.0.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.2...repaint-server-v2.0.5) (2023-10-03)


### Miscellaneous Chores

* release 2.0.5 ([ed82b90](https://github.com/after-school-garbage-squad/repaint-server/commit/ed82b90361d8c4c8334dadc23061a19756a500f7))

## [2.0.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v2.0.0...repaint-server-v2.0.1) (2023-10-03)


### Miscellaneous Chores

* release 2.0.1 ([6825042](https://github.com/after-school-garbage-squad/repaint-server/commit/68250422afec9d6809fc8088f998e1b959ea73e1))

## [2.0.0](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.6...repaint-server-v2.0.0) (2023-10-03)


### Miscellaneous Chores

* release 2.0.0 ([9d4e710](https://github.com/after-school-garbage-squad/repaint-server/commit/9d4e71044424645e7d018ac220fb3cc2266c2b4f))

## [1.3.6](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.5...repaint-server-v1.3.6) (2023-10-01)


### Miscellaneous Chores

* pre-reelase ([a0255da](https://github.com/after-school-garbage-squad/repaint-server/commit/a0255dad83de12a719433b29c2ff58db1c08a97b))

## [1.3.5](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.4...repaint-server-v1.3.5) (2023-10-01)


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))

## [1.3.4](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.3...repaint-server-v1.3.4) (2023-10-01)


### Bug Fixes

* **deps:** update rust crate firestore to 0.36.0 ([#187](https://github.com/after-school-garbage-squad/repaint-server/issues/187)) ([fb55c6f](https://github.com/after-school-garbage-squad/repaint-server/commit/fb55c6f6cf9479cee97c78ad140c9e893d5777e5))

## [1.3.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.2...repaint-server-v1.3.3) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([d110258](https://github.com/after-school-garbage-squad/repaint-server/commit/d1102587d0797d9f2bfcbadb379a53780bc8dddd))

## [1.3.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.3.1...repaint-server-v1.3.2) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([08653a1](https://github.com/after-school-garbage-squad/repaint-server/commit/08653a1d9a31f15a0f7ffff04105bac2dce1f4f8))

## [1.3.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.3...repaint-server-v1.3.1) (2023-09-30)


### Miscellaneous Chores

* pre-relase ([f9d4309](https://github.com/after-school-garbage-squad/repaint-server/commit/f9d43094bde2b74f22596ae26b5681e0a19aa683))

## [1.2.3](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.2...repaint-server-v1.2.3) (2023-09-30)


### Bug Fixes

* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))
* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))

## [1.2.2](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.1...repaint-server-v1.2.2) (2023-09-29)


### Bug Fixes

* revert credential_path logic and fix Dockerfile ([#172](https://github.com/after-school-garbage-squad/repaint-server/issues/172)) ([87135a5](https://github.com/after-school-garbage-squad/repaint-server/commit/87135a5a0b5adfd07255ab397ba2de402ba17dc3))

## [1.2.1](https://github.com/after-school-garbage-squad/repaint-server/compare/repaint-server-v1.2.0...repaint-server-v1.2.1) (2023-09-29)


### Bug Fixes

* manualy set GOOGLE_CREDENTIALS ([#168](https://github.com/after-school-garbage-squad/repaint-server/issues/168)) ([661124f](https://github.com/after-school-garbage-squad/repaint-server/commit/661124faaba6f6734120b486819686b0caa2e1a3))

## 1.2.0 (2023-09-29)


### Features

* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl gmail crate ([#163](https://github.com/after-school-garbage-squad/repaint-server/issues/163)) ([c34627b](https://github.com/after-school-garbage-squad/repaint-server/commit/c34627bc4c2f9710a6fb88c60f48ff3b08a3648c))
* impl set and check image update flag ([#157](https://github.com/after-school-garbage-squad/repaint-server/issues/157)) ([3f6e9e0](https://github.com/after-school-garbage-squad/repaint-server/commit/3f6e9e01be61009f210ff94ad629d5d462a01d1c))


### Bug Fixes

* fix version in Cargo.toml and generate license text ([#164](https://github.com/after-school-garbage-squad/repaint-server/issues/164)) ([592c532](https://github.com/after-school-garbage-squad/repaint-server/commit/592c532051a0d5d8dd87f1a0321b4a7d6bfd0a61))


### Miscellaneous Chores

* pre-release ([f8c9988](https://github.com/after-school-garbage-squad/repaint-server/commit/f8c99880dda8cfb52ea9edf92c8ddce6ae6246fb))
* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
