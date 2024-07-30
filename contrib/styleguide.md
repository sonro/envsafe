# envsafe Styleguide

When contributing, please follow these coding standards.

## Format

- All code must be formatted with the default `rustfmt` settings.
- File structure:
  - Imports
  - Modules
  - Exports

## Modules

- Modules are not to be exposed outside the crate.
- Top-level files (`mod.rs`, `lib.rs`) must not contain any logic.
- Top-level files must explicitly export their contents (`pub use ...`).

## Imports

- All imports must be on a single line.
- Imports must be sorted in alphabetical order.
- Import groups must be separated by a blank line in this sequence:
  - Standard library.
  - External crates.
  - Local crate.

```rust
use std::collections::HashMap;
use std::path::Path;

use ahash::RandomState;

use crate::module::Type;
use crate::module::Type2;
```

## Exports

- All exports must be on a single line.
- Exports must be sorted in alphabetical order.
