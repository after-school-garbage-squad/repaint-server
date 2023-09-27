# Changelog

## 1.0.0 (2023-09-27)


### Features

* **grafana:** Integrating grafana into docker compose ([#51](https://github.com/after-school-garbage-squad/repaint-server/issues/51)) ([37f9a4f](https://github.com/after-school-garbage-squad/repaint-server/commit/37f9a4f023a4bafdac0d8fba25223259bf1913df))
* impl api with axum ([#128](https://github.com/after-school-garbage-squad/repaint-server/issues/128)) ([f72b750](https://github.com/after-school-garbage-squad/repaint-server/commit/f72b750384aea69461c86afcf68cc44a2f0e33f9))
* impl authorization for fcm ([#137](https://github.com/after-school-garbage-squad/repaint-server/issues/137)) ([5c75906](https://github.com/after-school-garbage-squad/repaint-server/commit/5c75906a2c83ddd6257e83b0026396b1eb4dc625))
* impl firestore ([#101](https://github.com/after-school-garbage-squad/repaint-server/issues/101)) ([8d49c7a](https://github.com/after-school-garbage-squad/repaint-server/commit/8d49c7aeb9432505a0fb512ac269580ebf506d84))
* impl GCS client ([#134](https://github.com/after-school-garbage-squad/repaint-server/issues/134)) ([20080de](https://github.com/after-school-garbage-squad/repaint-server/commit/20080def8dce90ad8ec80399f5c6d29bed9f9cf4))
* impl image otp client ([#142](https://github.com/after-school-garbage-squad/repaint-server/issues/142)) ([2a36333](https://github.com/after-school-garbage-squad/repaint-server/commit/2a36333c0e58bd4e57ce440eb44379f4b7053ca5))
* impl merge publish ([#147](https://github.com/after-school-garbage-squad/repaint-server/issues/147)) ([41ed438](https://github.com/after-school-garbage-squad/repaint-server/commit/41ed4383a1f92511a6f0e5ba8a62890bf9e5b219))
* impl pubsub ([#143](https://github.com/after-school-garbage-squad/repaint-server/issues/143)) ([30db3ab](https://github.com/after-school-garbage-squad/repaint-server/commit/30db3ab52d5e67b9a3236963bb86fe4e1d66da3f))
* implement entity ([#14](https://github.com/after-school-garbage-squad/repaint-server/issues/14)) ([8b4d4f4](https://github.com/after-school-garbage-squad/repaint-server/commit/8b4d4f448fe75a3aa7d9031e83bf17b3aefa3ece))
* implement fcm ([#105](https://github.com/after-school-garbage-squad/repaint-server/issues/105)) ([a40935a](https://github.com/after-school-garbage-squad/repaint-server/commit/a40935a73fa2113785dda9f22be56e4b779c1e98))
* implement migration ([#8](https://github.com/after-school-garbage-squad/repaint-server/issues/8)) ([776cc50](https://github.com/after-school-garbage-squad/repaint-server/commit/776cc50f4922422851842f87d8b91477c969eb66))
* implement model ([#10](https://github.com/after-school-garbage-squad/repaint-server/issues/10)) ([f4fbbeb](https://github.com/after-school-garbage-squad/repaint-server/commit/f4fbbeb02808848dd4baa12dd030f7a4f52c52c7))
* implement send-grid ([#103](https://github.com/after-school-garbage-squad/repaint-server/issues/103)) ([b06a53f](https://github.com/after-school-garbage-squad/repaint-server/commit/b06a53fb05930e762a1283c85e6af93f803024aa))
* implement usecase ([#57](https://github.com/after-school-garbage-squad/repaint-server/issues/57)) ([86a625a](https://github.com/after-school-garbage-squad/repaint-server/commit/86a625a906d70a29c6d2ee1953ad12efadbac55a))
* initialize commit ([54555fd](https://github.com/after-school-garbage-squad/repaint-server/commit/54555fd926d33fe797d10d090faa2d0189a1b696))
* inject impls to repository ([#75](https://github.com/after-school-garbage-squad/repaint-server/issues/75)) ([7973ab5](https://github.com/after-school-garbage-squad/repaint-server/commit/7973ab5e697add8fbe23deece407fd370dadd164))


### Bug Fixes

* **deps:** update rust crate anyhow to 1.0.75 ([#71](https://github.com/after-school-garbage-squad/repaint-server/issues/71)) ([d033928](https://github.com/after-school-garbage-squad/repaint-server/commit/d0339280a2b8bc73884ea12b3304a75b8b3ff299))
* **deps:** update rust crate sea-orm to 0.12.1 ([#29](https://github.com/after-school-garbage-squad/repaint-server/issues/29)) ([8967087](https://github.com/after-school-garbage-squad/repaint-server/commit/896708784c2df32aa0bfb565ecdc795dfca65246))
* **deps:** update rust crate sea-orm to 0.12.2 ([#45](https://github.com/after-school-garbage-squad/repaint-server/issues/45)) ([132dfcd](https://github.com/after-school-garbage-squad/repaint-server/commit/132dfcd20b240551a0c7ef8fdde53294701765a8))
* **deps:** update rust crate sea-orm to 0.12.3 ([#124](https://github.com/after-school-garbage-squad/repaint-server/issues/124)) ([21ae02e](https://github.com/after-school-garbage-squad/repaint-server/commit/21ae02e71a4b454d22a49bc34a3d81de77ee5b0e))
* **deps:** update rust crate sea-orm-migration to 0.11.3 ([#1](https://github.com/after-school-garbage-squad/repaint-server/issues/1)) ([eb3468b](https://github.com/after-school-garbage-squad/repaint-server/commit/eb3468b9ef5fc3ce4fb5d283a7064d4fa7c28073))
* **deps:** update rust crate sea-orm-migration to 0.12.1 ([#30](https://github.com/after-school-garbage-squad/repaint-server/issues/30)) ([6d71c4f](https://github.com/after-school-garbage-squad/repaint-server/commit/6d71c4f78fd26960ad9444b8240846391067e12d))
* **deps:** update rust crate sea-orm-migration to 0.12.2 ([#46](https://github.com/after-school-garbage-squad/repaint-server/issues/46)) ([d248e6d](https://github.com/after-school-garbage-squad/repaint-server/commit/d248e6dd2fd714cd6383a9b2ed89a30df716a2e2))
* **deps:** update rust crate sea-orm-migration to 0.12.3 ([#125](https://github.com/after-school-garbage-squad/repaint-server/issues/125)) ([7d0176b](https://github.com/after-school-garbage-squad/repaint-server/commit/7d0176bc0fcf34ba5a90c8158c838156084c5cc3))
* **deps:** update rust crate serde to 1.0.173 ([#17](https://github.com/after-school-garbage-squad/repaint-server/issues/17)) ([dde258f](https://github.com/after-school-garbage-squad/repaint-server/commit/dde258f6423c42930f88b5834931fea1de0f9639))
* **deps:** update rust crate serde to 1.0.177 ([#22](https://github.com/after-school-garbage-squad/repaint-server/issues/22)) ([cde0f06](https://github.com/after-school-garbage-squad/repaint-server/commit/cde0f06ec09cbb0999e388afe94f14a14c218f45))
* **deps:** update rust crate serde to 1.0.178 ([#34](https://github.com/after-school-garbage-squad/repaint-server/issues/34)) ([184316d](https://github.com/after-school-garbage-squad/repaint-server/commit/184316d2456c2bc8a53865c24532ab29c222221b))
* **deps:** update rust crate serde to 1.0.180 ([#36](https://github.com/after-school-garbage-squad/repaint-server/issues/36)) ([c6fc7da](https://github.com/after-school-garbage-squad/repaint-server/commit/c6fc7daab951143bd26018296bf8a5b4cf68558a))
* **deps:** update rust crate serde to 1.0.183 ([#43](https://github.com/after-school-garbage-squad/repaint-server/issues/43)) ([30a0505](https://github.com/after-school-garbage-squad/repaint-server/commit/30a05059162412a65f360584b2dc30bdfc4b1ca1))
* **deps:** update rust crate serde to 1.0.185 ([#69](https://github.com/after-school-garbage-squad/repaint-server/issues/69)) ([05a3a0a](https://github.com/after-school-garbage-squad/repaint-server/commit/05a3a0aff5378f292f1a237485992aa2e8afb21e))
* **deps:** update rust crate serde to 1.0.188 ([#81](https://github.com/after-school-garbage-squad/repaint-server/issues/81)) ([3cf6ba6](https://github.com/after-school-garbage-squad/repaint-server/commit/3cf6ba6bc61dfbd5c87ff4301dbfa863b5077a8e))
* **deps:** update rust crate thiserror to 1.0.47 ([#72](https://github.com/after-school-garbage-squad/repaint-server/issues/72)) ([d603899](https://github.com/after-school-garbage-squad/repaint-server/commit/d60389910d5b02825dac140d39877503fe23537c))
* **deps:** update rust crate thiserror to 1.0.48 ([#94](https://github.com/after-school-garbage-squad/repaint-server/issues/94)) ([358d7b1](https://github.com/after-school-garbage-squad/repaint-server/commit/358d7b11390ecbb88f632ed183e0cd32245d7be8))
* **deps:** update rust crate thiserror to 1.0.49 ([#140](https://github.com/after-school-garbage-squad/repaint-server/issues/140)) ([4fedbe8](https://github.com/after-school-garbage-squad/repaint-server/commit/4fedbe87b1429f74b2282391d49b9baabbf5c066))
* **deps:** update rust crate ulid to 1.0.1 ([#88](https://github.com/after-school-garbage-squad/repaint-server/issues/88)) ([ee3bd42](https://github.com/after-school-garbage-squad/repaint-server/commit/ee3bd42153c8ac2912ce92f3dc1abed447cf9f44))
* **deps:** update rust crate ulid to 1.1.0 ([#109](https://github.com/after-school-garbage-squad/repaint-server/issues/109)) ([1c5dd41](https://github.com/after-school-garbage-squad/repaint-server/commit/1c5dd41e519df2bbdba4101e44b8f754f3406354))
* make Option return and default image can be the current image by ([#149](https://github.com/after-school-garbage-squad/repaint-server/issues/149)) ([672baae](https://github.com/after-school-garbage-squad/repaint-server/commit/672baaec52d1679b7d167df0c11ff4a697476a14))


### Miscellaneous Chores

* pre-release ([6e06deb](https://github.com/after-school-garbage-squad/repaint-server/commit/6e06deb70d36c79aab99b8d1ca00e6a2ceaaa525))
