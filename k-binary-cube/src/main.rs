use std::io::{self};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error reading string");
    let mut iter = buffer.split_whitespace();
    let mut i = 0;
    let mut from: String = "".to_string();
    let mut to: String = "".to_string();
    loop {
        if i == 2 { break; };
        match iter.next() {
            Some(str) => {
                if i == 0 {
                    from = str.to_string();
                } else {
                    to = str.to_string();
                }
            }
            None => {
                break;
            }
        };
        i += 1;
    }

    let from_chars: Vec<char> = from.chars().collect();
    let to_chars: Vec<char> = to.chars().collect();

    let mut diff_positions = Vec::new();
    let mut same_positions = Vec::new();

    for (i, x) in from_chars.iter().enumerate() {
        if *x != to_chars[i] {
            diff_positions.push(i);
        } else {
            same_positions.push(i);
        }
    }

    let dfp_len = diff_positions.len();

    for k in 0..dfp_len {
        let mut chars: Vec<char> = from_chars.to_vec();
        print!("{} ", from);
        for i in 0..dfp_len {
            let index = diff_positions[(i + k) % diff_positions.len()];
            if chars[index] == '0' {
                chars[index] = '1';
                // replace(&mut chars[index], '1');
            } else {
                chars[index] = '0';
                // replace(&mut chars[index], '0');
            }
            let str: String = chars.iter().collect();
            print!("{} ", str);
        }
        print!("\n")
    }

    for k in 0..same_positions.len() {
        let mut chars: Vec<char> = from_chars.to_vec();
        let index = same_positions[k];
        print!("{} ", from);
        if chars[index] == '0' {
            chars[index] = '1';

            // replace(&mut chars[index], '1');
        } else {
            chars[index] = '0';

            // replace(&mut chars[index], '0');
        }
        let str: String = chars.iter().collect();
        print!("{} ", str);
        for i in &diff_positions {
            if chars[*i] == '0' {
                chars[*i] = '1';
                // replace(&mut chars[*i], '1');
            } else {
                chars[*i] = '0';
                // replace(&mut chars[*i], '0');
            }
            let str: String = chars.iter().collect();
            print!("{} ", str);
        }
        if chars[index] == '0' {
            chars[index] = '1';
            // replace(&mut chars[index], '1');
        } else {
            chars[index] = '0';
            // replace(&mut chars[index], '0');
        }
        let str: String = chars.iter().collect();
        print!("{} \n", str);
    }
}