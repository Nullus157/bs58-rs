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
> echo '04305e2b2473f058' | xxd -r -p | bs58
he11owor1d
```

### Decoding some data

```console
> echo -n 'he11owor1d' | bs58 -d | xxd -p
04305e2b2473f058
```

### Decoding with a different alphabet

```console
> echo -n 'he11owor1d' | bs58 -da=ripple | xxd -p
6065e79bba2f78
```

### Encoding with a custom alphabet

```console
> echo 'babce1c947b425' | xxd -r -p | bs58 -a='custom(abcdefghijkmnopqrstuvwxyz123456789ABCDEFGHJKLMNPQRSTUVWXYZ)'
he11owor1d
```

[install Rust]: https://www.rust-lang.org/tools/install
