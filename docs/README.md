# semver-4-winres

Parser of [SemVer](https://semver.org/), for use with [winres](https://crates.io/crates/winres) specifically.

## Usage

> The crate can be found on [crates.io/crates/semver-4-winres](https://crates.io/crates/semver-4-winres)

### Import Using Cargo

> **Note**
>
> This crate is meant _primarily_ for use in build scripts.
> Example:
> ```toml
> [build-dependencies]
> semver-4-winres = "0.0.0" # Check for latest released version
> ```

### External References

For future reference, whether to understand how this project was created, or how future Rust projects could be created.

- [Rust Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html) (`Cargo.toml`)
- [Building & Testing Rust](https://docs.github.com/en/actions/tutorials/build-and-test-code/rust) (`.github/workflows/rust.yml`)
- [Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html) (`.github/workflows/rust-publish.yml`)
