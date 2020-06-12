use std::collections::LinkedList;
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
    // println!("stamp: {}, target: {}", stamp, target);
    let m = stamp.len();
    let n = target.len();
    let mut target_vec: Vec<char> = target.chars().collect();
    let stamp_vec: Vec<char> = stamp.chars().collect();
    let mut ans: Vec<i32> = Vec::new();
    let mut left = 0;
    let mut stars = 0;

    while !can_update(&stamp_vec, &target_vec, left) && left <= n - m + 1 {
        left += 1;
    }
    if left >= n - m + 1 {
        return Vec::new();
    }
    ans.push(left as i32);
    stars += do_update(&stamp_vec, &mut target_vec, left);

    if stars == n {
        ans.reverse();
        return ans;
    }
    let mut right = left + m;
    if left > 0 { left -= 1}
    // println!("After init left: {}, right: {}, target: {:?}", left, right, target_vec);
    while left >= 0 && right <= n {
        if can_update(&stamp_vec, &target_vec, left) {
            ans.push(left as i32);
            stars += do_update(&stamp_vec, &mut target_vec, left);
            // println!("Match at left {}, target: {:?}", left, target_vec);
            if left > 0 { left -= 1; }
        } else if can_update(&stamp_vec, &target_vec, right - m) {
            ans.push((right - m) as i32);
            stars += do_update(&stamp_vec, &mut target_vec, right - m);
            // println!("Match at right {}, target: {:?}", right, target_vec);
            right += 1;
        } else {
            // println!("Region {} - {} expanded, searching for a new one. Target: {:?}", left, right, target_vec);
            left = 0;
            while !can_update(&stamp_vec, &target_vec, left) && left <= n - m + 1 {
                left += 1;
            }
            if left >= n - m + 1 {
                return Vec::new();
            }
            stars += do_update(&stamp_vec, &mut target_vec, left);
            ans.push(left as i32);
            right = left + m;
            if left > 0 { left -= 1}
        }

        // println!("Left {}, right {}", left, right);

        if stars == n {
            ans.reverse();
            return ans;
        }
    }
    Vec::new()
}
