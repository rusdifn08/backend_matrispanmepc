use chrono::{DateTime, Duration, Utc};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FailedAttempt {
    pub count: u32,
    pub last_attempt: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct BruteForceProtector {
    attempts: Arc<RwLock<HashMap<String, FailedAttempt>>>,
}

impl Default for BruteForceProtector {
    fn default() -> Self {
        Self::new()
    }
}

impl BruteForceProtector {
    pub fn new() -> Self {
        Self {
            attempts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn is_locked(&self, identifier: &str) -> bool {
        let attempts = self.attempts.read();
        if let Some(entry) = attempts.get(identifier) {
            if let Some(locked_until) = entry.locked_until {
                if Utc::now() < locked_until {
                    return true;
                }
            }
        }
        false
    }

    pub fn record_failure(&self, identifier: &str) -> (u32, bool) {
        let mut attempts = self.attempts.write();
        let entry = attempts.entry(identifier.to_string()).or_insert(FailedAttempt {
            count: 0,
            last_attempt: Utc::now(),
            locked_until: None,
        });

        entry.count += 1;
        entry.last_attempt = Utc::now();

        // Lock after 5 consecutive failures for 5 minutes
        if entry.count >= 5 {
            entry.locked_until = Some(Utc::now() + Duration::minutes(5));
            (entry.count, true)
        } else {
            (entry.count, false)
        }
    }

    pub fn record_success(&self, identifier: &str) {
        let mut attempts = self.attempts.write();
        attempts.remove(identifier);
    }
}
