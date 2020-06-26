fn main() {
    assert_eq!("PAHNAPLSIIGYIR", convert("PAYPALISHIRING".to_string(), 3));
    assert_eq!("PINALSIGYAHRPI", convert("PAYPALISHIRING".to_string(), 4));
    assert_eq!("AB", convert("AB".to_string(), 2));
    assert_eq!("ACB", convert("ABC".to_string(), 2));
}

/**
The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
P   A   H   N
A P L S I I G
Y   I   R
And then read line by line: "PAHNAPLSIIGYIR"

Write the code that will take a string and make this conversion given a number of rows:

string convert(string s, int numRows);
*/
pub fn convert(s: String, num_rows: i32) -> String {
    let n = s.len();
    if num_rows <= 1 || num_rows >= n as i32 { return s }
    let interval: usize = (2 * (num_rows - 1)) as usize;
    let mut chars = 0;
    let char_vec:Vec<char> = s.chars().collect();
    let mut str = String::new();
    for row in 0..num_rows {
        let mut i = 0;
        while i < n {

            if (i + row as usize) < n {
                // println!("1 i: {}, row: {}, char: {}", i, row, char_vec[i + row as usize]);
                str.push(char_vec[i + row as usize]);
                chars += 1;
            }
            if row != 0 && row != num_rows - 1 && ((i + interval) - (row as usize)) < n {
                // println!("2 i: {}, row: {}, char: {}", i, row, char_vec[(i + interval) - (row as usize)]);
                str.push(char_vec[(i + interval) - (row as usize)]);
                chars += 1;
            }
            i += interval;
        }

        if chars >= n {
            break;
        }
    }
    str
}