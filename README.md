# cargo-whatfeatures

[![Documentation][docs_badge]][docs]
[![Crates][crates_badge]][crates]

## Table of Contents

- [Installation](#install)
- [Notes on color](#notes-on-color)
- [Usage](#usage)
- [License](#license)

## Install

with cargo installed, simply do:

> cargo install cargo-whatfeatures

**Note** you can switch to using [rustls](https://docs.rs/rustls/latest/rustls/) by doing:

> cargo install cargo-whatfeatures --no-default-features --features "rustls"

By default it uses the [native-tls](https://docs.rs/native-tls/latest/native_tls/) backend

## Notes on color

if the `NO_COLOR` env-var has a value, all color will be disabled.

See https://no-color.org/

if the `WHATFEATURES_THHEME` env-var has a one of the values: `[colorful, basic, palette, none]` the theme will be overriden. This will still honor `NO_COLOR`

## Usage

```
cargo-whatfeatures 0.9.12
the `whatfeatures` command

    USAGE:
        cargo whatfeatures [FLAGS] [OPTIONS] <crate>

    FLAGS:
        -h, --help
            Prints help information

        -V, --version
            Displays the program name and version

        -d, --deps
            Display dependencies for the crate
            This will list the required dependencies

        -n, --no-features
            Disable listing the features for the crate

        -r, --restricted
            When used on a local workspace, also included private packages

        -t, --this-crate
            When used on a crate in a local workspace, don't traverse to the root
            Normally, if you're in a workspace member, it will traverse to the root
            and list all sibling crates as well. This flag disabled that behavior

        -l, --list
            List all versions for the crate.
            When using the `-y` option, yanked crates can be filtered.

        -s, --short
            Display only the name and latest version, such as foo = 0.1.2

        -v, --verbose
            When this is enabled, all 'implied' features will be listed.
            Also, optional dependencies will be listed. Optional deps are technically features.

        -o, --offline
            Don't connect to the internet, limits the availities of this.
            If the crate is in either cargo's local registry, or whatfeatures' cache
            then this will work normally, otherwise it'll give you a nice error.

        -j, --json
            This outputs JSON rather than the human readable format

        --print-cache-dir
            Prints out the path to the cache directory

        --purge
            Purges the local cache. The command will automatically clean up after
            itself if it sees the crate in the cargo local registry. If its not
            in the cargo registry, it'll download the crate from crates.io and place
            it in its cache.

            This flag causes that cache to become invalidated.

            The cache is located at these locations:
            * Linux: $XDG_CACHE_HOME/museun/whatfeatures
            * Windows: %LOCALAPPDATA/museun/whatfeatures
            * macOS: $HOME/Library/Caches/museun/whatfeatures

        --theme [basic, colorful]
            use this provided theme

    OPTIONS:
        -c, --color [always, auto, never]
            Attempts to use colors when printing as text [default: auto]
            *NOTE* When NO_COLOR is set to any value, all colors will be disabled

        -p, --pkgid <semver>
            A specific version to lookup. e.g. foo:0.7.1
            If this is not provided, then the latest crate is used.

        --manifest-path <PATH>
            A path to the Cargo.toml you want to read, locally.
            This can be the root directory to the crate/workspace, or an explicit path to a Cargo.toml
            Use this to read from a local crate, rather than a remote one.

        -y, --show-yanked <exclude, include, only>
            Shows any yanked versions when using `--list`. [default: exclude].
            When 'exclude' is provided, only active releases versions will be listed
            When 'include' is provided, the listing will include yanked versions along with active releases.
            When 'only' is provided, only yanked versions will be listed

    ARGS:
        <crate>  The name of the crate to retrieve information for.

                 If this is a path to a directory containing a Cargo.toml,
                 or the path to the Cargo.toml then it'll use that directory
                 as the crate to operate one

                 This is exclusive with -p, --pkgid and with --manifest-path.

    CONFIG:
        WHATFEATURES_THEME  [colorful, basic, palette, none]
                            This allows you to override the --theme flag with an environmental variable
```

This allows you to lookup a **specific** crate, at a **_specific_** version and get its **default** and **optional** features. It also allows listing the deps for the specified crate.

You can also use this on local crates and workspaces.

Usage: [example.md](./docs/example.md)

## License

`cargo-whatfeatures` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE][apache] and [LICENSE-MIT][mit] for details.

[docs_badge]: https://docs.rs/cargo-whatfeatures/badge.svg
[docs]: https://docs.rs/cargo-whatfeatures
[crates_badge]: https://img.shields.io/crates/v/cargo-whatfeatures.svg
[crates]: https://crates.io/crates/cargo-whatfeatures
[apache]: ./LICENSE-APACHE
[mit]: ./LICENSE-MIT
