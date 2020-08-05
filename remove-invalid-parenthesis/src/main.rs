use std::collections::HashSet;

fn main() {
    println!("()())() -> {:?}", remove_invalid_parentheses("()())()".to_string()));
}

mod recurse_mod {
    use std::collections::HashSet;

    pub fn recurse(index: usize, s: &Vec<char>, chars: &mut String, left_count: i32, right_count: i32, left_rem: i32, right_rem: i32, result: &mut HashSet<String>) {
        if index == s.len() {
            if left_rem == 0 && right_rem == 0 {
                result.insert(chars.to_string());
            }
            return;
        }
        let char: char = s[index];

        match char {
            ')' => if right_rem > 0 { recurse(index + 1, s, chars, left_count, right_count, left_rem, right_rem - 1, result) },
            '(' => if left_rem > 0 { recurse(index + 1, s, chars, left_count, right_count, left_rem - 1, right_rem, result) }
            _ => {}
        }
        chars.push(char);
        if char != '(' && char != ')' {
            recurse(index + 1, s, chars, left_count, right_count, left_rem, right_rem, result);
        } else if char == '(' {
            recurse(index + 1, s, chars, left_count + 1, right_count, left_rem, right_rem, result);
        } else if right_count < left_count {
            recurse(index + 1, s, chars, left_count, right_count + 1, left_rem, right_rem, result);
        }
        chars.pop();
    }
}
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut left_invalid = 0;
        let mut right_invalid = 0;
        let mut result: HashSet<String> = HashSet::new();
        for x in s.chars() {
            match x {
                '(' => left_invalid += 1,
                ')' => if left_invalid > 0 { left_invalid -= 1 } else { right_invalid += 1 },
                _ => {}
            }
        };
        let mut chars = "".to_string();
        recurse_mod::recurse(0, &s.chars().collect(), &mut chars, 0, 0, left_invalid, right_invalid, &mut result);
        let mut ret = Vec::new();
        for x in result {
            ret.push(x)
        }
        ret
    }
}