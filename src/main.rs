use distributed_ledger::{DistributedLedger, Transaction};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Distributed Ledger - High Performance Demo");
    println!("Target: 10,000+ transactions per second");
    println!();
    
    // Create ledger
    let ledger = DistributedLedger::new();
    let ledger_clone = ledger.clone();
    
    // Start background processor
    ledger_clone.start_background_processor().await;
    
    // Create test accounts
    let accounts = vec![
        "alice".to_string(),
        "bob".to_string(),
        "charlie".to_string(),
        "diana".to_string(),
        "eve".to_string(),
    ];
    
    // Initialize accounts with balances
    println!("💰 Initializing accounts with 1M balance each...");
    for account in &accounts {
        let tx = Transaction::new(
            "".to_string(), // Genesis transaction
            account.clone(),
            1_000_000,
        );
        ledger.add_transaction(tx).await?;
    }
    
    // Wait for initialization
    sleep(Duration::from_millis(100)).await;
    
    println!("✅ {} accounts initialized", accounts.len());
    println!();
    
    // High-volume transaction test
    let transaction_count = 25_000; // Conservative test
    println!("📊 Generating {} transactions...", transaction_count);
    
    let start_time = std::time::Instant::now();
    
    for i in 0..transaction_count {
        let from = &accounts[i % accounts.len()];
        let to = &accounts[(i + 1) % accounts.len()];
        let amount = (i % 1000) + 1;
        
        let tx = Transaction::new(from.clone(), to.clone(), amount);
        ledger.add_transaction(tx).await?;
        
        // Progress indicator
        if (i + 1) % 5_000 == 0 {
            let stats = ledger.get_performance_stats();
            println!("  Processed {} transactions, Current TPS: {:.0}", 
                i + 1, stats.transactions_per_second);
        }
    }
    
    println!("⏳ Processing remaining transactions...");
    sleep(Duration::from_secs(3)).await;
    
    let total_time = start_time.elapsed();
    let stats = ledger.get_performance_stats();
    
    println!();
    println!("📈 Performance Results:");
    println!("  Total transactions: {}", stats.total_transactions);
    println!("  Total time: {:.2}s", total_time.as_secs_f64());
    println!("  Average TPS: {:.0}", stats.transactions_per_second);
    println!("  Peak TPS: {:.0}", stats.peak_tps);
    println!("  Average batch time: {:?}", stats.average_batch_time);
    
    // Performance validation
    println!();
    if stats.transactions_per_second >= 10_000.0 {
        println!("🎉 SUCCESS: Achieved target of 10,000+ TPS!");
        println!("   Actual performance: {:.0} TPS", stats.transactions_per_second);
    } else {
        println!("⚠️  Performance below target");
        println!("   Target: 10,000 TPS");
        println!("   Actual: {:.0} TPS", stats.transactions_per_second);
        println!("   Note: Performance depends on hardware capabilities");
    }
    
    // Show final balances
    println!();
    println!("💰 Final Account Balances:");
    for account in &accounts {
        let balance = ledger.get_balance(account).await;
        println!("  {}: {}", account, balance);
    }
    
    println!();
    println!("✅ Demo completed successfully!");
    
    Ok(())
}