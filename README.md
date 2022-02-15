# Marker traits for primitive types

This crate provides [marker traits](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.) for [primitive types](https://doc.rust-lang.org/reference/types.html):

* [Prim](https://docs.rs/just_prim/latest/just_prim/trait.Prim.html);
    * [PrimTextual](https://docs.rs/just_prim/latest/just_prim/trait.PrimTextual.html);
    * [PrimNum](https://docs.rs/just_prim/latest/just_prim/trait.PrimNum.html);
      * [PrimInt](https://docs.rs/just_prim/latest/just_prim/trait.PrimInt.html);
        * [PrimSignedInt](https://docs.rs/just_prim/latest/just_prim/trait.PrimSignedInt.html);
        * [PrimUnsignedInt](https://docs.rs/just_prim/latest/just_prim/trait.PrimUnsignedInt.html);
    * [PrimFloat](https://docs.rs/just_prim/latest/just_prim/trait.PrimFloat.html).

# Example

## Cargo.toml

```toml
## ...

[dependencies]
just_prim = { version = "0.1.0" }

## ...

[features]

## https://doc.rust-lang.org/beta/unstable-book/language-features/marker-trait-attr.html
marker_trait_attr = ["just_prim/marker_trait_attr"]
```

## src/main.rs

```rust
// Uncomment if you want to conditionally use the feature. Remove otherwise.
// #![cfg_attr(feature = "marker_trait_attr", feature(marker_trait_attr))]

use just_prim::Prim;

trait MyExtensionTraitForPrimTys: Prim {
    // ...
}

fn main() {}
```

## Building, running, and testing

* `cargo build <other options> --features marker_trait_attr` if you want the feature and `cargo build <other options>` otherwise;
* `cargo run<other options> --features marker_trait_attr ` if you want the feature and `cargo build <other options>` otherwise;
* `cargo test <other options> --features marker_trait_attr` if you want the feature and `cargo test <other options>` otherwise.

# Features

With [`marker_trait_attr`](https://doc.rust-lang.org/beta/unstable-book/language-features/marker-trait-attr.html) [Nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) [feature](https://doc.rust-lang.org/cargo/reference/features.html), each of the provided traits also has `#[marker]` attribute which allows more optimal implementation of traits via two or more "overlapping" generic `impl` blocks. Without `#[marker]` attribute, the "overlapping" `impl` blocks would be conflicting.

# Similar crates by the author:

* [`epui`](https://crates.io/crates/epui) - Equisized (primitive) unsigned ints for primitive ints: u8 for u8, u16 for i16, etc
* [`epsi`](https://crates.io/crates/epsi) - Equisized (primitive) signed ints for primitive ints
* [`primitive_promotion`](https://crates.io/crates/primitive_promotion) - Primitive promotions for primitive numeric types: u16 for u8, i32 for i16, f64 for f32, etc
* [`is_signed_trait`](https://crates.io/crates/is_signed_trait) - Trait for `IS_SIGNED` associated constant
* [`max_len_base_10_as_usize`](https://crates.io/crates/max_len_base_10_as_usize) - Trait offering constant maximum lengths of primitive integers as usize
* [`min_max_traits`](https://crates.io/crates/min_max_traits) - Traits for `MIN` and `MAX` associated constants
* as well as others that can be found on [crates.io](https://crates.io/users/JohnScience)

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>