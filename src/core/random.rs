use rand::distributions::uniform::SampleRange;
use rand::prelude::*;


pub struct Random;

impl Random {
    /// Generates `N` random numbers
    pub fn generate<const N: usize, R: SampleRange<i64> + Clone>(range: R) -> [i64; N] {
        let mut r = rand::thread_rng();

        let mut nums = [i64::default(); N];
        for i in 0..N {

            nums[i] = range.clone().sample_single(&mut r);
        }

        nums
    }

    /// Peaks `N` elements from `Vec`
    pub fn peak<const N: usize, T>(variants: Vec<T>) -> [T; N]
    where T: Default + Copy + Clone
    {
        let mut r = rand::thread_rng();
        let mut buf = [T::default(); N];

        for (n, v) in variants.choose_multiple(&mut r, N).zip(buf.iter_mut()) {
            *v = *n
        }

        buf
    }
}
