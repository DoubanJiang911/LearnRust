struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let binary = format!("{:b}", n);
        let mut chars: Vec<char> = binary.chars().collect();
        for x in &mut chars {
            if *x == '0' {
                *x = '1';
            }
        }
        let s: String = chars.iter().collect();
        let num = u32::from_str_radix(s.as_str(), 2).unwrap();
        num as i32
    }
}

pub fn main() {
    let result = Solution::smallest_number(10);
    println!("{}", result);
}
