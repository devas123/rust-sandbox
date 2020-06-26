fn main() {
    assert_eq!(42, my_atoi("42".to_string()));
    assert_eq!(-42, my_atoi("   -42".to_string()));
    assert_eq!(4193, my_atoi("4193 with words".to_string()));
    assert_eq!(0, my_atoi("words and 987".to_string()));
    assert_eq!(-2147483648, my_atoi("-2147483649".to_string()));
    assert_eq!(0, my_atoi("".to_string()));
    assert_eq!(0, my_atoi("     ".to_string()));
    assert_eq!(1, my_atoi("+1".to_string()));
    assert_eq!(0, my_atoi("+-2".to_string()));
    assert_eq!(12345678, my_atoi("  0000000000012345678".to_string()));
    assert_eq!(0, my_atoi("  00000000000-12345678".to_string()));
    assert_eq!(-2147483648, my_atoi("-6147483648".to_string()));
    assert_eq!(10, my_atoi("010".to_string()));
    assert_eq!(-1324000, my_atoi("-01324000".to_string()));
    assert_eq!(2147483646, my_atoi("2147483646".to_string()));
    assert_eq!(0, my_atoi("-   234".to_string()));
    assert_eq!(0, my_atoi("   +0 123".to_string()));
    assert_eq!(-1010023630, my_atoi(" -1010023630o4".to_string()));
}

fn my_atoi(str: String) -> i32 {
    let mut res: i32 = 0;
    let mut sign: i32 = 1;
    let mut saw_sign = false;
    let mut saw_digit = false;
    for c in str.as_bytes() {
        match *c as char {
            '0'..='9' => {
                match res.checked_mul(10) {
                    None => return if sign == 1 { i32::max_value() } else { i32::min_value() },
                    Some(tmp) => res = tmp,
                }
                match res.checked_add((c - 48) as i32) {
                    None => return if sign == 1 { i32::max_value() } else { i32::min_value() },
                    Some(tmp) => res = tmp,
                }
                saw_digit = true;
            }
            '-' => {
                if res != 0 || saw_sign || saw_digit {
                    return res * sign;
                }
                sign = -1;
                saw_sign = true;
            }
            '+' => {
                if saw_sign || saw_digit {
                    return res * sign;
                }
                sign = 1;
                saw_sign = true;
            }
            ' ' => {
                if saw_digit || saw_sign {
                    break;
                }
            }
            _ => break,
        }
    }
    res * sign
}