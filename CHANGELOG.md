# Changelog

## [0.4.0](https://github.com/RadekDvorak/helper-ecr-login-auto/compare/v0.3.1...v0.4.0) (2025-09-13)


### Features

* add SHA256 verification for build artifacts in workflow ([#16](https://github.com/RadekDvorak/helper-ecr-login-auto/issues/16)) ([a3980a1](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/a3980a1875058146d321dcb6072e4828d4616e91))
* enable buffered reading from stdin with StdinLock ([#14](https://github.com/RadekDvorak/helper-ecr-login-auto/issues/14)) ([6f3e66f](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/6f3e66f03e12cd981b68563e9a04640105e3413d))
* improve edge case handling ([e5b9481](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/e5b94817e6952c3427dac6bc2eb2781afcd160a6))

## [0.3.1](https://github.com/RadekDvorak/helper-ecr-login-auto/compare/v0.3.0...v0.3.1) (2025-09-02)


### Bug Fixes

* correct condition for dispatching build workflow in release-please.yml ([#12](https://github.com/RadekDvorak/helper-ecr-login-auto/issues/12)) ([e91e2dc](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/e91e2dc229662151b902e99fd08276f213c09cae))

## [0.3.0](https://github.com/RadekDvorak/helper-ecr-login-auto/compare/v0.2.2...v0.3.0) (2025-09-02)


### Features

* add account id type ([#7](https://github.com/RadekDvorak/helper-ecr-login-auto/issues/7)) ([5b22039](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/5b22039b5a42b02f78c2804696dfef0d154313f5))
* add GitHub Actions workflows for building and releasing binaries ([#9](https://github.com/RadekDvorak/helper-ecr-login-auto/issues/9)) ([063a15f](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/063a15f74f897855c23f164a8ab98c886879fe5e))
* mock environment ([#6](https://github.com/RadekDvorak/helper-ecr-login-auto/issues/6)) ([c03d673](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/c03d67365dc04a00dd528e262d227c54c47f5cc6))
* optimize logging, command execution, and AWS profile management ([#8](https://github.com/RadekDvorak/helper-ecr-login-auto/issues/8)) ([432de18](https://github.com/RadekDvorak/helper-ecr-login-auto/commit/432de18b2c39f1316302451926b1e5fcc92d8982))
