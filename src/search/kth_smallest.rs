use crate::sorting::partition;
use std::cmp::{Ordering, PartialOrd};

pub fn kth_smallest<T>(input: &mut [T], k: usize) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if input.is_empty() {
        return None;
    }

    let kth = _kth_smallest(input, k, 0, input.len() - 1);
    Some(kth)
}

fn _kth_smallest<T>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T
where
    T: PartialOrd + Copy,
{
    if lo == hi {
        return input[lo];
    }

    let pivot = partition(input, lo as isize, hi as isize) as usize;
    let i = pivot - lo + 1;
    match k.cmp(&i) {
        Ordering::Equal => input[pivot],
        Ordering::Less => _kth_smallest(input, k, lo, pivot - 1),
    }
}
