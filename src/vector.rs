/* Copyright (C) 2023-2024 Hangzhou HSH Co. Ltd.
 * All right reserved.*/

use std::ops::{Add, AddAssign, Deref, Mul};

use anyhow::anyhow;

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// pretend this is a heavy operation, CPU intensive
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        // a.len() => a.data.len() (deref trait)
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}
