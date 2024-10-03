use super::Shard;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

/// The frequency of active cleaning execution
const ACTIVE_CLEANING_PERIOD: Duration = Duration::from_secs(1);
/// The number of checks for potentially expired keys
const RANDOM_KEY_EXPIRE_CHECK_COUNT: usize = 20;

pub(super) struct ShardCleaner(Shard);

impl ShardCleaner {
    pub(super) fn run(ct: CancellationToken, shard: Shard) {
        let sc = Self(shard);
        tokio::spawn(sc.clean_task(ct));
    }

    async fn clean_task(self, ct: CancellationToken) {
        loop {
            tokio::select! {
                _ = ct.cancelled() => break,
                _ = tokio::time::sleep(ACTIVE_CLEANING_PERIOD) => self.clean().await
            }
        }
    }

    async fn clean(&self) {
        loop {
            // The number of expired keys in this iteration of the check
            let expired = 0;
            let mut check_count = RANDOM_KEY_EXPIRE_CHECK_COUNT;

            let keys_count = self.0.count_keys();

            if keys_count < check_count {
                check_count = keys_count
            }
        }
    }
}
