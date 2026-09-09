impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut current_count: i32 = 0;
        let mut max_count: i32 = 0;

        for num in nums {
            if num == 1 {
                current_count += 1;
                if current_count > max_count {
                    max_count = current_count;
                }
            }
            if num == 0 {
                current_count = 0;
            }
        }

        max_count
    }
}
