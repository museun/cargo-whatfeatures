# cargo-whatfeatures
[![Documentation][docs_badge]][docs]
[![Crates][crates_badge]][crates]

## Table of Contents
- [Install](#install)
- [Notes on color](#notes-on-color)
- [Usage](#usage)
- [License](#license)
- [Examples](#examples)
  * [Features](#features)
    - [list the features for the latest version](#list-the-features-for-the-latest-version)
    - [list the features for a specific version](#list-the-features-for-a-specific-version)
    - [list the features for a local crate](#list-the-features-for-a-local-crate)
  * [Simple listing](#simple-listing)
    - [get the latest version](#get-the-latest-version)
    - [list all name and version pairs](#list-all-name-and-version-pairs)
    - [list all name and version pairs, including yanked versions](#list-all-name-and-version-pairs-including-yanked-versions)
    - [list all name and version pairs, only showing yanked versions](#list-all-name-and-version-pairs-only-showing-yanked-versions)
  * [Dependencies](#dependencies)
    - [list the deps for the latest version](#list-the-deps-for-the-latest-version)
    - [list the deps for a specific version](#list-the-deps-for-a-specific-version)
    - [list the deps for a local crate](#list-the-deps-for-a-local-crate)

## Install
with cargo installed, simply do:
> cargo install cargo-whatfeatures

**Note** you can switch to using [rustls](https://docs.rs/rustls/latest/rustls/) by doing:
> cargo install cargo-whatfeatures --no-default-features --features "rustls"

By default it uses the [native-tls](https://docs.rs/native-tls/latest/native_tls/) backend

## Notes on color
if the `NO_COLOR` env-var has a value, all color will be disabled.

See https://no-color.org/

## Usage
```
cargo-whatfeatures 0.9.0
he `whatfeatures` command

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

        -l, --list
            List all versions for the crate.
            When using the `-y` option, yanked crates can be filtered.

        -s, --short
            Display only the name and latest version, such as foo/0.1.2

        -v, --verbose
            When this is enabled, all 'implied' features will be listed.
            Also, optional dependencies will be listed. Optional deps are technically features.

        -o, --offline
            Don't connect to the internet, limits the availities of this.
            If the crate is in either cargo's local registry, or whatfeatures' cache
            then this will work normally, otherwise it'll give you a nice error.

        --print-cache-dir
            Prints out the path to the cache directory

        --purge
            Purges the local cache. The command will automatically clean up after
            itself if it sees the crate in the cargo local registry. If its not
            in the cargo registry, it'll download the crate from crates.io and place
            it in its cache. This flag causes that cache to become invalidated.

            The cache is located at these locations:
            * Linux: $XDG_CACHE_HOME/museun/whatfeatures
            * Windows: %LOCALAPPDATA/museun/whatfeatures
            * macOS: $HOME/Library/Caches/museun/whatfeatures

    OPTIONS:
        -c, --color [always, auto, never]
            Attempts to use colors when printing as text [default: auto]
            *NOTE* When NO_COLOR is set to any value, all colors will be disabled

        -p, --pkgid <semver>
            A specific version to lookup. e.g. 0.7.1
            If this is not provided, then the latest crate is used.

        --manifest-path <PATH>
            A path to the Cargo.toml you want to read, locally.
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

```

This allows you to lookup a **specific** crate, at a ***specific*** version and get its **default** and **optional** features. It also allows listing the deps for the specified crate.

You can also use this on local crates and workspaces.

## Examples:
### Features
#### list the features for the latest version
>cargo whatfeatures serde

or

>cargo whatfeatures -p serde
```
serde = 1.0.114
└─ features
  ├─ default
  │ └─ std
  ├─ alloc
  ├─ derive
  ├─ rc
  ├─ std (default)
  └─ unstable
```

The `(default)` will let you easily reference what indv. features are included in the default.

#### list the features and optional deps for the latest version
> cargo whatfeatures serde -v

**Note** this also list 'implied features' (ones that are enabled by other features).
```
serde = 1.0.114
├─ features
│ ├─ default
│ │ └─ std
│ ├─ alloc
│ ├─ derive
│ │ └─ serde_derive
│ ├─ rc
│ ├─ std (default)
│ └─ unstable
└─ optional dependencies
  └─ serde_derive = = 1.0.114
```

#### list the features for a specific version
>cargo whatfeatures -p twitchchat:0.10.2
```
twitchchat = 0.10.2
└─ features
  ├─ default
  │ ├─ async
  │ └─ tokio_native_tls        
  ├─ async (default)
  ├─ tokio_native_tls (default)
  └─ tokio_rustls
```

### list the features for a local crate
>cargo whatfeatures --manifest-path .
```
cargo-whatfeatures = 0.8.3
└─ features
  ├─ default
  │ └─ native-tls
  ├─ native-tls (default) 
  └─ rustls
```

The command is somewhat smart, if you give it a local directory or the path to a Cargo.toml and it doesn't look like a remote crate, it'll use that. 

So the above could be expressed as `cargo whatfeatures .` or `cargo whatfeatures ~/p/foobar`

`--manifest-path` is a way to ensure it uses the local crate rather than an unfournate similarly named crate on crates.io.

### Simple listing
#### get the latest version
>cargo whatfeatures --short lock-api
```
lock_api = 0.4.0
```

#### list all name and version pairs
>cargo whatfeatures --list lock-api
```
lock_api = 0.4.0
lock_api = 0.3.4
lock_api = 0.3.3
lock_api = 0.3.2
lock_api = 0.3.1
lock_api = 0.2.0
lock_api = 0.1.5
lock_api = 0.1.4
lock_api = 0.1.3
lock_api = 0.1.1
lock_api = 0.1.0
```

#### list all name and version pairs, including yanked versions
>cargo whatfeatures --list --show-yanked include lock-api
```
lock_api = 0.4.0
lock_api = 0.3.4
lock_api = 0.3.3
lock_api = 0.3.2
lock_api = 0.3.1
lock_api = 0.3.0 # yanked
lock_api = 0.2.0
lock_api = 0.1.5
lock_api = 0.1.4
lock_api = 0.1.3
lock_api = 0.1.2 # yanked
lock_api = 0.1.1
lock_api = 0.1.0
```

#### list all name and version pairs, only showing yanked versions
>cargo whatfeatures --list --show-yanked only lock-api
```
lock_api = 0.3.0 # yanked
lock_api = 0.1.2 # yanked
```

### Dependencies
#### list the deps for the latest version
**Note** use `--no-features` (`-n`) to not list the features
>cargo whatfeatures curl --deps
```
curl = 0.4.30
├─ features
│ ├─ default
│ │ └─ ssl
│ ├─ force-system-lib-on-osx
│ ├─ http2
│ ├─ mesalink
│ ├─ protocol-ftp
│ ├─ spnego
│ ├─ ssl (default)
│ ├─ static-curl
│ └─ static-ssl
└─ required dependencies
  ├─ normal
  │ ├─ for cfg(target_env = "msvc")
  │ │ ├─ schannel = ^0.1.13
  │ │ └─ winapi = ^0.3 (has enabled features)
  │ ├─ curl-sys = ^0.4.32
  │ ├─ libc = ^0.2.42
  │ └─ socket2 = ^0.3.7
  ├─ development
  │ ├─ anyhow = ^1.0.31
  │ ├─ mio = ^0.6
  │ └─ mio-extras = ^2.0.3
  └─ no build dependencies
```

#### list the deps for a specific version
**note** use `-n`,`--no-features` to not list the features
>cargo whatfeatures -p curl:0.3.0 --deps
```
curl = 0.3.0
├─ no features
└─ required dependencies
  ├─ normal
  │ ├─ for cfg(all(unix, not(target_os = "macos")))
  │ │ └─ openssl-sys = ^0.7.0
  │ ├─ curl-sys = ^0.2.0
  │ └─ libc = ^0.2
  ├─ development
  │ └─ mio = ^0.5
  └─ no build dependencies
```

### list the deps for a local crate
>cargo whatfeatures --manifest-path . -d -n
```
cargo-whatfeatures = 0.8.3
└─ required dependencies
  ├─ normal
  │ ├─ anyhow = ^1.0.31
  │ ├─ attohttpc = ^0.15.0 (has enabled features)
  │ ├─ cargo_metadata = ^0.10.0
  │ ├─ crate_version_parse = ^0.2.0
  │ ├─ directories-next = ^1.0.1
  │ ├─ flate2 = ^1.0.16
  │ ├─ home = ^0.5.3
  │ ├─ pico-args = ^0.3.3
  │ ├─ serde = ^1.0.114 (has enabled features)
  │ ├─ tar = ^0.4.29
  │ └─ yansi = ^0.5.0
  ├─ no development dependencies
  └─ no build dependencies
```

### example of scrying a workspace
> cargo whatfeatures ~/dev/godot-rust
```
workspace for godot-rust
├─ gdnative = 0.8.1
│ └─ features
│   ├─ default
│   │ └─ bindings
│   ├─ bindings (default)
│   └─ gd_test
├─ gdnative-bindings = 0.8.1
│ └─ features
│   ├─ no default features
│   └─ formatted
├─ gdnative-core = 0.8.1
│ └─ features
│   ├─ default
│   │ └─ nativescript
│   ├─ gd_test
│   └─ nativescript (default)
├─ gdnative-derive = 0.8.1
│ └─ no features
├─ gdnative-impl-proc-macros = 0.9.0  
│ └─ no features
├─ gdnative-sys = 0.8.1
│ └─ no features
└─ gdnative_bindings_generator = 0.8.1
  └─ features
    ├─ no default features
    └─ debug
```

using the `-r`, `--restricted` will also list packages that are set to private
> cargo whatfeatures -r ~/dev/godot-rust
```
workspace for godot-rust
├─ dodge_the_creeps = 0.1.0 (restricted)
│ └─ no features
├─ gdnative = 0.8.1
│ └─ features
│   ├─ default
│   │ └─ bindings
│   ├─ bindings (default)
│   └─ gd_test
├─ gdnative-bindings = 0.8.1
│ └─ features
│   ├─ no default features
│   └─ formatted
├─ gdnative-core = 0.8.1
│ └─ features
│   ├─ default
│   │ └─ nativescript
│   ├─ gd_test
│   └─ nativescript (default)
├─ gdnative-derive = 0.8.1
│ └─ no features
├─ gdnative-impl-proc-macros = 0.9.0
│ └─ no features
├─ gdnative-sys = 0.8.1
│ └─ no features
├─ gdnative-test = 0.1.0 (restricted)
│ └─ no features
├─ gdnative_bindings_generator = 0.8.1
│ └─ features
│   ├─ no default features
│   └─ debug
├─ hello_world = 0.1.0 (restricted)
│ └─ no features
├─ scene_create = 0.1.0 (restricted)
│ └─ no features
├─ signals = 0.1.0 (restricted)
│ └─ no features
└─ spinning_cube = 0.1.0 (restricted)
  └─ no features
```


## License
`cargo-whatfeatures` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE][APACHE] and [LICENSE-MIT][MIT] for details.

[docs_badge]: https://docs.rs/cargo-whatfeatures/badge.svg
[docs]: https://docs.rs/cargo-whatfeatures
[crates_badge]: https://img.shields.io/crates/v/cargo-whatfeatures.svg
[crates]: https://crates.io/crates/cargo-whatfeatures

[APACHE]: ./LICENSE-APACHE
[MIT]: ./LICENSE-MIT
