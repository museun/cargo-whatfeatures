## whatfeatures
```
Usage: whatfeatures [OPTIONS]

Positional arguments:
  name

Optional arguments:
  -h, --help             display this message
  -d, --deps             look up the dependencies for this crate instead     
  -v, --version VERSION  a specific version
  -l, --list             list all versions
  -s, --show-yanked      shows any yanked versions before the latest stable
  -j, --json             prints results as json
  -n, --no-color         disables using colors when printing as text      
  -c, --color            tries to use colors when printing as text (default)
```

This allows you to lookup a **specific** crate, at a **specific** version and get its **default** and **optional** features. It also allows listing the deps for the specified crate.

note: `--show-yanked` and `--list` do nothing when `--deps` is used

# Examples:

## look up the features for the latest version of a crate
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
## look up a specific version
>whatfeatures twitchchat -v 0.5.0
```
twitchchat/0.5.0
    default: all
    all: serde_hashbrown, parking_lot
    serde_hashbrown: serde, hashbrown/serde
```

## list all versions
>whatfeatures markings --list
```
markings/0.1.1
  no default features
markings/0.1.0
  no default features
```

## look up a specific version as json
>whatfeatures twitchchat -v 0.3.0 --json | jq .
```json
[
  {
    "features": {
      "all": [
        "serde_hashbrown",
        "parking_lot"
      ],
      "default": [
        "all"
      ],
      "serde_hashbrown": [
        "serde",
        "hashbrown/serde"
      ]
    },
    "name": "twitchchat",
    "version": "0.3.0",
    "yanked": false
  }
]
```

## display yanked releases that are newer than the current (e.g. `futures`)
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

## get the deps for the current release of a crate
>whatfeatures curl --deps
```
curl/0.4.22
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

## get the deps for a specific crate
>whatfeatures curl --deps -v 0.3.0
```
curl/0.3.0
  normal
    curl-sys    = ^0.2.0
    libc        = ^0.2
    openssl-sys = ^0.7.0 if cfg(all(unix, not(target_os = "macos")))      
  dev
    mio         = ^0.5
```

## get the deps for the current release of a crate
>whatfeatures curl --deps --json | jq .
```json
[
  {
    "name": "curl",
    "version": "0.4.22"
  },
  {
    "id": 751510,
    "version_id": 152547,
    "crate_id": "curl-sys",
    "req": "^0.4.18",
    "optional": false,
    "default_features": false,
    "features": [],
    "target": null,
    "kind": "normal"
  },
  {
    "id": 751517,
    "version_id": 152547,
    "crate_id": "kernel32-sys",
    "req": "^0.2.2",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": "cfg(target_env = \"msvc\")",
    "kind": "normal"
  },
  {
    "id": 751511,
    "version_id": 152547,
    "crate_id": "libc",
    "req": "^0.2.42",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": null,
    "kind": "normal"
  },
  {
    "id": 751515,
    "version_id": 152547,
    "crate_id": "openssl-probe",
    "req": "^0.1.2",
    "optional": true,
    "default_features": true,
    "features": [],
    "target": "cfg(all(unix, not(target_os = \"macos\")))",
    "kind": "normal"
  },
  {
    "id": 751516,
    "version_id": 152547,
    "crate_id": "openssl-sys",
    "req": "^0.9.43",
    "optional": true,
    "default_features": true,
    "features": [],
    "target": "cfg(all(unix, not(target_os = \"macos\")))",
    "kind": "normal"
  },
  {
    "id": 751518,
    "version_id": 152547,
    "crate_id": "schannel",
    "req": "^0.1.13",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": "cfg(target_env = \"msvc\")",
    "kind": "normal"
  },
  {
    "id": 751512,
    "version_id": 152547,
    "crate_id": "socket2",
    "req": "^0.3.7",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": null,
    "kind": "normal"
  },
  {
    "id": 751519,
    "version_id": 152547,
    "crate_id": "winapi",
    "req": "^0.2.7",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": "cfg(windows)",
    "kind": "normal"
  },
  {
    "id": 751513,
    "version_id": 152547,
    "crate_id": "mio",
    "req": "^0.6",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": null,
    "kind": "dev"
  },
  {
    "id": 751514,
    "version_id": 152547,
    "crate_id": "mio-extras",
    "req": "^2.0.3",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": null,
    "kind": "dev"
  }
]
```

## get the deps for a specific crate as json
>whatfeatures curl --deps -v 0.3.0 --json | jq .
```json
[
  {
    "name": "curl",
    "version": "0.3.0"
  },
  {
    "id": 87603,
    "version_id": 27715,
    "crate_id": "curl-sys",
    "req": "^0.2.0",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": null,
    "kind": "normal"
  },
  {
    "id": 87604,
    "version_id": 27715,
    "crate_id": "libc",
    "req": "^0.2",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": null,
    "kind": "normal"
  },
  {
    "id": 87606,
    "version_id": 27715,
    "crate_id": "openssl-sys",
    "req": "^0.7.0",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": "cfg(all(unix, not(target_os = \"macos\")))",
    "kind": "normal"
  },
  {
    "id": 87605,
    "version_id": 27715,
    "crate_id": "mio",
    "req": "^0.5",
    "optional": false,
    "default_features": true,
    "features": [],
    "target": null,
    "kind": "dev"
  }
]
```
