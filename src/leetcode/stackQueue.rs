
pub fn l20_valid_parentheses(s: String) -> bool {
    let chars: Vec<char>  = s.chars().collect();
    if chars.len() == 0 {
        return true
    }
    let mut stack = Vec::new();
    for i in 0..chars.len() {
        if chars[i] == '(' {
            stack.push(')');
        } else if chars[i] == '[' {
            stack.push(']');
        } else if chars[i] == '{' {
            stack.push('}');
        } else if stack.is_empty() || chars[i] != stack.pop().unwrap() {
            return false;
        }
    }
    return stack.is_empty();
}

pub fn l20_valid_parentheses2(s: String) -> bool {
    let mut stack = Vec::new();
    for char in s.chars() {
        match char {
            '('|'{'|'[' => {
                stack.push(char)
            },
            ')'|'}'|']' => {
                if stack.is_empty() {
                    return false;
                }
                if char != stack.pop() {
                    return false;
                }
                // let res = || {
                //     "([{".find(char) == "([{".find(char)
                // };
            },
            _ => ()
        }
    }
    stack.is_empty()
}

pub mod l155_minstack {
    struct MinStack {
        stack: Vec<i32>,
        min_stack: Vec<i32>
    }
    impl MinStack {
        fn new() -> Self {
            Self {
                stack: Vec::new(),
                min_stack: Vec::new()
            }
        }

        fn push(&mut self, x: i32) {
            self.stack.push(x);
            if self.min_stack.is_empty()
                || x <= *self.min_stack.last().unwrap() {
                self.min_stack.push(x)
            }
        }

        fn pop(&mut self) {
            if self.stack.is_empty() {
                return;
            }
            if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
                self.min_stack.pop()
            }
        }

        fn top(&self) -> i32 {
            return *self.stack.last().unwrap();
        }

        fn get_min(&self) -> i32 {
            return *self.min_stack.last().unwrap()
        }
    }
}
pub mod l239 {
    use std::collections::VecDeque;
    pub fn l239_max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 || k == 1 {
            return nums;
        }
        let mut res = Vec::new();
        let mut deque = VecDeque::new();
        for i in 0..nums.len() {
            push(&mut deque, nums[i]);

            if (i as i32) > k - 1 {
                pop(&mut, nums[i - k as usize]);
                res.push(max(&deque));
            } else if (i as i32) == k - 1 {
                res.push(max(&deque));
            }
        }
        return res;
    }

    fn push(deque: &mut VecDeque<i32>, n: i32) {
        while !deque.is_empty() && *deque.back().unwrap() < n {
            deque.pop_back();
        }
        deque.push_back(n)
    }

    fn pop(deque: &mut VecDeque<i32>, n: i32) {
        if !deque.is_empty() && *deque.front().unwrap() == n {
            deque.pop_front()
        }
    }

    fn max(deque: &VecDeque<i32>) -> i32 {
        return *deque.front().unwrap()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test155() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        println!("{}", stack.get_min());
        stack.pop();
        println!("{}", stack.top());
        println!("{}", stack.get_min());

    }
}
