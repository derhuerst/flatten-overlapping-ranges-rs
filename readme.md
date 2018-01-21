# flatten-overlapping-ranges-rs

**Flatten overlapping ranges into a sequence of sections.**

```
---A---     ---D---
  -----B------
     --C--
```

```
-- --- -- --- -- -- -----
A  A   A         D  D
   B   B  B   B  B
       C  C
```

My first crate! 🙌

[![crates.io version](https://img.shields.io/crates/v/flatten_overlapping_ranges.svg)](https://crates.io/crates/flatten_overlapping_ranges)
[![build status](https://api.travis-ci.org/derhuerst/flatten-overlapping-ranges-rs.svg?branch=master)](https://travis-ci.org/derhuerst/flatten-overlapping-ranges-rs)
![ISC-licensed](https://img.shields.io/github/license/derhuerst/flatten-overlapping-ranges-rs.svg)
[![chat on gitter](https://badges.gitter.im/derhuerst.svg)](https://gitter.im/derhuerst)


## Installing

Put this into your `Cargo.toml`:

```toml
flatten_overlapping_ranges = "0.1.0"
```


## Usage

```rust
use flatten_overlapping_ranges::flatten;

let simple: Vec<(&char, usize, usize)> = vec![
    (&'a', 0, 7),
    (&'b', 2, 12),
    (&'c', 5, 5),
    (&'d', 12, 7)
];

println!("{:?}", flatten(&simple));
```

```
[
	(2, ['a']),
	(3, ['a', 'b']),
	(2, ['a', 'b', 'c']),
	(3, ['b', 'c']),
	(2, ['b']),
	(2, ['b', 'd']),
	(5, ['d'])
]
```


## Contributing

This is the first Rust code I've ever written, so be kind! 🙈

If you have a question or have difficulties using flatten-overlapping-ranges-rs, please double-check your code and setup first. If you think you have found a bug or want to propose a feature, refer to [the issues page](https://github.com/derhuerst/flatten-overlapping-ranges-rs/issues).
