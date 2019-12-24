# bs58-cli

A CLI utility for encoding/decoding base58 encoded data.

## Installation

Currently `bs58-cli` is only distributed through crates.io, [install Rust][]
then install with:

```console
> cargo install bs58-cli
Updating crates.io index
[...]

> bs58 --help
bs58 0.1.0
A utility for encoding/decoding base58 encoded data.
[...]
```

## Examples

### Encoding some data

```console
> echo -n '04305e2b2473f058' | xxd -r -p | bs58
he11owor1d
```

### Decoding some data

```console
> echo 'he11owor1d' | bs58 -d | xxd -p
04305e2b2473f058
```

[install Rust]: https://www.rust-lang.org/tools/install
