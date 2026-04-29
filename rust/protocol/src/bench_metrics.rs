use std::sync::atomic::{AtomicU64, Ordering};

static DR_DH_RATCHET_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn reset_bench_dr_dh_ratchet_count() {
    DR_DH_RATCHET_COUNT.store(0, Ordering::Relaxed);
}

pub fn bench_dr_dh_ratchet_count() -> u64 {
    DR_DH_RATCHET_COUNT.load(Ordering::Relaxed)
}

pub(crate) fn increment_bench_dr_dh_ratchet_count() {
    DR_DH_RATCHET_COUNT.fetch_add(1, Ordering::Relaxed);
}
