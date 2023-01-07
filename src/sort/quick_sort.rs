pub fn quick_sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let p = partion(array);
    let (left, right) = array.split_at_mut(p);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partion<T: Ord>(array: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..array.len() {
        if array[i] < array[p] {
            array.swap(i, p + 1);
            array.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

fn partion2(data: &mut [u32], start: usize, end: usize) -> usize {
    let mut left = start;
    let mut right = end - 1;
    while left < right {
        while data[left] <= data[start - 1] && left < right {
            left += 1;
        }

        while data[right] >= data[start - 1] && left < right {
            right -= 1;
        }
        data.swap(left, right);
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        quick_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        quick_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
