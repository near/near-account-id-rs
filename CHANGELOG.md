# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.18.0](https://github.com/near/near-account-id/compare/v0.17.0...v0.18.0) - 2023-10-24

### Added
- Added `len` method ([#13](https://github.com/near/near-account-id/pull/13))

### Other
- renamed back remove_unchecked -> remove_unvalidated and hid behind a discouraging feature-flag ([#9](https://github.com/near/near-account-id/pull/9))
- Add const AccountIdRef::new_or_panic ([#12](https://github.com/near/near-account-id/pull/12))
- Upgrade borsh to 1.0 ([#8](https://github.com/near/near-account-id/pull/8))
- Use stable Rust version for maximal-deps test ([#7](https://github.com/near/near-account-id/pull/7))
- Added automated release pipeline (release-plz!)
- Remove the `internal_unstable` feature flag
- bump MSRV to 1.65
- Merge pull request [#3](https://github.com/near/near-account-id/pull/3) from near/ci
- make `impl Arbitrary for AccountId` DRYer
- remove support for mutable AccountId refs
- add missing serde/borsh implementations for `AccountIdRef`
- convenience trait implementations for `AccountId`
- add `AsMut` and `DerefMut` implementations
- implement `Arbitrary` for `AccountIdRef`
- more idiomatic AsRef/Borrow impls
- move all methods to `AccountIdRef`
- Introduce AccountIdRef
- reorganize
- typo
- init
