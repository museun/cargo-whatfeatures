# whatfeatures
## Table of Contents
- [Install](#install)
- [Usage](#usage)
- [Examples](#examples)
  * [Features](#features)
    - [list the features for the latest version](#list-the-features-for-the-latest-version)
    - [list the features for a specific version](#list-the-features-for-a-specific-version)
    - [display yanked releases that are newer than the current](#display-yanked-releases-that-are-newer-than-the-current)
  * [Simple listing](#simple-listing)
    - [list all name and version pairs](#list-all-name-and-version-pairs)
    - [list all features for all versions](#list-all-features-for-all-versions)
  * [Dependencies](#dependencies)
    - [list the deps for the latest version](#list-the-deps-for-the-latest-version)
    - [list the deps for a specific version](#list-the-deps-for-a-specific-version)

## Install
with rustup installed, simply do:
```
cargo install --git https://github.com/museun/whatfeatures -f
```
or
```
git clone https://github.com/museun/whatfeatures
cd whatfeatures
cargo install --path . -f
```

## Usage
```
Usage: whatfeatures [OPTIONS]

Positional arguments:
  name

Optional arguments:
  -h, --help             display this message
  -d, --deps             look up the dependencies for this crate
  -v, --version VERSION  a specific version
  -f, --features bool    displays the features (default: true)
  -s, --short            list only the name/version for the crate
  -l, --list             list all versions
  -y, --show-yanked      shows any yanked versions before the latest stable
  -n, --no-color         disables using colors when printing as text
  -c, --color            tries to use colors when printing as text (default: true)
```

This allows you to lookup a **specific** crate, at a **specific** version and get its **default** and **optional** features. It also allows listing the deps for the specified crate.

## Examples:
### Features
#### list the features for the latest version
>whatfeatures serde
```
serde/1.0.97
    default: std
    alloc
    derive: serde_derive
    rc
    std
    unstable
```

#### list the features for a specific version
>whatfeatures twitchchat -v 0.5.0
```
twitchchat/0.5.0
    default: all
    all: serde_hashbrown, parking_lot
    serde_hashbrown: serde, hashbrown/serde
```

#### list the features for the latest version as json
>whatfeatures markings --list
```
markings/0.1.1
    no default features
markings/0.1.0
    no default features
```

#### display yanked releases that are newer than the current
>whatfeatures futures --show-yanked
```
yanked: futures/0.2.3-docs-yank.4
yanked: futures/0.2.3-docs-yank.3
yanked: futures/0.2.3-docs-yank.2
yanked: futures/0.2.3-docs-yank
yanked: futures/0.2.1
yanked: futures/0.2.0
yanked: futures/0.2.0-beta
yanked: futures/0.2.0-alpha
futures/0.1.28
    default: use_std, with-deprecated
    nightly
    use_std
    with-deprecated
```

### Simple listing
#### list all name and version pairs
>whatfeatures --list --short --show-yanked lock-api
```
lock_api/0.3.1
yanked: lock_api/0.3.0
lock_api/0.2.0
lock_api/0.1.5
lock_api/0.1.4
lock_api/0.1.3
yanked: lock_api/0.1.2
lock_api/0.1.1
lock_api/0.1.0
```

#### list all features for all versions
>whatfeatures simple-logger --list
```
simple_logger/1.3.0
    default: colored
simple_logger/1.2.0
    default: colored
simple_logger/1.1.0
    no default features
simple_logger/1.0.1
    no default features
simple_logger/1.0.0
    no default features
simple_logger/0.5.0
    no default features
simple_logger/0.4.0
    no default features
simple_logger/0.3.1
    no default features
simple_logger/0.3.0
    no default features
simple_logger/0.1.0
    no default features
simple_logger/0.0.2
    no default features
```

### Dependencies
#### list the deps for the latest version
**note** use `-f false` to not list the features
>whatfeatures curl --deps
```
curl/0.4.22
    default: ssl
    force-system-lib-on-osx: curl-sys/force-system-lib-on-osx
    http2: curl-sys/http2
    ssl: openssl-sys, openssl-probe, curl-sys/ssl
    static-curl: curl-sys/static-curl
    static-ssl: curl-sys/static-ssl
  normal
    curl-sys      = ^0.4.18
    kernel32-sys  = ^0.2.2  if cfg(target_env = "msvc")
    libc          = ^0.2.42
    openssl-probe = ^0.1.2  if cfg(all(unix, not(target_os = "macos")))
    openssl-sys   = ^0.9.43 if cfg(all(unix, not(target_os = "macos")))
    schannel      = ^0.1.13 if cfg(target_env = "msvc")
    socket2       = ^0.3.7
    winapi        = ^0.2.7  if cfg(windows)
  dev
    mio           = ^0.6
    mio-extras    = ^2.0.3
```

#### list the deps for a specific version
**note** use `-f false` to not list the features
>whatfeatures curl --deps -v 0.3.0
```
curl/0.3.0
    no default features
  normal
    curl-sys    = ^0.2.0
    libc        = ^0.2
    openssl-sys = ^0.7.0 if cfg(all(unix, not(target_os = "macos")))
  dev
    mio         = ^0.5
```
