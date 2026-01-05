use alloy::primitives::U256;

use crate::result::{AppErr, Rs};

pub trait CheckedPercent: Sized {
    fn checked_percent(&self, percent: u8) -> Rs<Self>;
    fn checked_percent_f32(&self, percent: f32) -> Rs<Self>;
}

pub trait Percent {
    type Output;

    fn percent(&self, percent: u8) -> Self::Output;
    fn percent_f32(&self, percent: f32) -> Self::Output;
}

impl Percent for f64 {
    type Output = f64;

    fn percent(&self, percent: u8) -> f64 {
        self / 100_f64 * (percent as f64)
    }

    fn percent_f32(&self, percent: f32) -> f64 {
        (*self) / 100f64 * (percent as f64)
    }
}

impl Percent for u64 {
    type Output = u64;

    fn percent(&self, percent: u8) -> u64 {
        self / 100u64 * (percent as u64)
    }

    fn percent_f32(&self, percent: f32) -> u64 {
        ((*self as f64) / 100f64 * (percent as f64)) as u64
    }
}

impl Percent for u128 {
    type Output = u128;

    fn percent(&self, percent: u8) -> u128 {
        self / 100u128 * (percent as u128)
    }

    fn percent_f32(&self, percent: f32) -> u128 {
        ((*self as f64) / 100f64 * (percent as f64)) as u128
    }
}

impl CheckedPercent for U256 {
    fn checked_percent(&self, percent: u8) -> Rs<Self> {
        self.checked_div(Self::from(100))
            .and_then(|amount| amount.checked_mul(Self::from(percent)))
            .ok_or(AppErr::custom("Operate units none error"))
    }

    fn checked_percent_f32(&self, percent: f32) -> Rs<Self> {
        let precision = 1_000_000f32;
        let precision_unit = Self::from(precision as u64);

        let percent_buffered = Self::from(percent * precision);

        self.checked_mul(precision_unit)
            .and_then(|amount| amount.checked_div(Self::from(100)))
            .and_then(|amount| amount.checked_mul(percent_buffered))
            .and_then(|amount| amount.checked_div(precision_unit))
            .and_then(|amount| amount.checked_div(precision_unit))
            .ok_or(AppErr::custom("Operate units none error"))
    }
}
