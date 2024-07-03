# Projects
A CLI tool for initialising projects from templates.


### Installation
See the releases section to download a binary for your specific system. If a precompiled binary is not available you can always build from source.


### Build from source

```bash
$ cargo build --release
```


### Usage

```bash
$ projects --help

Usage: projects [OPTIONS]

Options:
  -t, --template <TEMPLATE>  [default: java-gradle]
  -n, --name <NAME>          [default: sandbox]
  -l, --list
  -h, --help                 Print help
  -V, --version              Print version
```


### Available templates

```bash
$ projects --list

Valid templates include:
 - c
 - cpp-cmake
 - javascript (or 'js')
 - typescript (or 'ts')
 - python (or 'py')
 - java-gradle
 - go
```
