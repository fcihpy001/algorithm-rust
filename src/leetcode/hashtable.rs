
use std::collections::HashMap;

pub fn l1_two_sum_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for num in 0..nums.len() {
        let complement = target - nums[i];
        if map.contains_key(&complement)  && map[&complement] != i {
            return vec![i as i32, map[&complement] as i32];
        }
        map.insert(nums[i], num);
    }
    return vec![]
}
pub fn l1_two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        map.insert(nums[i], i);
    }
    for num in 0..nums.len() {
        let complemnet = target - nums[i];
        if map.contains_key(&complemnet) && map[&complemnet] != i {
            return vec![num as i32, map[&complemnet] as i32];
        }
    }
    return vec![];
}

pub fn l49_group_anagrams_v1(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut vecs = vec![];
    let mut used = vec![false; strs.len()];

    for str in 0..strs.len() {
        let  mut temp = Vec::new();
        if !used[i] {
            temp.push(strs[i].clone());
            for j in i + 1..strs.len() {
               let mut is_anagram = true;
                if strs[i].len() != strs[j].len() {
                    continue;
                }
                let mut map = HashMap::new();
                for char in strs[i].chars() {
                    let count = map.entry(char).or_insert(0);
                    *count += 1;
                }
                for char in strs[j].chars() {
                    let count = map.entry(char).or_insert(0);
                    *count -= 1;
                    if *count < 0 {
                        is_anagram = false;
                        break;
                    }
                }
                if is_anagram {
                    used[j] = true;
                    temp.push(strs[j].clone());
                }
            }
        }
        if !temp.is_empty() {
            vecs.push(temp);
        }
    }
    vecs
}

pub fn l49_group_anagrams_v2(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut vcs = Vec::new();
    let mut map = HashMap::new();

    for str in 0..strs.len() {
        let mut count = [0;26];
        for char in strs[i].chars() {
            let index = (char as u32 - 'a' as u32) as usize;
            count[index] += 1;
        }

        let mut chars = vec![];
        for i in 0..count.len() {
            chars.push(count[i].to_string() + "#");
        }
        let key = chars.into_iter().collect();
        let value = map.get(&key);

        if value != None {
            let mut v = value.unwrap().to_vec();
            v.push(strs[i].clone());
            map.insert(key,v);
        } else {
            let v = vec![strs[i].clone()];
            map.insert(key,v);
        }
    }

    for value in map.values() {
        vcs.push(value.to_vec())
    }
    vcs
}

pub fn l49_group_anagrams_v3(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut vcs = Vec::<Vec<String>>::new();
    let mut map = HashMap::<String, Vec<String>>::new();

    for str in 0..strs.len() {
        let mut chars = vec![];

        // 取每一个字符串的字符
        for char in strs[i].chars() {
            chars.push(char);
        }
        chars.sort();
        // 使用每个索引的字符串，排好序，作为map的key，value为原始字符串
        let key = chars.into_iter().collect::<String>();
        let value = map.get(&key);

        if value != None {
           let mut v = value.unwrap().to_vec();
            v.push(strs[i].clone());
            map.insert(key, v);
        } else {
            let v = vec![strs[i].clone];
            map.insert(key,v);
        }
    }

    for value in map.values() {
        vcs.push((*value).clone());
    }
    return vcs;
}

pub fn l242_is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
     let mut map = HashMap::new();

    for char in s.chars() {
        let count = map.entry(char).or_insert(0);
        *count += 1;
    }
    for char in t.chars() {
        let count = map.entry(char).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }
    return  true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");

        println!("{}", Solution::is_anagram(s, t));
    }

}






