use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use distributed_ledger::{DistributedLedger, Transaction};
use tokio::runtime::Runtime;

fn bench_transaction_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("transaction_throughput");
    
    for batch_size in [100, 500, 1000, 2000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("batch_processing", batch_size),
            batch_size,
            |b, &batch_size| {
                b.to_async(&rt).iter(|| async {
                    let ledger = DistributedLedger::new();
                    
                    // Create test transactions
                    let transactions: Vec<Transaction> = (0..batch_size)
                        .map(|i| {
                            Transaction::new(
                                format!("sender_{}", i % 100),
                                format!("receiver_{}", (i + 1) % 100),
                                1000,
                            )
                        })
                        .collect();
                    
                    // Add transactions to ledger
                    for tx in transactions {
                        let _ = ledger.add_transaction(tx).await;
                    }
                    
                    // Process transactions
                    let _ = ledger.process_transactions(batch_size).await;
                    
                    black_box(ledger.get_performance_stats())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_concurrent_transactions(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("concurrent_transaction_processing", |b| {
        b.to_async(&rt).iter(|| async {
            let ledger = DistributedLedger::new();
            let ledger_clone = ledger.clone();
            
            // Start background processor
            ledger_clone.start_background_processor().await;
            
            // Create many transactions concurrently
            let handles: Vec<_> = (0..1000)
                .map(|i| {
                    let ledger = ledger.clone();
                    tokio::spawn(async move {
                        for j in 0..10 {
                            let tx = Transaction::new(
                                format!("sender_{}", (i * 10 + j) % 100),
                                format!("receiver_{}", (i * 10 + j + 1) % 100),
                                1000,
                            );
                            let _ = ledger.add_transaction(tx).await;
                        }
                    })
                })
                .collect();
            
            // Wait for all transactions to be added
            for handle in handles {
                let _ = handle.await;
            }
            
            // Wait a bit for processing
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            black_box(ledger.get_performance_stats())
        });
    });
}

criterion_group!(benches, bench_transaction_throughput, bench_concurrent_transactions);
criterion_main!(benches);