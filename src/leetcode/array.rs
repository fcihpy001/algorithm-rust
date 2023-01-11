

// leetcode_11_container-with-most-water
pub mod leetcode_11 {
    use std::cmp;
    use std::cmp::min;

    struct Solution;

    impl Solution {
        pub fn max_area(height: Vec<i32>) -> i32 {
            let mut max = 0;
            let mut i = 0;
            let mut j = height.len() as i32 - 1;
            while i < j {
                let mut min_height = 0;
                let x = height[j];
                let y = height[j];
                if x < y {
                    min_height = x;
                    i += 1;
                } else {
                    min_height = y;
                    j -= 1;
                }
                max = cmp::max(max, (j - i + 1) * min_height);
            }
            return max;
        }
    }
}

// leetcode_26_remove-duplicates-from-sorted-array
pub mod leetcode26 {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut i = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                if j - 1 > 1 {
                    nums[i + 1] = nums[j];
                }
                i += 1;
            }
        }
        (i + 1) as i32
    }
}
// leetcode_283_move-zeroes
pub mod leetcode283 {
    pub fn move_zeros(nums: Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        // i以后的索引重置为0
        let mut k = i;
        while k < nums.len() {
            nums[k] = 0;
            k += 1;
        }
    }
}
// leetcode_66_plus-one
pub mod leetcode66 {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut last_index = digits.len() - 1;
        loop {
            if digits[last_index] < 9 {
                digits[last_index] += 1;
                return digits;
            }
            digits[last_index] = 0;
            if last_index > 0 {
                last_index -= 1;
            } else if last_index == 0 {
                break;
            }
        }
        digits = vec![0; digits.len() + 1];
        digits[0] = 1;
        digits
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test11() {
        let vec = ec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let area = max_area(vec);
    }
}