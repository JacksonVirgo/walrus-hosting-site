#[cfg(test)]
mod tests {
    use crate::utils::snowflake::{CUSTOM_EPOCH, SnowflakeGenerator, SnowflakePayload};

    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn snowflake_roundtrip() {
        let snowflake = SnowflakePayload {
            timestamp: CUSTOM_EPOCH + 1_000_000,
            worker_id: 0x1F & 3,
            process_id: 0x1F & 7,
            increment: 0xFF & 42,
        };

        let value = snowflake.to_snowflake();
        let decoded = SnowflakePayload::from_snowflake(value);

        assert_eq!(snowflake.timestamp, decoded.timestamp);
        assert_eq!(snowflake.worker_id, decoded.worker_id);
        assert_eq!(snowflake.process_id, decoded.process_id);
        assert_eq!(snowflake.increment, decoded.increment);
    }

    #[test]
    fn global_uniqueness() {
        let thread_count = 8;
        let ids_per_thread = 10_000;
        let total = thread_count * ids_per_thread;

        let set = Arc::new(Mutex::new(HashSet::with_capacity(total)));

        let mut handles = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            let set_cloned = Arc::clone(&set);
            handles.push(thread::spawn(move || {
                for _ in 0..ids_per_thread {
                    let sf = SnowflakePayload::new().expect("Snowflake failed to be generated");
                    let v = sf.to_snowflake();
                    let mut guard = set_cloned.lock().unwrap();
                    let inserted = guard.insert(v);
                    if !inserted {
                        panic!("Duplicate snowflake detected: {}", v);
                    }
                }
            }));
        }

        for h in handles {
            h.join().expect("thread panicked");
        }

        let guard = set.lock().unwrap();
        assert_eq!(guard.len(), total);
    }

    #[test]
    fn local_sequence() {
        let mut generator = SnowflakeGenerator::new(1, 1);
        let first = generator.next().unwrap();
        let second = generator.next().unwrap();

        assert!(
            first.to_snowflake() != second.to_snowflake(),
            "Consequtive snowflakes must differ"
        );

        if first.timestamp == second.timestamp {
            assert!(second.increment > first.increment);
        }
    }

    #[test]
    fn high_volume_consequtive_unique() {
        let mut generator = SnowflakeGenerator::new(2, 2);
        let mut seen = HashSet::new();
        let n = 50_000usize;
        for _ in 0..n {
            let sf = generator.next().unwrap();
            let v = sf.to_snowflake();
            if !seen.insert(v) {
                panic!("Duplicate seen in single generator run: {}", v);
            }
        }
        assert_eq!(seen.len(), n);
    }
}
