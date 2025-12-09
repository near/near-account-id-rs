# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.3.0](https://github.com/near/near-account-id-rs/compare/v2.2.0...v2.3.0) - 2025-12-09

### Added

- granular `arbitrary` impls ([#50](https://github.com/near/near-account-id-rs/pull/50))

### Fixed

- Ensure transmute safety for AccountIdRef ([#48](https://github.com/near/near-account-id-rs/pull/48))

### Other

- Unpinned Rust version for release-plz (use stable again as we bumped MSRV)

## [2.2.0](https://github.com/near/near-account-id-rs/compare/v2.1.0...v2.2.0) - 2025-12-04

### Added

- `impl AsRef<AccountIdRef> for AccountIdRef` ([#46](https://github.com/near/near-account-id-rs/pull/46))

## [2.1.0](https://github.com/near/near-account-id-rs/compare/v2.0.0...v2.1.0) - 2025-12-03

### Added

- Added `IntoAccountId` trait ([#44](https://github.com/near/near-account-id-rs/pull/44))

## [2.0.0](https://github.com/near/near-account-id-rs/compare/v1.1.4...v2.0.0) - 2025-09-26

### Added

- BREAKING: New variant `NearDeterministicAccount` in `AccountType` as part of [NEP-616](https://github.com/near/NEPs/pull/616) implementation. ([#42](https://github.com/near/near-account-id-rs/pull/42))

### Changed

- `AccountType::is_implicit()` will return `true` for the new account type `NearDeterministicAccount`. Callers should check their assumptions on what this method means.

## [1.1.4](https://github.com/near/near-account-id-rs/compare/v1.1.3...v1.1.4) - 2025-09-07

### Added

- Support both schemars versions at once (without conflicts) ([#40](https://github.com/near/near-account-id-rs/pull/40))

## [1.1.3](https://github.com/near/near-account-id-rs/compare/v1.1.2...v1.1.3) - 2025-07-14

### Other

- Add schemars-v1 and schemars-v0_8 features, reverts schemars-stable to 0.8.22 ([#38](https://github.com/near/near-account-id-rs/pull/38))

## [1.1.2](https://github.com/near/near-account-id-rs/compare/v1.1.1...v1.1.2) - 2025-07-03

### Other

- use toolchain 1.74 for release-plz ([#36](https://github.com/near/near-account-id-rs/pull/36))
- The range of supported schemars versions is extended to >=0.8.22, <2 tu support the latest 1.0 release ([#34](https://github.com/near/near-account-id-rs/pull/34))

## [1.1.1](https://github.com/near/near-account-id-rs/compare/v1.1.0...v1.1.1) - 2025-05-05

### Other

- remove an unnecessary optional feature ([#31](https://github.com/near/near-account-id-rs/pull/31))

## [1.1.0](https://github.com/near/near-account-id-rs/compare/v1.0.0...v1.1.0) - 2025-04-23

### Added

- Added optional schemars-alpha feature to support (schemars 1.0-alpha releases) ([#28](https://github.com/near/near-account-id-rs/pull/28))

## 1.0.0 - 2023-12-22

This is the first stable release of near-account-id crate!

`AccountId` and `AccountIdRef` are two main types of this crate that have same relation as `String` and `str` in standard library of Rust.

AccountId guarantees to hold a valid NEAR account id (unless users explicitly opt-in for the unvalidated constructors feature and break this promise).

See all the changes listed in alpha releases below to learn about `AccountIdRef` and various new helper methods.

### Added
- Add `get_parent_account_id` method ([#24](https://github.com/near/near-account-id-rs/pull/24))

## [1.0.0-alpha.4](https://github.com/near/near-account-id-rs/compare/v1.0.0-alpha.3...v1.0.0-alpha.4) - 2023-11-24

### Fixed
- Remove account_id validation from `new_unvalidated()` when `internal_unstable` feature is enabled (required by nearcore) ([#20](https://github.com/near/near-account-id-rs/pull/20))

## [1.0.0-alpha.3](https://github.com/near/near-account-id-rs/compare/v1.0.0-alpha.2...v1.0.0-alpha.3) - 2023-11-06

### Other
- Add schemars support ([#17](https://github.com/near/near-account-id-rs/pull/17))

## [1.0.0-alpha.2](https://github.com/near/near-account-id-rs/compare/v1.0.0-alpha.1...v1.0.0-alpha.2) - 2023-11-03

### Other
- `AccountType`, add `EthImplicitAccount` ([#14](https://github.com/near/near-account-id-rs/pull/14))

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
