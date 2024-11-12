# inline-str

`inline-str` is a small and cheaply cloned string type, intended for use in cases where you expect to be cloning the same short string many times.

It is a thin layer over [`inline-array`](https://github.com/komora-io/inline-array) inspired by [@spacejam's](https://github.com/spacejam) work who suggested I build this crate a while back.

## Contributions

I implemented things that seem obviously useful to me, but would love to accept PRs to introduce more functionality people would fine useful.

## Alternatives

The are many crates with similar design and purpose, and I highly encourage you to use the one that you like the most:

- [compact_str](https://github.com/ParkMyCar/compact_str)
- [inlinable_string](https://github.com/fitzgen/inlinable_string)
- [hipstr](https://github.com/polazarus/hipstr)
- [imstr](https://github.com/xfbs/imstr)
- Any many, many others

## License

This work is dual-licensed under Apache 2.0 and MIT.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: Apache-2.0 OR MIT`
