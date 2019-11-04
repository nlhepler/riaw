use std::cell::UnsafeCell;
use std::ptr::NonNull;

pub use rand::Rng;

type FastRng = rand_pcg::Pcg64Mcg;

thread_local!(
    pub static THREAD_RNG_KEY: UnsafeCell<FastRng> = { UnsafeCell::new(FastRng::new(0xDEADBEEF)) };
);

pub struct ThreadRng {
    rng: NonNull<FastRng>,
}

pub fn thread_rng() -> ThreadRng {
    let raw = THREAD_RNG_KEY.with(|t| t.get());
    let rng = NonNull::new(raw).unwrap();
    ThreadRng { rng }
}

impl rand::RngCore for ThreadRng {
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
