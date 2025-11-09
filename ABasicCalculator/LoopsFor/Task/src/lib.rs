// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    let mut res = 1u32;
    for i in 1..=n {
        res *= i;
    }
    res
}
