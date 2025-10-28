fn simulator_move(len: usize, start_index: usize, nums: &[i32], forward: i32) -> bool {
    let mut i = start_index as i32;
    let mut temp = nums.to_vec();
    let mut direction = forward;


    while i < len as i32 && i >= 0 {
        let current_index = i as usize;

        if temp[current_index] == 0 {
            i += direction;
        } else {
            temp[current_index] -= 1;
            direction *= -1;
            i += direction;
        }
    }
    temp.iter().all(|&x| x == 0)
}

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut zero_positions  = Vec::new();

        for (index, &value) in nums.iter().enumerate() {
            if value == 0 {
                zero_positions .push(index);
            }
        }

        let len = nums.len();
        for &start_index  in &zero_positions {
            if simulator_move(len, start_index , &nums, 1) {
                ans += 1;
            }
            if simulator_move(len, start_index , &nums, -1) {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let test = vec![2,3,4,0,4,1,0];
    let result = Solution::count_valid_selections(test);
    println!("{}", result);
}
