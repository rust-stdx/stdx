# Rust's (unofficial) extended standard library: simplicity, performance and supply chain security for everyone

Inspired by Go's extensive standard library, we are building Rust's extended standard library to bring simplicity, performance and supply chain security to every Rust developer.

Once the idea has proven valuable, we plan to donate the entire codebase to the Rust Foundation to build trust and drive adoption.


Learn more in the announcement post: https://kerkour.com/stdx


## Usage

To avoid namespace clashes and supply chain risks, we do not use a centralized package repository. Import packages directly from source, for example:

```toml
base64 = { git = "https://github.com/rust-stdx/stdx", branch = "main" }
# or, to pin a commit
base64 = { git = "https://github.com/rust-stdx/stdx", rev = "1234" }
# or, to use Codeberg
base64 = { git = "https://codeberg.org/rust-stdx/stdx", branch = "main" }
# or, fork it and use your own mirror
base64 = { git = "https://git.[my-organization].com/[username]/stdx", branch = "main" }
```

Contrary to what the name may suggest, most packages also support `no_std` environments by disabling the default `std` feature, and some also work without `alloc` by disabling the `alloc` feature. Look at the documentation of individual packages to learn more.


Other than crates in the `work_in_progress` folder, the `main` branch is considered stable. Minor breaking changes may happen from time to time, but we will do our best to reduce disruptions.



## Documentation

https://rust-stdx.github.io/stdx


## Partners


<table align="center">
  <tr>
    <td>
      <a href="https://github.com/pingooio/pingoo">
        <img src="https://avatars.githubusercontent.com/u/211801982" height="80" alt="Pingoo" /><br />
        Pingoo
      </a>
    </td>
  </tr>
</table>

`stdx` is only possible thanks to our awesome partners!

Join our partner program to invest in the future of Rust and get unfair advantages such as end-to-end supply chain traceability and security, priority support and roadmap prioritization. See [FUNDING.md](./FUNDING.md) or reach out at [code@pingoo.io](mailto:code@pingoo.io) to learn more.


## Contributing

Contributions are welcome, especially bug reports, improvement ideas and new package suggestions.

Except for minor typos, no pull request will be accepted without a preliminary discussion. Please open an issue first.

Commits need to be signed and merged (squashed).

### AI Policy

The use of AI-generated code is generally encouraged, **but**, AI-generated issues, text or any other form of communication will not be tolerated.

Please keep the size of your Pull / Merge requests reasonnable.

Like all humans, our time is precious. Be succinct, precise and use your brain.


## Development

See [.devcontainer/Dockerfile](./.devcontainer/Dockerfile).

Then:

```bash
rustup default stable
```

And you are ready to ~~Go~~ Rust!

See `Makefile` for the most common commands used during development.


## License

MIT ([LICENSE.txt](./LICENSE.txt))


### Forks

| Package | Forked from | Commit | Original License |
| --- | --- | --- | --- |
| `bel` | https://github.com/cel-rust/cel-rust | 8287d04156a1b31efe0dd53db78e943fef15c59a | MIT |
| `cron` | https://github.com/zslayton/cron | ?? | MIT |
| `acme` | https://github.com/instant-labs/instant-acme | 5e12971830a5907f0aeba4dfd602ec26db4bc30c | Apache 2.0 |
| `anyerr` | https://github.com/dtolnay/anyhow | 5a88bc48ca18c9720be292487dcdcbc93004d15a | MIT |
| `constant_time_eq` | https://github.com/cesarb/constant_time_eq | 09a34625babf29e1b622ed46e959ea517986b12a | CC0-1.0 |
| `crc32fast` | https://github.com/srijs/rust-crc32fast | 479ecdf0174dd3a0f7d48b2f66a386d8d2369963 | MIT |
| `embed` | https://github.com/pyrossh/rust-embed | 105fdfebab5820ea0628149ee62b34f6d2df3bb8 | MIT |
| `derivative` | https://github.com/mcarton/rust-derivative | 5179a968ca6d70792f62dfe6727ab8d5b8b5cf5e | MIT |
| `form_urlencoded` | https://github.com/servo/rust-url | 54346fa288e16b25b71c45149d7067c752b450e0 | MIT |
| `httpdate` | https://github.com/pyfisch/httpdate | 63f723c6eae30ec130a6c5625ec38c4b49b0891c | MIT |
| `ipnetwork` | https://github.com/achanda/ipnetwork | f01575cbf2fc596c0a1761c122aa92525cbb7974 | MIT |
| `itoa` | https://github.com/dtolnay/itoa | 945f297a243887f66407fcd65088b3713a464851 | MIT |
| `maxminddb` | https://github.com/oschwald/maxminddb-rust | b5a6ccc2f1c8e990b54bbac648f524cdf043522a | ISC |
| `memmem` | https://github.com/jneem/memmem | d6e6a0b1fb391539cf8511e7a2de76016d86a870 | MIT |
| `mimeguess` | https://github.com/abonander/mime_guess | 1ae11679916b18fcced93c11104b7aed53bd35a2 | MIT |
| `num_cpus` | https://github.com/seanmonstar/num_cpus | 7c03fc930cc47a9b94e8ca66ca44ef1a454c8f51 | MIT |
| `percent_encoding` | https://github.com/servo/rust-url | 54346fa288e16b25b71c45149d7067c752b450e0 | MIT |
| `ryu` | https://github.com/dtolnay/ryu | 8234c4d95f97565bfa562cd1572bb0e8ed80cc44 | Apache 2.0 |
| `serde_urlencoded` | https://github.com/nox/serde_urlencoded | 0cca840185fa85b39e2cc8a0b2547fff5ace8e68 | MIT |
| `serde_yaml` | https://github.com/dtolnay/serde-yaml | 2009506d33767dfc88e979d6bc0d53d09f941c94 | MIT |
| `tld` | https://github.com/rushmorem/publicsuffix | 47958d65a3eab3a01e4a9cf46ccf40c11a7e8052 | MIT |
| `unsafe_libyaml` | https://crates.io/crates/unsafe-libyaml | 417668ce6565ece14bbd9b4a73137d9241ea1365 | MIT |
