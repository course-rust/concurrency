// Metrics data struct
// 基本功能：inc/dec/snapshot

use std::fmt::{Display, Formatter};
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct CMapMetrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}
impl CMapMetrics {
    pub fn new() -> Self {
        CMapMetrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }
    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;

        Ok(())
    }
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl Default for CMapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CMapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read().map_err(|_e| fmt::Error {})?;
        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
