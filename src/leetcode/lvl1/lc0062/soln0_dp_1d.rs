impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n];
        
        for r in 1..m{
            for c in 1..n{                
                dp[c] = dp[c] + dp[c - 1];
            }
        }
        
        return dp[n - 1];
    }
}