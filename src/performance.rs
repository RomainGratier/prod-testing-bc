use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub total_transactions: u64,
    pub transactions_per_second: f64,
    pub average_batch_time: Duration,
    pub peak_tps: f64,
}

pub struct PerformanceMonitor {
    stats: Arc<RwLock<PerformanceData>>,
}

struct PerformanceData {
    total_transactions: u64,
    batch_times: VecDeque<Duration>,
    batch_sizes: VecDeque<usize>,
    peak_tps: f64,
    last_reset: Instant,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(PerformanceData {
                total_transactions: 0,
                batch_times: VecDeque::new(),
                batch_sizes: VecDeque::new(),
                peak_tps: 0.0,
                last_reset: Instant::now(),
            })),
        }
    }
    
    pub async fn record_batch(&self, batch_size: usize, processing_time: Duration) {
        let mut data = self.stats.write().await;
        
        data.total_transactions += batch_size as u64;
        data.batch_times.push_back(processing_time);
        data.batch_sizes.push_back(batch_size);
        
        // Keep only last 100 batches for rolling average
        if data.batch_times.len() > 100 {
            data.batch_times.pop_front();
            data.batch_sizes.pop_front();
        }
        
        // Calculate current TPS
        let current_tps = if processing_time.as_secs_f64() > 0.0 {
            batch_size as f64 / processing_time.as_secs_f64()
        } else {
            0.0
        };
        
        if current_tps > data.peak_tps {
            data.peak_tps = current_tps;
        }
    }
    
    pub fn get_stats(&self) -> PerformanceStats {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let data = self.stats.read().await;
                
                let total_time = data.last_reset.elapsed();
                let overall_tps = if total_time.as_secs_f64() > 0.0 {
                    data.total_transactions as f64 / total_time.as_secs_f64()
                } else {
                    0.0
                };
                
                let avg_batch_time = if !data.batch_times.is_empty() {
                    data.batch_times.iter().sum::<Duration>() / data.batch_times.len() as u32
                } else {
                    Duration::from_millis(0)
                };
                
                PerformanceStats {
                    total_transactions: data.total_transactions,
                    transactions_per_second: overall_tps,
                    average_batch_time: avg_batch_time,
                    peak_tps: data.peak_tps,
                }
            })
        })
    }
}