use std::process::id;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Default> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2;
    }
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            let ldx = self.left_child_idx(idx);
            let rdx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[idx]) {
                ldx
            } else {
                rdx
            }
        }
    }
    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        let mut idx = self.count;
        while self.parent_idx(idx) > 0 {
            let pdx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[pdx]) {
                self.items.swap(idx, pdx)
            }
            idx = pdx;
        }
    }
}

impl<T: Default + Ord> Heap<T> {
    pub fn new_min() -> Heap<T> {
        Self::new(|a, b| a < b)
    }

    pub fn new_max() -> Heap<T> {
        Self::new(|a, b| a > b)
    }
}

impl<T: Default> Iterator for Heap<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        let next = Some(self.items.swap_remove(1));
        self.count -= 1;
        if self.count > 0 {
            let mut idx = 1;
            while self.children_present(idx) {
                let cdx = self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx], &self.items[cdx]) {
                    self.items.swap(idx, cdx);
                }
                idx = cdx;
            }
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap: Heap<i32> = Heap::new_max();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }

    struct Point(/* x */ i32, /* y */ i32);
    impl Default for Point {
        fn default() -> Self {
            Self(0, 0)
        }
    }

    #[test]
    fn test_key_heap() {
        let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
        heap.add(Point(1, 5));
        heap.add(Point(3, 10));
        heap.add(Point(-2, 4));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.next().unwrap().0, -2);
        assert_eq!(heap.next().unwrap().0, 1);
        heap.add(Point(50, 34));
        assert_eq!(heap.next().unwrap().0, 3);
    }
}
