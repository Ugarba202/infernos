use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ClientBudgetTracker {
    allocated_sats: u64,
    spent_sats: Arc<AtomicU64>,
}

impl ClientBudgetTracker {
    pub fn new(allocated_sats: u64) -> Self {
        Self {
            allocated_sats,
            spent_sats: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_spend(&self, amount_sats: u64) -> bool {
        let current = self.spent_sats.load(Ordering::SeqCst);
        if current + amount_sats > self.allocated_sats {
            return false;
        }
        self.spent_sats.fetch_add(amount_sats, Ordering::SeqCst);
        true
    }

    pub fn remaining(&self) -> u64 {
        let spent = self.spent_sats.load(Ordering::SeqCst);
        self.allocated_sats.saturating_sub(spent)
    }
}
