use rand::distributions::uniform::SampleRange;

pub struct Random;

impl Random {
    /// Generates `N` random numbers
    pub fn generate<const N: usize, R: SampleRange<i64> + Clone>(range: R) -> [i64; N] {
        let mut r = rand::thread_rng();

        let mut nums = [0i64; N];
        for i in 0..N {

            nums[i] = range.clone().sample_single(&mut r);
        }

        nums
    }

    /// Peaks `N` elements from `Vec`
    pub fn peak<const N: usize, T: Clone>(variants: Vec<T>) -> [T; N] {
        Self::generate(0..variants.len() as i64).map(|v| variants.get(v as usize).unwrap().clone())
    }
}