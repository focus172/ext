# Ext

Rust standard library extension. Includes many crates that provide quality of life 
features that arnt in the standard.


This library tries to keep dependencies to a minimum using crates with large 
overlap. The full dependency tree is shown below.

```txt
ext v0.1.0 (/home/focus/code/ext)
├── arrayvec v0.7.4
├── bitflags v2.4.1
├── cfg-if v1.0.0
├── either v1.9.0
├── env_logger v0.10.1
│   ├── humantime v2.1.0
│   ├── is-terminal v0.4.9
│   │   └── rustix v0.38.26
│   │       ├── bitflags v2.4.1
│   │       └── linux-raw-sys v0.4.12
│   ├── log v0.4.20
│   ├── regex v1.10.2
│   │   ├── aho-corasick v1.1.2
│   │   │   └── memchr v2.6.4
│   │   ├── memchr v2.6.4
│   │   ├── regex-automata v0.4.3
│   │   │   ├── aho-corasick v1.1.2 (*)
│   │   │   ├── memchr v2.6.4
│   │   │   └── regex-syntax v0.8.2
│   │   └── regex-syntax v0.8.2
│   └── termcolor v1.4.0
├── error-stack v0.4.1
│   [build-dependencies]
│   └── rustc_version v0.4.0
│       └── semver v1.0.20
├── glam v0.24.2
├── indexmap v2.1.0
│   ├── equivalent v1.0.1
│   └── hashbrown v0.14.3
├── log v0.4.20
├── num-bigint v0.4.4
│   ├── num-integer v0.1.45
│   │   └── num-traits v0.2.17
│   │       [build-dependencies]
│   │       └── autocfg v1.1.0
│   │   [build-dependencies]
│   │   └── autocfg v1.1.0
│   └── num-traits v0.2.17 (*)
│   [build-dependencies]
│   └── autocfg v1.1.0
├── num-traits v0.2.17 (*)
├── petgraph v0.6.4
│   ├── fixedbitset v0.4.2
│   └── indexmap v2.1.0 (*)
├── portable-atomic v1.6.0
├── rand v0.8.5
│   ├── libc v0.2.150
│   ├── rand_chacha v0.3.1
│   │   ├── ppv-lite86 v0.2.17
│   │   └── rand_core v0.6.4
│   │       └── getrandom v0.2.11
│   │           ├── cfg-if v1.0.0
│   │           └── libc v0.2.150
│   └── rand_core v0.6.4 (*)
└── regex v1.10.2 (*)
```
