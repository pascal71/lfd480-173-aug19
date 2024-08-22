#[inline]
pub fn fac(n: u64) -> u64 {
    match n {
        0 => 1,
        n => n * fac(n - 1),
    }
}
