use std::cell::UnsafeCell;
use std::ptr::NonNull;

pub use rand::{rngs::OsRng, Rng, RngCore, SeedableRng};

type FastRng = rand_pcg::Pcg64Mcg;

thread_local!(
    pub static THREAD_RNG_KEY: UnsafeCell<FastRng> = {
        let mut seed = [0u8; 16];
        OsRng.fill_bytes(&mut seed[..]);
        UnsafeCell::new(FastRng::from_seed(seed))
    };
);

pub struct ThreadRng {
    rng: NonNull<FastRng>,
}

pub fn thread_rng() -> ThreadRng {
    let raw = THREAD_RNG_KEY.with(|t| t.get());
    let rng = NonNull::new(raw).unwrap();
    ThreadRng { rng }
}

impl RngCore for ThreadRng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        unsafe { self.rng.as_mut().next_u32() }
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        unsafe { self.rng.as_mut().next_u64() }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        unsafe { self.rng.as_mut().fill_bytes(dest) }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        unsafe { self.rng.as_mut().try_fill_bytes(dest) }
    }
}
