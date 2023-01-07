use std::fmt::Debug;

// 选择
// 每轮选出一个假定最小值，然后依次与假定值比较，较小的先记录索引，
// 本轮结束后，与假定值索引进行替换，找到最小值，依次找到每一轮最小值
pub fn selection_sort<T: PartialOrd + Debug>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let size = array.len();
    for i in 0..size - 1 {
        println!("外层循环: {}---{:?}", i, array);
        let mut min_index = i;
        for j in i + 1..size {
            if array[j] < array[min_index] {
                min_index = j;
            }
            println!("--内层循环: {} -最小值索引：{}， {:?}", j, min_index, array);
        }
        if min_index != i {
            array.swap(i, min_index);
            println!("开始替换,替换后的数据为: {:?}", array);
        }
    }
}

pub fn selection_sort2<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec!["d", "a", "c", "b"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        selection_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec!["a"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a"]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!["a", "b", "c"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c"]);
    }
}
