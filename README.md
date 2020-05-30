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
  * [Simple listing](#simple-listing)
    - [get the latest version](#get-the-latest-version)
    - [list all name and version pairs](#list-all-name-and-version-pairs)
    - [list all name and version pairs, including yanked versions](#list-all-name-and-version-pairs-including-yanked-versions)
    - [list all name and version pairs, only showing yanked versions](#list-all-name-and-version-pairs-only-showing-yanked-versions)
  * [Dependencies](#dependencies)
    - [list the deps for the latest version](#list-the-deps-for-the-latest-version)
    - [list the deps for a specific version](#list-the-deps-for-a-specific-version)

## Install
with cargo installed, simply do:
> cargo install -f cargo-whatfeatures

**Note** -f will replace the previous installed version

## Notes on color
if the `NO_COLOR` env-var has a value, all color will be disabled.

See https://no-color.org/

## Usage
```
cargo-whatfeatures 0.8.0
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
        <crate>  The name of the remote crate to retrieve information for.
                 Using this means you want a 'remote' crate.
                 This is exclusive with -p, --pkgid and with --manifest-path.

```

This allows you to lookup a **specific** crate, at a ***specific*** version and get its **default** and **optional** features. It also allows listing the deps for the specified crate.

You can also use this on local crates.

## Examples:
### Features
#### list the features for the latest version
>cargo whatfeatures serde

or

>cargo whatfeatures -p serde
```
serde/1.0.110
features
├─ default
├─ alloc
├─ derive
├─ rc
├─ std
└─ unstable
```

#### list the features and optional deps for the latest version
> cargo whatfeatures serde -v

**Note** this also list 'implied features' (ones that are enabled by other features).
```
serde/1.0.111
features
├─ default
│ └─ std
├─ alloc
├─ derive
│ └─ serde_derive
├─ rc
├─ std
└─ unstable
optional dependencies
└─ serde_derive = = 1.0.111 
```

#### list the features for a specific version
>cargo whatfeatures -p twitchchat:0.10.2
```
twitchchat/0.10.2
features
├─ default
│ ├─ async
│ └─ tokio_native_tls
├─ async
├─ tokio_native_tls
└─ tokio_rustls
```

### Simple listing
#### get the latest version
>cargo whatfeatures --short lock-api
```
lock_api/0.3.4
```

#### list all name and version pairs
>cargo whatfeatures --list lock-api
```
lock-api/0.3.4
lock-api/0.3.3
lock-api/0.3.2
lock-api/0.3.1
lock-api/0.2.0
lock-api/0.1.5
lock-api/0.1.4
lock-api/0.1.3
lock-api/0.1.1
lock-api/0.1.0
```

#### list all name and version pairs, including yanked versions
>cargo whatfeatures --list --show-yanked include lock-api
```
lock-api/0.3.4
lock-api/0.3.3
lock-api/0.3.2
lock-api/0.3.1
yanked: lock-api/0.3.0
lock-api/0.2.0
lock-api/0.1.5
lock-api/0.1.4
lock-api/0.1.3
yanked: lock-api/0.1.2
lock-api/0.1.1
lock-api/0.1.0
```

#### list all name and version pairs, only showing yanked versions
>cargo whatfeatures --list --show-yanked only lock-api
```
yanked: lock-api/0.3.0
yanked: lock-api/0.1.2
```

### Dependencies
#### list the deps for the latest version
**Note** use `--no-features` (`-n`) to not list the features
>cargo whatfeatures curl --deps
```
curl/0.4.29
features
├─ default
│ └─ ssl
├─ force-system-lib-on-osx
├─ http2
├─ mesalink
├─ protocol-ftp
├─ spnego
├─ ssl
├─ static-curl
└─ static-ssl
required dependencies
├─ normal
│ ├─ for cfg(target_env = "msvc")
│ │ ├─ schannel = ^0.1.13 
│ │ └─ winapi = ^0.3 (has enabled features)
│ ├─ curl-sys = ^0.4.31 
│ ├─ libc = ^0.2.42 
│ └─ socket2 = ^0.3.7 
├─ development
│ ├─ mio = ^0.6 
│ └─ mio-extras = ^2.0.3 
└─ no build dependencies
```

#### list the deps for a specific version
**note** use `-f false` to not list the features
>cargo whatfeatures curl --deps -v 0.3.0
```
curl/0.3.0
no features
no optional dependencies
required dependencies
├─ normal
│ ├─ for cfg(all(unix, not(target_os = "macos")))
│ │ └─ openssl-sys = ^0.7.0
│ ├─ curl-sys = ^0.2.0
│ └─ libc = ^0.2
├─ development
│ └─ mio = ^0.5
└─ no build dependencies
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
