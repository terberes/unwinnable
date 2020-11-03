
pub fn calculate_strategy(ball_count: u32, allowed_range: &Vec<u32>) -> Vec<u32> {
    let mut dp = vec![0; ball_count as usize + 1];

    'outer: for i in 1..=ball_count {
        // Для максимальной производительности вектор должен быть отсортирован.
        // Он сортируется в `parse_number_selection()`
        for j in allowed_range.iter().rev() {
            if i >= *j && dp[(i - j) as usize] == 0 {
                dp[i as usize] = *j;
                continue 'outer;
            }
        }
    };
    dp.remove(0);
    // dp.reverse();
    dp
}