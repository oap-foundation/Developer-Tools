use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Default)]
pub struct Metrics {
    pub total_requests: AtomicU64,
    pub successful_forwards: AtomicU64,
    pub dropped_packets: AtomicU64,
    pub corrupted_requests: AtomicU64,
    pub replayed_requests: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc_total(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_success(&self) {
        self.successful_forwards.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_dropped(&self) {
        self.dropped_packets.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_corrupted(&self) {
        self.corrupted_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_replayed(&self) {
        self.replayed_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn print_report(&self) {
        let total = self.total_requests.load(Ordering::Relaxed);
        let success = self.successful_forwards.load(Ordering::Relaxed);
        let dropped = self.dropped_packets.load(Ordering::Relaxed);
        let corrupted = self.corrupted_requests.load(Ordering::Relaxed);
        let replayed = self.replayed_requests.load(Ordering::Relaxed);

        info!("📊 --- OAP CHAOS MONKEY REPORT ---");
        info!("Total Requests:      {}", total);
        info!("Successful Forwards: {}", success);
        info!("Dropped (Loss):      {}", dropped);
        info!("Corrupted/Sabotaged: {}", corrupted);
        info!("Replayed (Security): {}", replayed);
        info!("---------------------------------");
    }
}
