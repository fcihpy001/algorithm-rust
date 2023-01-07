// use std::fmt::{Debug, Display};
//
// // 插入
// // 会划分两个区域，已排序区域和未排序区域， 每次从未排序区域里拿出第一个值，分别与已经排序区域数字比较，找到合适的位置后插入
// pub fn insert_sort1<T: PartialOrd+Debug+Display>(array: &mut [T]) {
//     for i in 1..array.len() {
//         println!("外层循环: {}---{:?}", i, array);
//         let mut j = i;
//
//         while j > 0 && array[j-1] > array[j] {
//             println!("--内层已经排好序循环: J:{} -索引值：{}， {:?}", j,array[j], array);
//             array.swap(j-1, j);
//             j -= 1
//         }
//     }
// }
//
// pub fn insertion_sort2<T: Ord + Copy>(array: &mut [T]) {
//     for i in 1..array.len() {
//         let mut j = i;
//         let cur = array[i];
//         while j > 0 && cur < array[j - 1] {
//             array[j] = array[j - 1];
//             j -= 1;
//         }
//         array[j] = cur;
//     }
// }
//
// pub fn insert_binary_search<T: Ord>(array: &mut [T]) {
//     for i in 1..array.len() {
//         let pos = array[..i].binary_search(&array[i]).unwrap_or_else(|pos|pos);
//         let mut j = i;
//         while j > pos {
//             array.swap(j -1 , j);
//             j -= 1;
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::super::is_sorted;
//     use super::*;
//
//     #[test]
//     fn empty() {
//         let mut arr: [u8; 0] = [];
//         insertion_sort(&mut arr);
//         assert!(is_sorted(&arr));
//     }
//
//     #[test]
//     fn one_element() {
//         let mut arr: [char; 1] = ['a'];
//         insertion_sort(&mut arr);
//         assert!(is_sorted(&arr));
//     }
//
//     #[test]
//     fn already_sorted() {
//         let mut arr: [&str; 3] = ["a", "b", "c"];
//         insertion_sort(&mut arr);
//         assert!(is_sorted(&arr));
//     }
//
//     #[test]
//     fn basic() {
//         let mut arr: [&str; 4] = ["d", "a", "c", "b"];
//         insertion_sort(&mut arr);
//         assert!(is_sorted(&arr));
//     }
//
//     #[test]
//     fn odd_number_of_elements() {
//         let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
//         insertion_sort(&mut arr);
//         assert!(is_sorted(&arr));
//     }
//
//     #[test]
//     fn repeated_elements() {
//         let mut arr: Vec<usize> = vec![542, 542, 542, 542];
//         insertion_sort(&mut arr);
//         assert!(is_sorted(&arr));
//     }
// }
