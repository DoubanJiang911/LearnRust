impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        println!("{:?}", sorted);
        let length = nums.len();
        let mut index = 0;
        while index < length-1 {
            index = index + 1;
            if sorted[index-1] == sorted[index] {
                ans.push(sorted[index-1]);
            }
        }
        ans
    }
}
