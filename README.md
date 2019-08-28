# polygon3

Polygon boolean operations, focused on correctness.

## Correctness

The important functions have been fuzz tested.
Fuzz tests will be able to be found [here][1] once the crate is published.

This crate uses integer arithmetic internally, so floating point error cannot happen.
I was also careful to make integer overflow impossible.

## TODO
[API Checklist][2].
Publish Crate.

[1]: https://docs.rs/crate/polygon3/0.1.0/source/fuzz/src/main.rs
[2]: https://rust-lang-nursery.github.io/api-guidelines/checklist.html