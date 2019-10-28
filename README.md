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
    - [list all name and version pairs including yanked versions](#list-all-name-and-version-pairs-including-yanked-versions)
    - [list all features for all versions](#list-all-features-for-all-versions)
  * [Dependencies](#dependencies)
    - [list the deps for the latest version](#list-the-deps-for-the-latest-version)
    - [list the deps for a specific version](#list-the-deps-for-a-specific-version)
  * [JSON output](#JSON-output)
    - [list the features for the latest version as json](#list-the-features-for-the-latest-version-as-json)
    - [list the features for a specific version as json](#list-the-features-for-a-specific-version-as-json)
    - [display yanked releases that are newer than the current as json](#display-yanked-releases-that-are-newer-than-the-current-as-json)
    - [list all name and version pairs as json](#list-all-name-and-version-pairs-as-json)
    - [list all name and version pairs including yanked versions as json](#list-all-name-and-version-pairs-including-yanked-versions-as-json)
    - [list all features for all versions as json](#list-all-features-for-all-versions-as-json)
    - [list the deps for the latest version as json](#list-the-deps-for-the-latest-version-as-json)
    - [list the deps for a specific version as json](#list-the-deps-for-a-specific-version-as-json)
## Install
with rustup installed, simply do:
```
cargo install whatfeatures
```

## Usage
```
USAGE:
    whatfeatures [FLAGS] [OPTIONS] <crate>

FLAGS:
    -d, --deps           Display dependencies for this crate
    -h, --help           Prints help information
    -j, --json           Use JSON as the output format. Defaults to a textual format
    -l, --list           List all versions for the crate
    -n, --no-features    Disable listing the features for the crate
    -s, --short          Display only the name and version, such as foo/0.1.2
    -y, --show-yanked    Shows any yanked versions. Defaults to hiding them

OPTIONS:
    -c, --color <bool>        Attempts to use colors when printing as text [default: true]
    -v, --version <semver>    A specific version to lookup. e.g. 0.7.1

ARGS:
    <crate>    The name of the crate to retrieve information for
```

This allows you to lookup a **specific** crate, at a ***specific*** version and get its **default** and **optional** features. It also allows listing the deps for the specified crate.

## Examples:
### Features
#### list the features for the latest version
>cargo whatfeatures serde
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
>cargo whatfeatures twitchchat -v 0.5.0
```
twitchchat/0.5.0
  default: all
  all: serde_hashbrown, parking_lot
  serde_hashbrown: serde, hashbrown/serde
```

#### display yanked releases that are newer than the current
>cargo whatfeatures futures --show-yanked
```
yanked: futures/0.2.3-docs-yank.4
  no default features
```

### Simple listing
### list all name and version pairs
>cargo whatfeatures --list --short lock-api
```
lock_api/0.3.1
lock_api/0.2.0
lock_api/0.1.5
lock_api/0.1.4
lock_api/0.1.3
lock_api/0.1.1
lock_api/0.1.0
```

#### list all name and version pairs, including yanked versions
>cargo whatfeatures --list --short --show-yanked lock-api
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
>cargo whatfeatures simple-logger --list
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
>cargo whatfeatures curl --deps
```
curl/0.4.22
  features
    default: ssl
    force-system-lib-on-osx: curl-sys/force-system-lib-on-osx
    http2: curl-sys/http2
    ssl: openssl-sys, openssl-probe, curl-sys/ssl
    static-curl: curl-sys/static-curl
    static-ssl: curl-sys/static-ssl
  dependencies
    normal
      openssl-probe   = ^0.1.2  if cfg(all(unix, not(target_os = "macos")))
      openssl-sys     = ^0.9.43 if cfg(all(unix, not(target_os = "macos")))
      optional
        curl-sys      = ^0.4.18
        kernel32-sys  = ^0.2.2  if cfg(target_env = "msvc")
        libc          = ^0.2.42
        schannel      = ^0.1.13 if cfg(target_env = "msvc")
        socket2       = ^0.3.7
        winapi        = ^0.2.7  if cfg(windows)
    dev
      mio             = ^0.6
      mio-extras      = ^2.0.3
```

#### list the deps for a specific version
**note** use `-f false` to not list the features
>cargo whatfeatures curl --deps -v 0.3.0
```
curl/0.3.0
  features
    no default features
  dependencies
    normal
      optional
        curl-sys    = ^0.2.0
        libc        = ^0.2
        openssl-sys = ^0.7.0 if cfg(all(unix, not(target_os = "macos")))
    dev
      mio         = ^0.5
```

### JSON output
#### list the features for a specific version as json
> whatfeatures serde -j | jq .
```json
{
  "name": "serde",
  "version": "1.0.97",
  "yanked": false,
  "features": {
    "std": [],
    "unstable": [],
    "alloc": [],
    "default": [
      "std"
    ],
    "rc": [],
    "derive": [
      "serde_derive"
    ]
  }
}
```

#### display yanked releases that are newer than the current as json
> whatfeatures twitchchat -v 0.5.0 -j | jq .
```json
{
  "name": "twitchchat",
  "version": "0.5.0",
  "yanked": false,
  "features": {
    "serde_hashbrown": [
      "serde",
      "hashbrown/serde"
    ],
    "default": [
      "all"
    ],
    "all": [
      "serde_hashbrown",
      "parking_lot"
    ]
  }
}
```

#### list all versions and features for a crate as json
> whatfeatures futures --show-yanked -j | jq .
```json
{
  "name": "futures",
  "version": "0.2.3-docs-yank.4",
  "yanked": true,
  "features": {}
}
```

#### list all name and version pairs as json
> whatfeatures --list --short lock-api -j | jq .
```json
{
  "lock_api": [
    {
      "version": "0.3.1",
      "yanked": false
    },
    {
      "version": "0.2.0",
      "yanked": false
    },
    {
      "version": "0.1.5",
      "yanked": false
    },
    {
      "version": "0.1.4",
      "yanked": false
    },
    {
      "version": "0.1.3",
      "yanked": false
    },
    {
      "version": "0.1.1",
      "yanked": false
    },
    {
      "version": "0.1.0",
      "yanked": false
    }
  ]
}
```

#### list all name and version pairs including yanked versions as json
> whatfeatures --list --short --show-yanked lock-api -j | jq .
```json
{
  "lock_api": [
    {
      "version": "0.3.1",
      "yanked": false
    },
    {
      "version": "0.3.0",
      "yanked": true
    },
    {
      "version": "0.2.0",
      "yanked": false
    },
    {
      "version": "0.1.5",
      "yanked": false
    },
    {
      "version": "0.1.4",
      "yanked": false
    },
    {
      "version": "0.1.3",
      "yanked": false
    },
    {
      "version": "0.1.2",
      "yanked": true
    },
    {
      "version": "0.1.1",
      "yanked": false
    },
    {
      "version": "0.1.0",
      "yanked": false
    }
  ]
}
```

#### list all features for all versions as json
> whatfeatures simple-logger --list -j | jq .
```json
{
  "simple_logger": [
    {
      "version": "1.3.0",
      "features": {
        "default": [
          "colored"
        ]
      },
      "yanked": false
    },
    {
      "version": "1.2.0",
      "features": {
        "default": [
          "colored"
        ]
      },
      "yanked": false
    },
    {
      "version": "1.1.0",
      "features": {},
      "yanked": false
    },
    {
      "version": "1.0.1",
      "features": {},
      "yanked": false
    },
    {
      "version": "1.0.0",
      "features": {},
      "yanked": false
    },
    {
      "version": "0.5.0",
      "features": {},
      "yanked": false
    },
    {
      "version": "0.4.0",
      "features": {},
      "yanked": false
    },
    {
      "version": "0.3.1",
      "features": {},
      "yanked": false
    },
    {
      "version": "0.3.0",
      "features": {},
      "yanked": false
    },
    {
      "version": "0.1.0",
      "features": {},
      "yanked": false
    },
    {
      "version": "0.0.2",
      "features": {},
      "yanked": false
    }
  ]
}
```

#### list the deps for the latest version as json
> whatfeatures curl --deps -j | jq .
```json
{
  "name": "curl",
  "version": "0.4.22",
  "yanked": false,
  "features": {
    "ssl": [
      "openssl-sys",
      "openssl-probe",
      "curl-sys/ssl"
    ],
    "force-system-lib-on-osx": [
      "curl-sys/force-system-lib-on-osx"
    ],
    "static-curl": [
      "curl-sys/static-curl"
    ],
    "default": [
      "ssl"
    ],
    "http2": [
      "curl-sys/http2"
    ],
    "static-ssl": [
      "curl-sys/static-ssl"
    ]
  },
  "dependencies": {
    "normal": [
      {
        "curl-sys": {
          "req": "^0.4.18",
          "optional": false,
          "default_features": false,
          "features": [],
          "target": null
        }
      },
      {
        "kernel32-sys": {
          "req": "^0.2.2",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": "cfg(target_env = \"msvc\")"
        }
      },
      {
        "libc": {
          "req": "^0.2.42",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": null
        }
      },
      {
        "schannel": {
          "req": "^0.1.13",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": "cfg(target_env = \"msvc\")"
        }
      },
      {
        "socket2": {
          "req": "^0.3.7",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": null
        }
      },
      {
        "winapi": {
          "req": "^0.2.7",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": "cfg(windows)"
        }
      },
      {
        "openssl-probe": {
          "req": "^0.1.2",
          "optional": true,
          "default_features": true,
          "features": [],
          "target": "cfg(all(unix, not(target_os = \"macos\")))"
        }
      },
      {
        "openssl-sys": {
          "req": "^0.9.43",
          "optional": true,
          "default_features": true,
          "features": [],
          "target": "cfg(all(unix, not(target_os = \"macos\")))"
        }
      }
    ],
    "dev": [
      {
        "mio": {
          "req": "^0.6",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": null
        }
      },
      {
        "mio-extras": {
          "req": "^2.0.3",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": null
        }
      }
    ]
  }
}
```

#### list the deps for a specific version as json
> whatfeatures curl --deps -v 0.3.0 -j | jq .
```json
{
  "name": "curl",
  "version": "0.3.0",
  "yanked": false,
  "features": {},
  "dependencies": {
    "normal": [
      {
        "curl-sys": {
          "req": "^0.2.0",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": null
        }
      },
      {
        "libc": {
          "req": "^0.2",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": null
        }
      },
      {
        "openssl-sys": {
          "req": "^0.7.0",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": "cfg(all(unix, not(target_os = \"macos\")))"
        }
      }
    ],
    "dev": [
      {
        "mio": {
          "req": "^0.5",
          "optional": false,
          "default_features": true,
          "features": [],
          "target": null
        }
      }
    ]
  }
}
```
