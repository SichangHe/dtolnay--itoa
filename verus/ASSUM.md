# u128_div_rem_contract

`src/lib.rs::div_rem_1e16` is treated by the Verus arithmetic model as Euclidean division and remainder by `10^16`. The executable bridge checks that concrete magic-multiply helper on the bounded corpus; this proof does not prove the multiplier implementation for every `u128`.
