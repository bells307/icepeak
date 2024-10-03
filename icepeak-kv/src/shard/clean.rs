use super::Shard;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

/// The frequency of active cleaning execution
const ACTIVE_CLEANING_PERIOD: Duration = Duration::from_secs(1);
/// The number of checks for potentially expired keys
const RANDOM_KEY_EXPIRE_CHECK_COUNT: usize = 20;
/// The ratio of expired keys to the number of checks at which the check stops
const EXPIRE_KEYS_THRESHOLD: f32 = 0.2;

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
        let mut rng = thread_rng();

        loop {
            // The number of expired keys in this iteration of the check
            let mut expired = 0;
            // The number of checks we need to perform
            let mut check_count = RANDOM_KEY_EXPIRE_CHECK_COUNT;

            let keys_count = self.0.keys.read().len();

            // If there are fewer total keys than the number of checks, there's no need to run the loop unnecessarily
            if keys_count < check_count {
                check_count = keys_count
            }

            for _ in 0..check_count {
                if self.random_key_expire_check(&mut rng) {
                    expired += 1;
                }
            }

            // If the percentage of expired keys is higher than the threshold, we'll repeat the cleaning process
            // Otherwise, we end the loop
            if (expired as f32 / check_count as f32) < EXPIRE_KEYS_THRESHOLD {
                break;
            }
        }
    }

    /// Take a random key from the hashmap and check if it has expired. If it has, delete it on the spot.
    /// Returns `true` if the random key has expired.
    fn random_key_expire_check(&self, rng: &mut ThreadRng) -> bool {
        let keys_lock = self.0.keys.read();

        if keys_lock.is_empty() {
            return false;
        }

        // Get a random key
        let rand_idx = rng.gen_range(0..keys_lock.len() - 1);
        let rand_key = keys_lock
            .get(rand_idx)
            .expect("rand_idx is not out of bounds");

        let map_lock = self.0.map.read();

        // Get the value for the random key
        let Some(val) = map_lock.get(rand_key) else {
            return false;
        };

        if val.expired() {
            let rand_key = rand_key.clone();

            drop(keys_lock);
            drop(map_lock);

            self.0.remove(&rand_key);
            true
        } else {
            false
        }
    }
}
