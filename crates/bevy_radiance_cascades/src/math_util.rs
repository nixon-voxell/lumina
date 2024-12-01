use bevy::prelude::*;

pub fn batch_count(length: UVec3, batch_size: UVec3) -> UVec3 {
    (length + batch_size - 1) / batch_size
}

/// Fast log2 ceil based on:
///
/// https://stackoverflow.com/questions/72251467/computing-ceil-of-log2-in-rust
pub fn fast_log2_ceil(number: u32) -> u32 {
    u32::BITS - u32::leading_zeros(number)
}
