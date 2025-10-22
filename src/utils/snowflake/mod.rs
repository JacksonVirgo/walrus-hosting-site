use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod tests;

const CUSTOM_EPOCH: u64 = 1420070400000;

#[derive(Debug, Clone, Copy)]
pub struct Snowflake {
    pub timestamp: u64,
    pub worker_id: u64,
    pub process_id: u64,
    pub increment: u64,
}

impl Snowflake {
    pub fn new() -> anyhow::Result<Snowflake> {
        let mut generator = SNOWFLAKE
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        generator.next()
    }

    pub fn to_u64(&self) -> u64 {
        ((self.timestamp - CUSTOM_EPOCH) << 22)
            | (self.worker_id << 17)
            | (self.process_id << 12)
            | self.increment
    }

    pub fn from_u64(value: u64) -> Self {
        let timestamp = (value >> 22) + CUSTOM_EPOCH;
        let worker_id = (value >> 17) & 0x1F;
        let process_id = (value >> 12) & 0x1F;
        let increment = value & 0xFFF;
        Self {
            timestamp,
            worker_id,
            process_id,
            increment,
        }
    }
}

pub struct SnowflakeGenerator {
    worker_id: u64,
    process_id: u64,
    increment: u64,
    last_timestamp: u64,
}

impl SnowflakeGenerator {
    pub fn new(worker_id: u64, process_id: u64) -> Self {
        Self {
            worker_id: worker_id & 0x1F,
            process_id: process_id & 0x1F,
            increment: 0,
            last_timestamp: 0,
        }
    }

    fn current_millis() -> anyhow::Result<u64> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(time.as_millis() as u64)
    }

    pub fn next(&mut self) -> anyhow::Result<Snowflake> {
        let mut timestamp = Self::current_millis()?;
        if timestamp == self.last_timestamp {
            self.increment = (self.increment + 1) & 0xFFF;
            if self.increment == 0 {
                while timestamp <= self.last_timestamp {
                    timestamp = Self::current_millis()?;
                }
            }
        } else {
            self.increment = 0;
        }

        self.last_timestamp = timestamp;

        Ok(Snowflake {
            timestamp,
            worker_id: self.worker_id,
            process_id: self.process_id,
            increment: self.increment,
        })
    }
}

// If this gets scaled each instance
// needs a different worker_id and process_id
pub static SNOWFLAKE: Lazy<Arc<Mutex<SnowflakeGenerator>>> =
    Lazy::new(|| Arc::new(Mutex::new(SnowflakeGenerator::new(1, 1))));
