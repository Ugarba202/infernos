use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SessionBudget {
    pub total_sats: u64,
    pub remaining_sats: Arc<AtomicU64>,
}

impl SessionBudget {
    pub fn new(total_sats: u64) -> Self {
        Self {
            total_sats,
            remaining_sats: Arc::new(AtomicU64::new(total_sats)),
        }
    }

    pub fn debit(&self, amount_sats: u64) -> bool {
        let mut current = self.remaining_sats.load(Ordering::SeqCst);
        loop {
            if current < amount_sats {
                return false;
            }
            match self.remaining_sats.compare_exchange_weak(
                current,
                current - amount_sats,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
    }

    pub fn remaining(&self) -> u64 {
        self.remaining_sats.load(Ordering::SeqCst)
    }
}
