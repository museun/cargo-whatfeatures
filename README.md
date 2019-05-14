## whatfeatures
```
Usage: whatfeatures [OPTIONS]

Positional arguments:
  name

Optional arguments:
  -h, --help             display this message
  -v, --version VERSION  specific version to lookup
  -l, --list             list all version
  -j, --json             prints results as json
```

This allows you to lookup a **specific** crate, at a **specific** version and get its **default** and **optional** features.

# Examples:

## look up a specific version
```
whatfeatures twitchchat -v 0.5.0
twitchchat/0.5.0    
    default: all
    all: serde_hashbrown, parking_lot
    serde_hashbrown: serde, hashbrown/serde
```

## list all versions
```
whatfeatures markings --list
markings/0.1.1
  no default features
markings/0.1.0
  no default features
```

## specific version as json
```
whatfeatures twitchchat -v 0.3.0 --json | jq .    
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
