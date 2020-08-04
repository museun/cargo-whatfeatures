## Table of Contents
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
    - [list the deps and transitive features](#list_the_deps_and_transitive_features)

    
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
>cargo whatfeatures serde -v

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
  └─ serde_derive = =1.0.114
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
cargo-whatfeatures = 0.9.2
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
lock_api = 0.4.1 # 4 weeks ago
```

#### get the latest version, with timestamp
>cargo whatfeatures --verbose --short lock-api
```
lock_api = 0.4.1 # 2020-07-06 08:10:00 +0000
```

#### list all name and version pairs
>cargo whatfeatures --list lock-api
```
lock_api = 0.4.1 # 4 weeks ago
lock_api = 0.4.0 # 5 weeks ago
lock_api = 0.3.4 # 16 weeks ago
lock_api = 0.3.3 # 30 weeks ago
lock_api = 0.3.2 # 36 weeks ago
lock_api = 0.3.1 # 55 weeks ago
lock_api = 0.2.0 # 65 weeks ago
lock_api = 0.1.5 # 89 weeks ago
lock_api = 0.1.4 # 96 weeks ago
lock_api = 0.1.3 # 111 weeks ago
lock_api = 0.1.1 # 112 weeks ago
lock_api = 0.1.0 # 112 weeks ago
```

#### list all name and version pairs, with timestamp
>cargo whatfeatures --verbose --list lock-api
```
lock_api = 0.4.1 # 2020-07-06 08:10:00 +0000
lock_api = 0.4.0 # 2020-06-23 18:35:22 +0000
lock_api = 0.3.4 # 2020-04-10 16:18:43 +0000
lock_api = 0.3.3 # 2020-01-04 10:54:14 +0000
lock_api = 0.3.2 # 2019-11-25 21:16:41 +0000
lock_api = 0.3.1 # 2019-07-14 12:55:33 +0000
lock_api = 0.2.0 # 2019-05-04 09:29:36 +0000
lock_api = 0.1.5 # 2018-11-18 21:58:31 +0000
lock_api = 0.1.4 # 2018-09-25 22:03:20 +0000
lock_api = 0.1.3 # 2018-06-18 11:40:56 +0000
lock_api = 0.1.1 # 2018-06-08 00:17:16 +0000
lock_api = 0.1.0 # 2018-06-08 00:16:14 +0000
```

#### list all name and version pairs, including yanked versions
>cargo whatfeatures --list --show-yanked include lock-api
```
lock_api = 0.4.1 # 4 weeks ago
lock_api = 0.4.0 # 5 weeks ago
lock_api = 0.3.4 # 16 weeks ago
lock_api = 0.3.3 # 30 weeks ago
lock_api = 0.3.2 # 36 weeks ago
lock_api = 0.3.1 # 55 weeks ago
lock_api = 0.3.0 # 56 weeks ago -- yanked
lock_api = 0.2.0 # 65 weeks ago
lock_api = 0.1.5 # 89 weeks ago
lock_api = 0.1.4 # 96 weeks ago
lock_api = 0.1.3 # 111 weeks ago
lock_api = 0.1.2 # 111 weeks ago -- yanked
lock_api = 0.1.1 # 112 weeks ago
lock_api = 0.1.0 # 112 weeks ago
```

#### list all name and version pairs, including yanked versions, with timestamp
>cargo whatfeatures --verbose --list --show-yanked include lock-api
```
lock_api = 0.4.1 # 2020-07-06 08:10:00 +0000
lock_api = 0.4.0 # 2020-06-23 18:35:22 +0000
lock_api = 0.3.4 # 2020-04-10 16:18:43 +0000
lock_api = 0.3.3 # 2020-01-04 10:54:14 +0000
lock_api = 0.3.2 # 2019-11-25 21:16:41 +0000
lock_api = 0.3.1 # 2019-07-14 12:55:33 +0000
lock_api = 0.3.0 # 2019-07-03 11:21:03 +0000 -- yanked
lock_api = 0.2.0 # 2019-05-04 09:29:36 +0000
lock_api = 0.1.5 # 2018-11-18 21:58:31 +0000
lock_api = 0.1.4 # 2018-09-25 22:03:20 +0000
lock_api = 0.1.3 # 2018-06-18 11:40:56 +0000
lock_api = 0.1.2 # 2018-06-18 02:07:52 +0000 -- yanked
lock_api = 0.1.1 # 2018-06-08 00:17:16 +0000
lock_api = 0.1.0 # 2018-06-08 00:16:14 +0000
```

#### list all name and version pairs, only showing yanked versions
>cargo whatfeatures --list --show-yanked only lock-api
```
lock_api = 0.3.0 # 56 weeks ago -- yanked
lock_api = 0.1.2 # 111 weeks ago -- yanked
```

#### list all name and version pairs, only showing yanked versions, with timestamp
>cargo whatfeatures --list --show-yanked only lock-api
```
lock_api = 0.3.0 # 2019-07-03 11:21:03 +0000 -- yanked
lock_api = 0.1.2 # 2018-06-18 02:07:52 +0000 -- yanked
```

### Dependencies
#### list the deps for the latest version
**Note** use `--no-features` (`-n`) to not list the features
>cargo whatfeatures curl --deps
```
curl = 0.4.31
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
  │ ├─ curl-sys = ^0.4.33
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

#### list the deps for a local crate
>cargo whatfeatures --manifest-path . -d -n
```
cargo-whatfeatures = 0.9.2
├─ no optional dependencies
└─ required dependencies
  ├─ normal
  │ ├─ anyhow = ^1.0.32 
  │ ├─ attohttpc = ^0.15.0 (has enabled features)
  │ ├─ cargo_metadata = ^0.11.1 
  │ ├─ crate_version_parse = ^0.2.0 
  │ ├─ directories-next = ^1.0.1 
  │ ├─ flate2 = ^1.0.16 
  │ ├─ home = ^0.5.3 
  │ ├─ pico-args = ^0.3.3 
  │ ├─ serde = ^1.0.114 (has enabled features)
  │ ├─ tar = ^0.4.29 
  │ ├─ time = ^0.2.16 
  │ └─ yansi = ^0.5.0 
  ├─ no development dependencies
  └─ no build dependencies
```

#### example of scrying a workspace
>cargo whatfeatures ~/dev/godot-rust
```
workspace for godot-rust
├─ gdnative = 0.9.0-preview.0
│ └─ features
│   ├─ default
│   │ └─ bindings
│   ├─ bindings (default)
│   ├─ formatted
│   └─ gd_test
├─ gdnative-bindings = 0.9.0-preview.0
│ └─ features
│   ├─ no default features
│   ├─ formatted
│   └─ one_class_one_file
├─ gdnative-core = 0.9.0-preview.0
│ └─ features
│   ├─ default
│   │ └─ nativescript
│   ├─ gd_test
│   └─ nativescript (default)
├─ gdnative-derive = 0.9.0-preview.0
│ └─ no features
├─ gdnative-impl-proc-macros = 0.9.0-preview.0
│ └─ no features
├─ gdnative-sys = 0.9.0-preview.0
│ └─ no features
└─ gdnative_bindings_generator = 0.9.0-preview.0
  └─ features
    ├─ no default features
    └─ debug
```

using the `-r`, `--restricted` will also list packages that are set to private
>cargo whatfeatures -r ~/dev/godot-rust
```
workspace for godot-rust
├─ dodge_the_creeps = 0.1.0 (restricted)
│ └─ no features
├─ gdnative = 0.9.0-preview.0
│ └─ features
│   ├─ default
│   │ └─ bindings
│   ├─ bindings (default)
│   ├─ formatted
│   └─ gd_test
├─ gdnative-bindings = 0.9.0-preview.0
│ └─ features
│   ├─ no default features
│   ├─ formatted
│   └─ one_class_one_file
├─ gdnative-core = 0.9.0-preview.0
│ └─ features
│   ├─ default
│   │ └─ nativescript
│   ├─ gd_test
│   └─ nativescript (default)
├─ gdnative-derive = 0.9.0-preview.0
│ └─ no features
├─ gdnative-impl-proc-macros = 0.9.0-preview.0
│ └─ no features
├─ gdnative-sys = 0.9.0-preview.0
│ └─ no features
├─ gdnative-test = 0.1.0 (restricted)
│ └─ no features
├─ gdnative_bindings_generator = 0.9.0-preview.0
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

#### list the deps and transitive features
you can list the deps of a crate, with its transitively enabled features
> cargo whatfeatures --deps --no-features --verbose reqwest 
```
reqwest = 0.10.7
├─ optional dependencies
│ ├─ for cfg(not(target_arch = "wasm32"))
│ │ ├─ async-compression = ^0.3.0 (has enabled features)
│ │ │ └─ stream
│ │ ├─ cookie = ^0.14 (renamed to cookie_crate)
│ │ ├─ cookie_store = ^0.12 
│ │ ├─ hyper-rustls = ^0.21 
│ │ ├─ hyper-tls = ^0.4 
│ │ ├─ native-tls = ^0.2 (renamed to native-tls-crate)
│ │ ├─ rustls = ^0.18 (has enabled features)
│ │ │ └─ dangerous_configuration
│ │ ├─ time = ^0.2.11 
│ │ ├─ tokio-rustls = ^0.14 
│ │ ├─ tokio-socks = ^0.2 
│ │ ├─ tokio-tls = ^0.3.0 
│ │ ├─ trust-dns-resolver = ^0.19 
│ │ └─ webpki-roots = ^0.19 
│ └─ serde_json = ^1.0 
└─ required dependencies
  ├─ normal
  │ ├─ for cfg(not(target_arch = "wasm32"))
  │ │ ├─ base64 = ^0.12 
  │ │ ├─ encoding_rs = ^0.8 
  │ │ ├─ futures-core = ^0.3.0 
  │ │ ├─ futures-util = ^0.3.0 
  │ │ ├─ http-body = ^0.3.0 
  │ │ ├─ hyper = ^0.13.4 (has enabled features)
  │ │ │ └─ tcp
  │ │ ├─ ipnet = ^2.3 
  │ │ ├─ lazy_static = ^1.4 
  │ │ ├─ log = ^0.4 
  │ │ ├─ mime = ^0.3.7 
  │ │ ├─ percent-encoding = ^2.1 
  │ │ ├─ pin-project-lite = ^0.1.1 
  │ │ └─ tokio = ^0.2.5 (has enabled features)
  │ │   ├─ tcp
  │ │   └─ time
  │ ├─ for cfg(target_arch = "wasm32")
  │ │ ├─ js-sys = ^0.3.28 
  │ │ ├─ wasm-bindgen = ^0.2.51 (has enabled features)
  │ │ │ └─ serde-serialize
  │ │ ├─ wasm-bindgen-futures = ^0.4.1 
  │ │ └─ web-sys = ^0.3.25 (has enabled features)
  │ │   ├─ Headers
  │ │   ├─ Request
  │ │   ├─ RequestInit
  │ │   ├─ RequestMode
  │ │   ├─ Response
  │ │   ├─ Window
  │ │   ├─ FormData
  │ │   ├─ Blob
  │ │   └─ BlobPropertyBag
  │ ├─ for cfg(windows)
  │ │ └─ winreg = ^0.7 
  │ ├─ bytes = ^0.5 
  │ ├─ http = ^0.2 
  │ ├─ mime_guess = ^2.0 
  │ ├─ serde = ^1.0 
  │ ├─ serde_urlencoded = ^0.6.1 
  │ └─ url = ^2.1 
  ├─ development
  │ └─ for cfg(not(target_arch = "wasm32"))
  │   ├─ brotli = ^3.3.0 (renamed to brotli_crate)
  │   ├─ doc-comment = ^0.3 
  │   ├─ env_logger = ^0.7 
  │   ├─ hyper = ^0.13 (has enabled features)
  │   │ ├─ tcp
  │   │ └─ stream
  │   ├─ libflate = ^1.0 
  │   ├─ serde = ^1.0 (has enabled features)
  │   │ └─ derive
  │   └─ tokio = ^0.2.0 (has enabled features)
  │     └─ macros
  └─ no build dependencies
```
