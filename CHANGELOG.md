# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 1.0.0-alpha.1 - 2023-10-24

near-account-id was extracted from [nearcore](https://github.com/near/nearcore) as of 2023-08-01, and extended with the following features to reach stable 1.0.0 release.

### Added
- Introduce `AccountIdRef`, move all `AccountId` methods to `AccountIdRef`, and more idiomatic AsRef/Borrow impls
- Added `len` method ([#13](https://github.com/near/near-account-id/pull/13))
- Added const `AccountIdRef::new_or_panic` ([#12](https://github.com/near/near-account-id/pull/12))
- Added missing serde/borsh implementations for `AccountIdRef`
- Upgrade `borsh` dependency to 1.0 ([#8](https://github.com/near/near-account-id/pull/8))
- Implemented `Arbitrary` for `AccountIdRef`

### Other
- Use stable Rust version for maximal-deps test ([#7](https://github.com/near/near-account-id/pull/7))
- Added automated release pipeline (release-plz!)
- bump MSRV to 1.65
