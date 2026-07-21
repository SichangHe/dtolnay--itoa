# u128 two-chunk layout

`u128_chunks.rs` proves the arithmetic bound used by the high-`u128` branch of `Unsigned::fmt`: two `10^16` divisions produce two sixteen-digit remainders and a leading quotient below `10^7`.

The corresponding runtime check is the `u128_split_bridge_matches_production` unit test in `src/lib.rs`. It compares the actual private divider with Rust Euclidean division and compares `Buffer::format` with Rust formatting on split boundaries and a fixed deterministic corpus.

The proof intentionally stops at the Euclidean contract boundary. It does not claim to verify the magic multiplier, lookup-table encoder, or all `itoa` integer paths.
