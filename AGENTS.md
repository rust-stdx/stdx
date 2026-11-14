When working in this repository, read and follow the stdx project skill at
`skills/stdx/SKILL.md`. It covers how to use stdx crates (import from git,
never crates.io), the full crate catalog, and the zero-external-dependency
policy.

## Contributor guidelines

### Writing and updating code

Measure, don't assume (especially when asked about performance, edge cases...).

Always document code but without necessarily explaining the internals of the
functions, instead, explain what the user can expect, and the situations where
it can returns an error / panic (if it applies).

Don't create READMEs for packages, instead produce clear package-level documentation.

When creating a new library named `mylib` (for example), don't use the usual
`mylib/src/lib.rs` architecture, but instead use `mylib/mylib.rs` and update
the `[lib]` field in `Cargo.toml`.

When adding a workspace member to the root Cargo.toml, ensure that the
workspace members are alphabetically ordered.

When reviewing a package, always ensure that the documentation is up-to-date and correct.

### Cargo.toml conventions

Only external dependencies belong in the workspace-level
`[workspace.dependencies]`. Packages in this workspace must never be listed
there; they are imported by path from other packages in the workspace.

Packages must declare external dependencies through the workspace, using
`dependency = { workspace = true }`. No package may declare an external
dependency directly (with its own `version` or `git` source).

In every dependency table (`[dependencies]`, `[dev-dependencies]`,
`[build-dependencies]`, and target-specific dependency tables), list the
workspace-local path dependencies first, alphabetically, then a blank line,
then the external workspace dependencies, alphabetically:

```toml
[dependencies]
# workspace-local packages, imported by path
base64 = { path = "../base64" }
crypto = { path = "../crypto" }

# external dependencies, imported from [workspace.dependencies]
bytes = { workspace = true }
serde = { workspace = true }
```

Every package's `[package]` table must contain only the following fields, in
this order:

```toml
[package]
name = "alloc_counter"
version = "0.1.0"
description = "Per-thread allocation counting for tests."
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
```

The `description` field is required. Shared package metadata is defined in the
root `[workspace.package]` table (`edition`, `rust-version`, `license`, and
`repository`).

After doing changes, run the tests and checks only for the updated code. Running tests and all checks may be very slow and will be run by CI/CD, but sensure that `make lint` passes.
