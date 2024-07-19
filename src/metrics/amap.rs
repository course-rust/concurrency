use std::fmt::{Display, Formatter};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use anyhow::{Ok, Result};

/// Atomic Map
#[derive(Debug)]
pub struct AMapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}
impl Clone for AMapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}
impl AMapMetrics {
    pub fn new(metrics_name: &[&'static str]) -> Self {
        let map = metrics_name
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();

        AMapMetrics {
            data: Arc::new(map),
        }
    }
    pub fn inc(&mut self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?;

        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
impl Display for AMapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }
        Result::Ok(())
    }
}
