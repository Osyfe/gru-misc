pub use rand::{Rng, RngExt, seq::SliceRandom as Slice};
pub use rand_distr::{self as distr, Distribution};

pub type Algo = rand_xoshiro::Xoshiro256PlusPlus;

pub fn rng_seed(seed: u64) -> Algo
{
    use rand::SeedableRng;
    Algo::seed_from_u64(seed)
}

pub fn rng_instant() -> Algo
{
    use std::time::{SystemTime, UNIX_EPOCH};
    rng_seed(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
}

pub fn rng_entropy() -> Algo
{
	use rand::{rngs::SysRng, TryRng};
    rng_seed(SysRng.try_next_u64().unwrap())
}
