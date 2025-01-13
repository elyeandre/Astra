# Changelog

All notable changes to this project will be documented in this file.

## [0.7.0] - 2025-01-07

### Documentation

- Added basic documentation and guides ([f15be4d](https://github.com/ArkForgeLabs/Astra/commit/f15be4d9772aa0d46947f4b6225122581e040259))
- Updated the links in the page ([f572b38](https://github.com/ArkForgeLabs/Astra/commit/f572b3897548b656aab8641718bf3ad0d272301e))
- Updated to add versioning and a main page ([709b9d0](https://github.com/ArkForgeLabs/Astra/commit/709b9d06c71242147551c5a3725c1be143207004))
- Started writing documentation for the project ([caed6d5](https://github.com/ArkForgeLabs/Astra/commit/caed6d5543e61c6bba1543212f269c62d9ccaeff))

### Features

- Added multipart support ([7778e8a](https://github.com/ArkForgeLabs/Astra/commit/7778e8abe59957754cb14698d461929563da7bf6))
- Changed Makefile into a lua script for better automation ([eec31fa](https://github.com/ArkForgeLabs/Astra/commit/eec31fa8d587b492b386d5fe2ecc36309727d974))

### Miscellaneous Tasks

- Update version ([553a9c9](https://github.com/ArkForgeLabs/Astra/commit/553a9c91c6219d5254f3da07bf5bc397fb02ce60))

## [0.6.0] - 2025-01-05

### Features

- Added async tasks throughg tokio tasks ([97bcba8](https://github.com/ArkForgeLabs/Astra/commit/97bcba8f35d93a68e05b98a485ca3a793a6d0f16))

## [0.5.0] - 2025-01-01

### Features

- Added async task client http request support ([9ea6cfa](https://github.com/ArkForgeLabs/Astra/commit/9ea6cfa263f8a45d1f167fe8fc4e5a3dddac07da))

### Miscellaneous Tasks

- Updated dependencies and now Windows releases are supported ([577671c](https://github.com/ArkForgeLabs/Astra/commit/577671c93b4d28ac0baa7f9211b5760436d2555c))
- Removed caches from GH Actions ([3b0308c](https://github.com/ArkForgeLabs/Astra/commit/3b0308c825ce343ba3252a2bfa4381faf67264f0))

## [0.4.0] - 2024-12-29

### Features

- Added responses ([c94b9f7](https://github.com/ArkForgeLabs/Astra/commit/c94b9f7daae79d221d7246be9d6ffa7c01f53c0b))
- The body now can have text or json processed ([9041831](https://github.com/ArkForgeLabs/Astra/commit/9041831eacff5e016b7cabbfe86402c8c6e613d1))
- Fixed parse_query and added table schema validation ([2dc0ddf](https://github.com/ArkForgeLabs/Astra/commit/2dc0ddfa8b3b9af00c7162144369786408312565))

## [0.3.5] - 2024-12-27

### Bug Fixes

- #2 ([30ee5f5](https://github.com/ArkForgeLabs/Astra/commit/30ee5f5eaf36061c6cad8ac33f32ec4f67d8ff7e))

### Miscellaneous Tasks

- Updated the versions ([c556150](https://github.com/ArkForgeLabs/Astra/commit/c5561507ff9f2f9fb5d227badbedc39680843c54))

## [0.3.4] - 2024-12-27

### Miscellaneous Tasks

- Attempt at chmod during build ([56172b3](https://github.com/ArkForgeLabs/Astra/commit/56172b3ba2a9bb0e96d3abe349b9683557f8f1e4))

## [0.3.3] - 2024-12-26

### Features

- Dotenv and pretty print ([013a0d1](https://github.com/ArkForgeLabs/Astra/commit/013a0d199b6a02bde1630e78f85ddf530da41e39))

## [0.3.2] - 2024-12-26

### Features

- Compression layer is now included but optionally enabled through settings ([216eb9e](https://github.com/ArkForgeLabs/Astra/commit/216eb9ea3b3111c3e02513919df95acfc3e8fc9a))
- Added static_file serving support ([721510f](https://github.com/ArkForgeLabs/Astra/commit/721510f4b596227e667b732dc2d5a529190761e7))
- Optional compression and added new route methods ([ae7b01e](https://github.com/ArkForgeLabs/Astra/commit/ae7b01e81af3b3d9a23c5fee79b152cae710d8b3))

## [0.3.0] - 2024-12-24

### Features

- Added static routes and compression ([0bd553b](https://github.com/ArkForgeLabs/Astra/commit/0bd553b816b1bd9a73f266d3d4f9425b0fbcfdc4))

### Miscellaneous Tasks

- Updated versions ([a53971e](https://github.com/ArkForgeLabs/Astra/commit/a53971e76ed099edf8a535af3d734bcb5d88162b))

## [0.2.0] - 2024-12-24

### Features

- Postgresql support added ([5d4a941](https://github.com/ArkForgeLabs/Astra/commit/5d4a941ad522d696514c4424a6bef5ec19e92691))
- Added database and examples ([512dd38](https://github.com/ArkForgeLabs/Astra/commit/512dd3848b6fa5749d73a1161745c5a2ffef4589))

### Miscellaneous Tasks

- Updated version ([167d323](https://github.com/ArkForgeLabs/Astra/commit/167d3230088ec3598d043ce7ad9f5086886ba074))

## [0.1.0] - 2024-12-23

### Features

- Added bundle export option ([cccc13c](https://github.com/ArkForgeLabs/Astra/commit/cccc13c438eb0d5db78263d35442ac3a333092ee))
- Added version ([126361c](https://github.com/ArkForgeLabs/Astra/commit/126361cc166c1c8f7a87147ef013495b094acf29))
- Added workflow and cleaned up the source code ([161ba3b](https://github.com/ArkForgeLabs/Astra/commit/161ba3bb0a6fcda804403452f5d3a12adf0f8b55))
- Improved request methods ([c9b95d4](https://github.com/ArkForgeLabs/Astra/commit/c9b95d4e9985ed225829ca43afe3ac8f11bcb030))
- Added license and description ([82819f9](https://github.com/ArkForgeLabs/Astra/commit/82819f943b9dbf83c0b357bac00ee4045e1bb23f))
- Performance improved and added README ([3860a22](https://github.com/ArkForgeLabs/Astra/commit/3860a2263bfac5f3639f6791089be4838a31d404))
- Added requests information ([132a6d0](https://github.com/ArkForgeLabs/Astra/commit/132a6d02116f899e3226217f0d0ebb9e598c5c9d))
- Workable version with returns ([dce9274](https://github.com/ArkForgeLabs/Astra/commit/dce92748ff321e6e60f9445048e406266edb3314))
