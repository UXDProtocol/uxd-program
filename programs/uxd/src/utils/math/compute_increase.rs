use crate::utils::checked_sub;
use anchor_lang::prelude::*;

pub fn compute_increase(before: u64, after: u64) -> Result<u64> {
    checked_sub(after, before)
}
