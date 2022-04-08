pub mod delta_neutral_position;
pub mod perp_info;
pub use delta_neutral_position::*;
pub use perp_info::*;

pub fn dist(a: i64, b: i64) -> u64 {
    if a < b {
        (b as u64).wrapping_sub(a as u64)
    } else {
        (a as u64).wrapping_sub(b as u64)
    }
}
