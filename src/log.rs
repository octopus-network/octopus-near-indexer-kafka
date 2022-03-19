use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::cli::indexer::{INDEXER, Stats};
use crate::utils::fetch_latest_block;


pub fn init_tracing() {
    tracing_subscriber::fmt::Subscriber::builder().init();
}

pub async fn indexer_logger(
    stats: Arc<Mutex<Stats>>,
    view_client: actix::Addr<near_client::ViewClientActor>,
) {
    let interval_secs = 10;
    let mut prev_blocks_processed_count: u64 = 0;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
        let stats_lock = stats.lock().await;
        let stats_copy = stats_lock.clone();
        drop(stats_lock);

        let block_processing_speed: f64 = ((stats_copy.blocks_processed_count
            - prev_blocks_processed_count) as f64)
            / (interval_secs as f64);

        let time_to_catch_the_tip_duration =
            if let Ok(block_height) = fetch_latest_block(&view_client).await {
                Some(std::time::Duration::from_millis(
                    if  block_height - stats_copy.last_processed_block_height == 0 || block_processing_speed == 0.0 {
                        0
                    } else {
                        (block_height - stats_copy.last_processed_block_height)
                            / ((block_processing_speed * 1_000f64) as u64)
                    }
                ))
            } else {
                None
            };

        tracing::info!(
            target: INDEXER,
            "# {} | Blocks processing: {}| Blocks done: {}. Bps {:.2} b/s {}",
            stats_copy.last_processed_block_height,
            stats_copy.block_heights_processing.len(),
            stats_copy.blocks_processed_count,
            block_processing_speed,
            if let Some(duration) = time_to_catch_the_tip_duration {
                format!(
                    " | {} to catch up the tip",
                    humantime::format_duration(duration)
                )
            } else {
                "".to_string()
            }
        );
        prev_blocks_processed_count = stats_copy.blocks_processed_count;
    }
}
