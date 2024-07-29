# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-07-29

### Added 

#### Prototype functionality

- `load` default function.
- `Error` enum type.
- Main `EnvSafe` map type.
- `EnvSafeConfig` builder type.
  - `new` method.
  - `app_env_config` method.
  - `add_envfile` method.
  - `load` method.
- `AppEnvConfig` builder type.
  - `new` method.
  - `add_app_env` method.
- `EnvConfig` builder type.
  - `new` method.
  - `add_envfile` method.
  - `add_envfile_override` method.
  - `sequence` method.
  - `errors` method.
- `Error return` enum type.
- `EnvSequence` enum type.
- Defaults for public types.

[Unreleased]: https://github.com/sonro/envsafe/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sonro/envsafe/releases/tag/v0.1.0
