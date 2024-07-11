> ![IMPORTANT]
> Uses released UniFFI version `0.28.0`.
> Working bindings tests in Swift and Kotlin.

# Works

Simple demo of workspace setup with three UniFFI consuming crates:

- `zero`
- `one`
- `two`

```sh
$ cargo tree -i zero --format "{lib}"
zero
├── one
│   └── two
└── two
```

Crate `two` contains bindings tests (`uniffi::build_foreign_language_testcases`)

## Tests

```sh
cargo test
```

# Setup

## Swift

### Xcode

Install Xcode 15

## Kotlin

### JNA

Install JNA:

```sh
curl https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.14.0/jna-5.14.0.jar --output jna-5.14.0.jar
```

### Direnv

Install direnv to automatically load ENV variables when standing in project root.

```sh
brew install direnv
```

Then, standing in project root, run:

```sh
direnv allow .
```

# Design

Crate `zero` exports two types, one record and one object, which are "imported"
by crate "one" in UDL:

```webudl
[ExternalExport="zero"]
typedef extern ZeroRecord;

[ExternalInterfaceExport="zero"]
typedef extern ZeroObject;
```

In turn, crate "one" uses `ZeroRecord` and `ZeroObject` in one record and in one object

Crate "two"'s UDL then "imports" the types from crate "one" (apparently the types from "zero" are "re-exported" automatically, i.e. crate "two" does not need to reference types from "zero" in UDL.):

```webudl
[ExternalExport="one"]
typedef extern OneRecord;

[ExternalInterfaceExport="one"]
typedef extern OneObject;
```

Finally crate "two" declares one record and one object, both containing records
and objects from "zero", "one" and also newtype declared in "two", for which
we have working bindings tests in Swift and Kotlin.
