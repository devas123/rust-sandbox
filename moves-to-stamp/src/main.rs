use std::mem::{replace};

fn main() {
    println!("{:?} ", moves_to_stamp("de".to_string(), "ddeddeddee".to_string()));
    println!("{:?} ", moves_to_stamp("abc".to_string(), "ababc".to_string()));
    println!("{:?} ", moves_to_stamp("ffebb".to_string(), "fffeffebbb".to_string()));
    println!("{:?} ", moves_to_stamp("mda".to_string(), "mdadddaaaa".to_string()));
    println!("{:?} ", moves_to_stamp("ffc".to_string(), "ffffcfffffffffc".to_string()));
}

const STAR: char = '*';

fn can_update(stamp: &Vec<char>, target: &Vec<char>, start_index: usize) -> bool {
    if target.len() - start_index < stamp.len() {
        return false;
    }
    let mut all_stars = true;
    for i in 0..stamp.len() {
        if target[start_index + i] != STAR { all_stars = false }
        if target[start_index + i] != STAR && stamp[i] != target[start_index + i] {
            return false;
        }
    }
    !all_stars
}

fn do_update(stamp: &Vec<char>, target: &mut Vec<char>, start_index: usize) -> usize {
    if target.len() - start_index < stamp.len() {
        return start_index;
    }
    let mut updated = 0;
    for i in 0..stamp.len() {
        if target[start_index + i] != STAR {
            replace(&mut target[start_index + i], STAR);
            updated += 1;
        }
    }
    updated
}

fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    let m = stamp.len();
    let n = target.len();
    let mut target_vec: Vec<char> = target.chars().collect();
    let stamp_vec: Vec<char> = stamp.chars().collect();
    let mut ans: Vec<i32> = Vec::new();
    let mut stars = 0;
    while stars < n {
        let mut matched = false;
        for i in 0..n - m + 1 {
            if can_update(&stamp_vec, &target_vec, i) {
                stars += do_update(&stamp_vec, &mut target_vec, i);
                ans.push(i as i32);
                matched = true;
            }
        }
        if stars == n {
            ans.reverse();
            return ans;
        }
        if !matched {
            return Vec::new();
        }
    }
    Vec::new()
}
