use vstd::arithmetic::div_mod::{
    lemma_div_basics_4, lemma_fundamental_div_mod, lemma_mod_bound, lemma_multiply_divide_lt,
};
use vstd::prelude::*;

verus! {

pub open spec fn decimal_chunk() -> int {
    10_000_000_000_000_000int
}

pub open spec fn u128_max() -> int {
    340282366920938463463374607431768211455int
}

pub open spec fn leading_block_limit() -> int {
    10_000_000int
}

pub open spec fn low_block(n: int) -> int {
    n % decimal_chunk()
}

pub open spec fn middle_block(n: int) -> int {
    (n / decimal_chunk()) % decimal_chunk()
}

pub open spec fn leading_block(n: int) -> int {
    n / decimal_chunk() / decimal_chunk()
}

// @ASSUME:u128_div_rem_contract
/// This is the arithmetic layout required before `quot2 as u64` in the real u128 formatter.
pub proof fn u128_two_chunk_layout(n: int)
    requires
        0 <= n,
        n <= u128_max(),
    ensures
        0 <= low_block(n) < decimal_chunk(),
        0 <= middle_block(n) < decimal_chunk(),
        0 <= leading_block(n) < leading_block_limit(),
        n == decimal_chunk() * (decimal_chunk() * leading_block(n) + middle_block(n)) + low_block(n),
{
    assert(decimal_chunk() > 0);
    assert(u128_max() < decimal_chunk() * decimal_chunk() * leading_block_limit());

    lemma_mod_bound(n, decimal_chunk());
    lemma_fundamental_div_mod(n, decimal_chunk());
    lemma_div_basics_4(n, decimal_chunk());
    assert(0 <= n / decimal_chunk());

    lemma_mod_bound(n / decimal_chunk(), decimal_chunk());
    lemma_fundamental_div_mod(n / decimal_chunk(), decimal_chunk());
    lemma_div_basics_4(n / decimal_chunk(), decimal_chunk());

    lemma_multiply_divide_lt(n, decimal_chunk(), decimal_chunk() * leading_block_limit());
    lemma_multiply_divide_lt(n / decimal_chunk(), decimal_chunk(), leading_block_limit());

    assert(n == decimal_chunk() * (decimal_chunk() * (n / decimal_chunk() / decimal_chunk())
        + (n / decimal_chunk()) % decimal_chunk()) + n % decimal_chunk());
}

}

fn main() {}
