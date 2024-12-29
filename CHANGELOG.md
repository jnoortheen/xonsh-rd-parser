# Changelog

## [1.4.1](https://github.com/jnoortheen/xonsh-rd-parser/compare/v1.4.0...v1.4.1) (2024-12-29)


### Bug Fixes

* clippy warnings ([c7297de](https://github.com/jnoortheen/xonsh-rd-parser/commit/c7297de8b1c9a86b5b7c89afdd39da0f4976e993))

## [1.4.0](https://github.com/jnoortheen/xonsh-rd-parser/compare/v1.3.0...v1.4.0) (2024-12-25)


### Features

* implement lexer.split ([2947d4c](https://github.com/jnoortheen/xonsh-rd-parser/commit/2947d4c089dc1abbdb2d45272b47a6d2a544ebea))
* pull changes from ruff-main ([04a8351](https://github.com/jnoortheen/xonsh-rd-parser/commit/04a8351084ab4b9a1f6812b102cb51b86cf70de1))
* support tolerant lexing ([9654263](https://github.com/jnoortheen/xonsh-rd-parser/commit/965426318b7dec576a6c4b9bb494aaefada26b47))

## [1.3.0](https://github.com/jnoortheen/xonsh-rd-parser/compare/v1.2.1...v1.3.0) (2024-12-25)


### Features

* implement call-macros ([10f8b79](https://github.com/jnoortheen/xonsh-rd-parser/commit/10f8b79a5a2ff434614f59c1deb1ef045b6d05e3))
* implement with macros ([09b7b31](https://github.com/jnoortheen/xonsh-rd-parser/commit/09b7b31159455e866bc0ef16ab6a2659a67b9f2d))


### Bug Fixes

* linter errors ([af3344d](https://github.com/jnoortheen/xonsh-rd-parser/commit/af3344dde98c7029d74a53aaf7103a247c0d7cca))

## [1.2.1](https://github.com/jnoortheen/xonsh-rd-parser/compare/v1.2.0...v1.2.1) (2024-12-24)


### Bug Fixes

* parse 0 starting numbers without validation ([8d1164a](https://github.com/jnoortheen/xonsh-rd-parser/commit/8d1164a5ed85bf1adf8113f425fad0848e15816b)), closes [#11](https://github.com/jnoortheen/xonsh-rd-parser/issues/11)

## [1.2.0](https://github.com/jnoortheen/xonsh-rd-parser/compare/v1.1.0...v1.2.0) (2024-12-19)


### Features

* update pypi description ([7f847c2](https://github.com/jnoortheen/xonsh-rd-parser/commit/7f847c255933387e46b5d64e82bcfa4ab35c4499))

## [1.1.0](https://github.com/jnoortheen/xonsh-rd-parser/compare/v1.0.0...v1.1.0) (2024-12-18)


### Features

* implement redirects parsing ([c86bf3d](https://github.com/jnoortheen/xonsh-rd-parser/commit/c86bf3dd0753688791dcc5d233a82e9edf4dcdf3))


### Bug Fixes

* clippy warnings ([0483810](https://github.com/jnoortheen/xonsh-rd-parser/commit/0483810012e107c98453489fd029a48db9c5356c))

## [1.0.0](https://github.com/jnoortheen/xonsh-rd-parser/compare/v0.3.1...v1.0.0) (2024-12-17)


### ⚠ BREAKING CHANGES

* single interface Parser for lexing and parsing

### Features

* add Parser class ([88b488d](https://github.com/jnoortheen/xonsh-rd-parser/commit/88b488da211c10b45b864862fca6f17b06f67a46))
* Token.lexpos and line numbers passed ([9fadf13](https://github.com/jnoortheen/xonsh-rd-parser/commit/9fadf130ea54b93342ad3829a29d368e81607f73))
* Token.value is now supported ([e788d90](https://github.com/jnoortheen/xonsh-rd-parser/commit/e788d904779f5a4e7acd46cf49839dcfee9622cb))


### Code Refactoring

* single interface Parser for lexing and parsing ([6ddd96f](https://github.com/jnoortheen/xonsh-rd-parser/commit/6ddd96f43483a9059515d5316e845efde2e0946e))

## [0.3.1](https://github.com/jnoortheen/xonsh-rd-parser/compare/v0.3.0...v0.3.1) (2024-12-17)


### Bug Fixes

* call syntaxerror with filename and location ([b9aaf27](https://github.com/jnoortheen/xonsh-rd-parser/commit/b9aaf2724aa0d44a516de77322a3e24b6b236756))
* cargo fix ([3798a69](https://github.com/jnoortheen/xonsh-rd-parser/commit/3798a69a30b6ca0f42da0fc3c26fd1fa29e31991))
* clippy autofixes ([b3fc53e](https://github.com/jnoortheen/xonsh-rd-parser/commit/b3fc53e386634733f04f10787a937ef2ff48f0f3))
* clippy warnings ([4df93af](https://github.com/jnoortheen/xonsh-rd-parser/commit/4df93af0986c85954d431c977c85b2290792897b))
* clippy warnings ([6cafbb6](https://github.com/jnoortheen/xonsh-rd-parser/commit/6cafbb65ada99bbc02eaa5c03d07cdad77c2d300))

## [0.3.0](https://github.com/jnoortheen/xonsh-rd-parser/compare/v0.2.0...v0.3.0) (2024-12-14)


### Features

* add lexer helpers ([b4edd04](https://github.com/jnoortheen/xonsh-rd-parser/commit/b4edd04c9b7437684fcbfde6ea00777303a36cba))
* add lexing for subproc_tokens ([b429e1b](https://github.com/jnoortheen/xonsh-rd-parser/commit/b429e1bd9279e3529428ac0caa880d29adaf01cd))
* implement parsing subproc macros ([f0775b0](https://github.com/jnoortheen/xonsh-rd-parser/commit/f0775b094f2a01ef68a3beecb29747990cce13ad))
* Lexer class interface ([1d9fd70](https://github.com/jnoortheen/xonsh-rd-parser/commit/1d9fd704076d391b2e8c4dd0424f8089fe5de47e))
* macros implement recursive nested ([30be297](https://github.com/jnoortheen/xonsh-rd-parser/commit/30be2978f3eedf82a67a2aaeae49895e50a26a5c))

## [0.2.0](https://github.com/jnoortheen/xonsh-rd-parser/compare/v0.1.1...v0.2.0) (2024-12-12)


### Features

* accept file_name for parse_string function ([cdacde2](https://github.com/jnoortheen/xonsh-rd-parser/commit/cdacde296844f72b60c058ebe23445f1f652d5c2))
* add lexer python interface and tests ([9f92fa1](https://github.com/jnoortheen/xonsh-rd-parser/commit/9f92fa1abd987c9f9aedf176bf0b3260e6164e2f))
* add py.typed ([fcec046](https://github.com/jnoortheen/xonsh-rd-parser/commit/fcec0469830b39fa8f81791b14ef45c664609df2))
* lexer expose Python compatible token names ([1d54252](https://github.com/jnoortheen/xonsh-rd-parser/commit/1d5425214b9a3084c57cda9f6c405884582c0f6e))
* remove lexing with extra-state ([ea46c4d](https://github.com/jnoortheen/xonsh-rd-parser/commit/ea46c4d0c67ca7f846dcdfe9e9f5b143a531fb4e))


### Bug Fixes

* bring back Token.kind ([5188182](https://github.com/jnoortheen/xonsh-rd-parser/commit/51881822955d01dab818b20ff8ba573b011a94a0))
* warnings ([5820637](https://github.com/jnoortheen/xonsh-rd-parser/commit/5820637ed7844772f07a2c13137ae8d6437e78e7))

## [0.1.1](https://github.com/jnoortheen/xonsh-rd-parser/compare/v0.1.0...v0.1.1) (2024-12-02)


### Bug Fixes

* release with PyPI's trusted publisher ([b7ed5bd](https://github.com/jnoortheen/xonsh-rd-parser/commit/b7ed5bd8f4b54674a0615475d8c5c1e638b9451a))

## 0.1.0 (2024-12-02)


### Features

* add parse_file ([f2ae1f8](https://github.com/jnoortheen/xonsh-rd-parser/commit/f2ae1f84767da0f401e566cb73ef15d071c9c786))
* add parse_string ([2b4d5bb](https://github.com/jnoortheen/xonsh-rd-parser/commit/2b4d5bb9dc19b5e87aa85a41ce4d06b01d14b495))
* add pytest ([f763c7d](https://github.com/jnoortheen/xonsh-rd-parser/commit/f763c7d6fe306ef31a39d3a2645a6cbffe5721c4))
* add pytest-benchmark ([42b26df](https://github.com/jnoortheen/xonsh-rd-parser/commit/42b26dfc20ab46917d29e9b10f08c7d8567ff90a))
* add type annotation ([5c0c62f](https://github.com/jnoortheen/xonsh-rd-parser/commit/5c0c62fd1a2e2421ac6ac00f761584468a8a0503))
* add xonsh lexing of operators ([154dd19](https://github.com/jnoortheen/xonsh-rd-parser/commit/154dd1984a6b0d97edd725bbc0e1b288087b4985))
* added test cases from pegen ([64a699d](https://github.com/jnoortheen/xonsh-rd-parser/commit/64a699d39c9f6eac6feb985cdff6f921942021b4))
* annotate error on source code using miette ([8e548c5](https://github.com/jnoortheen/xonsh-rd-parser/commit/8e548c575ee6494bfa5ab1e3feb2a7161df8200d))
* converting all the ast expr ([11e7cfb](https://github.com/jnoortheen/xonsh-rd-parser/commit/11e7cfb4084f94037fb48a1d3f83c025a0beba90))
* implement @(...) ([dc09ff2](https://github.com/jnoortheen/xonsh-rd-parser/commit/dc09ff25cfc7587a78549897c8eb53d77c553e78))
* implement @$(...) ([8105e5e](https://github.com/jnoortheen/xonsh-rd-parser/commit/8105e5e34fe40e6eacff2fc7ff7d26df53b7fc7c))
* implement backtick strings ([2bd4d58](https://github.com/jnoortheen/xonsh-rd-parser/commit/2bd4d583225095346c3ccae2debb8a12c7e0ad17))
* implement empty ast conversions ([f4a1503](https://github.com/jnoortheen/xonsh-rd-parser/commit/f4a1503c7d70de18d39f215bfe2bce403e535197))
* implement more expressions ([4d71908](https://github.com/jnoortheen/xonsh-rd-parser/commit/4d719085d92ead1b0797d0e42c1eee9c40fe9630))
* implement parsing $env or ${env} ([839f1a1](https://github.com/jnoortheen/xonsh-rd-parser/commit/839f1a13f71419741fe53197d9894466e3f2941c))
* implement parsing piped procs ([f706b29](https://github.com/jnoortheen/xonsh-rd-parser/commit/f706b29be7fe14f02e40399545ea8399a465405e))
* implement rest of the ast conversion ([d394b62](https://github.com/jnoortheen/xonsh-rd-parser/commit/d394b62e01ce81a88fd7cf75ff2a25b53d5dd413))
* initial commit with pkgs from ruff ([fd444da](https://github.com/jnoortheen/xonsh-rd-parser/commit/fd444daee6947c95e89ea57a6c6cbcdddf85fa64))
* lex path string prefix ([ce26b76](https://github.com/jnoortheen/xonsh-rd-parser/commit/ce26b76ffbd5a8bbe0a5ca0dac6e8ac34410c12a))
* move to_ast to its own module ([21e3313](https://github.com/jnoortheen/xonsh-rd-parser/commit/21e33138e63522416554721736fba3b751ed1cfd))
* move xonsh specific parsers to own module ([89404cb](https://github.com/jnoortheen/xonsh-rd-parser/commit/89404cb20a21648371c18b0cc7460329811b24fb))
* parse [@foo](https://github.com/foo)`` inside python expressions correctly ([bdff89e](https://github.com/jnoortheen/xonsh-rd-parser/commit/bdff89ec60f81c5028c1f0e204b3ef1e1a2301be))
* parse && and || ([51b2d78](https://github.com/jnoortheen/xonsh-rd-parser/commit/51b2d78f7a1f285ce884c0253736e9ca7dce6c78))
* parse help? and ?? operators ([3d31f22](https://github.com/jnoortheen/xonsh-rd-parser/commit/3d31f225a39bd1d6db06a54dd68a877ccb136a76))
* parse interpolated values ([55897b2](https://github.com/jnoortheen/xonsh-rd-parser/commit/55897b2ab122b29db2f7a7e89d1d286863a0ec2a))
* parse path/regex/glob strings ([06df67a](https://github.com/jnoortheen/xonsh-rd-parser/commit/06df67acf80c0432a89b3fd813d394582717b752))
* parse sub-proc args ([d57bf9a](https://github.com/jnoortheen/xonsh-rd-parser/commit/d57bf9adcd7e428e4ec26a542b34e5fecb984c2f))
* parse subprocs ([fe46921](https://github.com/jnoortheen/xonsh-rd-parser/commit/fe469215d105ea5270fbfb41621cbaa4db2cdb47))
* split conversion code ([b999825](https://github.com/jnoortheen/xonsh-rd-parser/commit/b999825de057fb3f7ab230333534654f006043ce))
* update regex/glob search paths ([2c449aa](https://github.com/jnoortheen/xonsh-rd-parser/commit/2c449aa672afc4dcbba9d8ee5f7fd7a5c56024b5))
* upgrade to pyo3 ([b91b821](https://github.com/jnoortheen/xonsh-rd-parser/commit/b91b821e70a06019ff21e8938b0f730b1a31fa3c))
* use __xonsh__.cmd for subproc functions ([8d831ae](https://github.com/jnoortheen/xonsh-rd-parser/commit/8d831ae6a418997dda0a4406c993fb5e0d5d7942))
* use code-snippets to raise exceptions ([e5d0110](https://github.com/jnoortheen/xonsh-rd-parser/commit/e5d0110535711c0fc729117fdda0059f7844161a))
* use conversion trait ([1c10060](https://github.com/jnoortheen/xonsh-rd-parser/commit/1c100609e5cf68120d91a3ac4c74d5bbd41b8647))
* use Python's ast module directly ([0abe52e](https://github.com/jnoortheen/xonsh-rd-parser/commit/0abe52eb3488c1dc03c21f56a87f28592b68c17c))
* use syrupy to test parsing/unparsing ([ef26711](https://github.com/jnoortheen/xonsh-rd-parser/commit/ef26711ca133e96753c97db507b9de4a8279cffa))


### Bug Fixes

* ast parse errors ([600ff2d](https://github.com/jnoortheen/xonsh-rd-parser/commit/600ff2d110bb638949b62e16c5be302d4ea28f34))
* ast warnings and failing tests ([d906354](https://github.com/jnoortheen/xonsh-rd-parser/commit/d906354bdf46bf3b0796576494ded53e299fec27))
* elif clause generation ([0da28be](https://github.com/jnoortheen/xonsh-rd-parser/commit/0da28be30a689f89b151af42eead3b98b4f8e7c9))
* failing tests with ([e94a3a1](https://github.com/jnoortheen/xonsh-rd-parser/commit/e94a3a1f4cb477b65f149a8e5fa09c0f4bbac071))
* handle parser infinite loop ([57f3078](https://github.com/jnoortheen/xonsh-rd-parser/commit/57f307846af3926468102b504c7afd5daca61ae5))
* lambda ast generated ([ce3b8d0](https://github.com/jnoortheen/xonsh-rd-parser/commit/ce3b8d0a92538b62053ea12623840c3249a946b4))
* parse && and || ([355f40f](https://github.com/jnoortheen/xonsh-rd-parser/commit/355f40f73d9d86c5c4a77232f98cd572d8fa6b4f))
* parsing @(...) to wrap in list_of_strs_or_callables ([1d41fbe](https://github.com/jnoortheen/xonsh-rd-parser/commit/1d41fbe5a0437ca18a88a71f30d9494627691262))
* parsing sub-proc atoms ([a608f6a](https://github.com/jnoortheen/xonsh-rd-parser/commit/a608f6a5c4de7f0a1c9448f407cde3f53af7cb84))
* parsing subproc args ([c54b175](https://github.com/jnoortheen/xonsh-rd-parser/commit/c54b175627de5db5bd702029d8ae0a70c1ffb68b))
* pyo3 deprecation warnings about IntoPy ([e4c3bd4](https://github.com/jnoortheen/xonsh-rd-parser/commit/e4c3bd43624bd89ef5b63a50f9904dca3dc5b17e))
* string generation ([95be94f](https://github.com/jnoortheen/xonsh-rd-parser/commit/95be94f3dea04d7001c5ba418ebb4ad175157585))
* terminating sub-procs at Rpar ([eed4ef0](https://github.com/jnoortheen/xonsh-rd-parser/commit/eed4ef08136ca3ebdbab38cc16c5e3a2a917297a))
