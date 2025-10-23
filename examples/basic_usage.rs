use distributed_ledger::{DistributedLedger, Transaction};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Starting Distributed Ledger Demo");
    println!("Target: 10,000+ transactions per second");
    
    // Create ledger
    let ledger = DistributedLedger::new();
    let ledger_clone = ledger.clone();
    
    // Start background processor
    ledger_clone.start_background_processor().await;
    
    // Create test accounts with initial balances
    let accounts = vec![
        "alice".to_string(),
        "bob".to_string(),
        "charlie".to_string(),
        "diana".to_string(),
    ];
    
    // Give initial balances to accounts
    for account in &accounts {
        let tx = Transaction::new(
            "".to_string(), // Empty sender for genesis transaction
            account.clone(),
            1_000_000, // 1M initial balance
        );
        ledger.add_transaction(tx).await?;
    }
    
    println!("✅ Initial balances set for {} accounts", accounts.len());
    
    // Generate high-volume transactions
    let start_time = std::time::Instant::now();
    let transaction_count = 50_000; // Target for testing
    
    println!("📊 Generating {} transactions...", transaction_count);
    
    for i in 0..transaction_count {
        let from = &accounts[i % accounts.len()];
        let to = &accounts[(i + 1) % accounts.len()];
        let amount = (i % 1000) + 1; // Varying amounts
        
        let tx = Transaction::new(from.clone(), to.clone(), amount);
        ledger.add_transaction(tx).await?;
        
        // Print progress every 10k transactions
        if (i + 1) % 10_000 == 0 {
            let stats = ledger.get_performance_stats();
            println!("Processed {} transactions, Current TPS: {:.2}", 
                i + 1, stats.transactions_per_second);
        }
    }
    
    // Wait for all transactions to be processed
    println!("⏳ Waiting for all transactions to be processed...");
    sleep(Duration::from_secs(5)).await;
    
    let total_time = start_time.elapsed();
    let stats = ledger.get_performance_stats();
    
    println!("\n📈 Performance Results:");
    println!("Total transactions: {}", stats.total_transactions);
    println!("Total time: {:.2}s", total_time.as_secs_f64());
    println!("Average TPS: {:.2}", stats.transactions_per_second);
    println!("Peak TPS: {:.2}", stats.peak_tps);
    println!("Average batch time: {:?}", stats.average_batch_time);
    
    // Check final balances
    println!("\n💰 Final Account Balances:");
    for account in &accounts {
        let balance = ledger.get_balance(account).await;
        println!("{}: {}", account, balance);
    }
    
    // Validate performance target
    if stats.transactions_per_second >= 10_000.0 {
        println!("\n🎉 SUCCESS: Achieved target of 10,000+ TPS!");
    } else {
        println!("\n⚠️  WARNING: Did not reach target of 10,000 TPS");
    }
    
    Ok(())
}