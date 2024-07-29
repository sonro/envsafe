# envsafe (WIP)

[![Crates.io](https://img.shields.io/crates/v/envsafe.svg)](https://crates.io/crates/envsafe)
[![msrv
1.80.0](https://img.shields.io/badge/msrv-1.80.0-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.80.0)
[![Documentation](https://img.shields.io/docsrs/envsafe?logo=docs.rs)](https://docs.rs/envsafe/)
[![license](https://img.shields.io/crates/l/envsafe.svg)](#license)

Load `.env` files in Rust, without using unsafe!

**WORK IN PROGRESS | DO NOT USE**

```sh
# .env
SOME_ENV_VAR='Hello envsafe'
```

```rust
// main.rs
use std::error::Error as StdError;

fn main() -> Result<(), Box<dyn StdError>> {
    let safe = envsafe::load()?;

    if let Some(var) = safe.get("SOME_ENV_VAR") {
        println!("SOME_ENV_VAR: {var}");
    }
}
```

## Features

`envsafe` approaches the environment as **readonly** - combining existing
variables with dotenv files in a single map. It is therefore **thread-safe**.

### Default behavior

- Load all the environment variables
- Load `.env` file if it exists
  - Error if `.env` has syntax errors
  - Do not override existing variables

### Configuration

Customize how the `EnvSafe` is loaded using [`EnvSafeConfig`]. You can specify:

- Multiple `.env` files
- Overriding behavior
- Sequence of loading
- Error ignoring
- Multiple strategies based on the environment

The following example always tries to load `common.env`. 

If the variable `APP_ENV` is set to `DEV`, it will first load the environment,
then `dev.env`. It will fail if the file does not exist or has syntax errors
and will override all existing variables.

If `APP_ENV` is set to `PROD`, it will load `prod.env` AND THEN load the
environment. All errors will be ignored.

```rust
EnvSafeConfig::<MAX_ENV_FILES, MAX_APP_ENVS>::new()
    .add_envfile("common.env")
    .app_env_config(
        AppEnvConfig::new("APP_ENV")
            .add_app_env(
                "DEV",
                EnvConfig::new()
                    .add_envfile_override("dev.env")
                    .sequence(EnvSequence::EnvThenFiles)
                    .errors(ErrorReturn::All),
            )
            .add_app_env(
                "PROD",
                EnvConfig::new()
                    .add_envfile("prod.env")
                    .sequence(EnvSequence::FilesThenEnv)
                    .errors(ErrorReturn::None),
            ),
    )
    .load()
    .unwrap();
```

### Allocations

Only the `EnvSafe` is allocated on the heap. All the configurations are kept
on the stack and are dropped when the `EnvSafe` is loaded. This is why we
specify `MAX_ENV_FILES` and `MAX_APP_ENVS` in the example above.

## Comparison

[`dotenvy`](https://github.com/allan2/dotenvy) currently modifies the
environment, which is unsafe in a multithreaded program on Unix-based systems.
This safety is trivial to enforce, but if you do not want to use `unsafe`:
`envsafe` may be a better choice. See
[this issue](https://github.com/allan2/dotenvy/issues/112) for more details.

## Contributing

**Thank you for considering contributing to envsafe!**

We welcome any form of contribution:

- New issues (feature requests, bug reports, questions, ideas, ...)
- Pull requests (documentation improvements, code improvements, new features,
  ...)

**Note**: Before you take the time to open a pull request, please open an issue
first.

## License

`envsafe` is distributed under the [MIT License](LICENSE).

