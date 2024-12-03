use bevy::prelude::*;

pub fn batch_count(length: UVec3, batch_size: UVec3) -> UVec3 {
    (length + batch_size - 1) / batch_size
}

/// Fast log2 ceil based on:
///
/// <https://stackoverflow.com/questions/72251467/computing-ceil-of-log2-in-rust>
pub fn fast_log2_ceil(number: u32) -> u32 {
    u32::BITS - u32::leading_zeros(number)
}

/// Calculate cascade count based on target max length and initial interval length.
///
/// # How it works?
///
/// The sum of all intervals can be achieved using geometric sequence:
/// <https://saylordotorg.github.io/text_intermediate-algebra/s12-03-geometric-sequences-and-series.html>
///
/// Formula: Sn = a1(1−r^n)/(1−r)
/// Where:
/// - Sn: sum of all intervals
/// - a1: first interval
/// -  r: factor (4 as each interval increases its length by 4 every new cascade)
/// -  n: number of cascades
///
/// The goal here is to find n such that Sn < max_length.
/// let x = max_length
///
/// Factoring in the numbers:
/// x > Sn
/// x > a1(1−4^n)/-3
///
/// Rearranging the equation:
/// -3(x) > a1(1−4^n)
/// -3(x)/a1 > 1−4^n
/// 4^n > 1 + 3(x)/a1
/// n > log4(1 + 3(x)/a1)
pub fn cascade_count(max_length: f32, init_interval: f32) -> usize {
    // Ceil is used becaues n should be greater than the value we get.
    f32::log(1.0 + 3.0 * max_length / init_interval, 4.0).ceil() as usize
}
