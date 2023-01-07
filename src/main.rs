extern crate core;
extern crate core;

use crate::general::two_sum;
use crate::sort::heap_sort::heap_sort;
use crate::sort::quick_sort::quick_sort;

pub mod data_structures;
pub mod general;
pub mod search;
pub mod sort;

fn main() {
    println!("Hello, world!");
    let mut a = vec![1, 4, 2, 9, 5, 7];
    heap_sort(&mut a);
    println!("{:?}", a);
}
