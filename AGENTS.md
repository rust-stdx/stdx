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
