# Haste

> Choose a willing creature that you can see within range. Until the spell ends, the target's speed is doubled[..].
>
> -- <cite>Dnd 5e Player's Handbook</cite>

A work in-progress benchmarking library inspired by [divan](https://github.com/nvzqz/divan) and [criterion](https://github.com/bheisler/criterion.rs).

```rust,no_run
use haste::Throughput;

// Anywhere in `benches/` or your library!*
#[haste::bench(args = [1, 2, 3], throughput = Throughput::Bytes(arg * 10))]
async fn process_bytes(arg: usize) {
    // ..
} 

// In your benches/ folder.

// extern crate your_crate; <-- Add this if your benches use no items of your benchmarked library 
fn main() {
    haste::main();
}
```
\* Note: When registering benchmarks in your library, don't place them in a `#[cfg(test)]` module, as they won't be compiled. See [Benchmarking private APIs](#benchmarking-private-apis)

## Benchmarking private APIs
You can use Haste to benchmark private functions of your library. Unfortunately, these can't be placed in modules annotated with `#[cfg(test)]` as the library is not compiled with the `test` cfg when running `cargo bench`.

There are two workarounds for this:
### "Private" cargo feature
In your `Cargo.toml` declare e.g. an `_internal_benchmarking` feature
```toml
[features]
# Do not depend on this feature!
_internal_benchmarking = ["dep:haste"]

[dependencies]
haste = { version = "..", optional = true }
```
By preceding it with `_` this feature will [not be listed on docs.rs](https://github.com/rust-lang/cargo/issues/10882#issuecomment-1880844632).

Then you can use this feature to `cfg` your benchmarks in your library with acess to private items.

