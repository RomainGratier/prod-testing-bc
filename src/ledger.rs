use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;
use crossbeam_channel::{bounded, Receiver, Sender};
use rayon::prelude::*;
use tracing::{info, warn, error};

use crate::{Transaction, Block, LedgerError, Result};
use crate::performance::PerformanceMonitor;

pub struct DistributedLedger {
    blocks: Arc<RwLock<Vec<Block>>>,
    balances: Arc<DashMap<String, u64>>,
    transaction_pool: Arc<DashMap<uuid::Uuid, Transaction>>,
    performance_monitor: Arc<PerformanceMonitor>,
    tx_sender: Sender<Transaction>,
    tx_receiver: Receiver<Transaction>,
}

impl DistributedLedger {
    pub fn new() -> Self {
        let (tx_sender, tx_receiver) = bounded(100_000); // Large buffer for high throughput
        
        let ledger = Self {
            blocks: Arc::new(RwLock::new(Vec::new())),
            balances: Arc::new(DashMap::new()),
            transaction_pool: Arc::new(DashMap::new()),
            performance_monitor: Arc::new(PerformanceMonitor::new()),
            tx_sender,
            tx_receiver,
        };
        
        // Initialize with genesis block
        ledger.initialize_genesis_block();
        ledger
    }
    
    fn initialize_genesis_block(&self) {
        let genesis_block = Block::new(String::new(), Vec::new());
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let mut blocks = self.blocks.write().await;
                blocks.push(genesis_block);
            });
        });
    }
    
    pub async fn add_transaction(&self, transaction: Transaction) -> Result<()> {
        // Validate transaction
        transaction.validate()?;
        
        // Check for duplicates
        if self.transaction_pool.contains_key(&transaction.id) {
            return Err(LedgerError::DuplicateTransaction);
        }
        
        // Check balance (for non-genesis transactions)
        if !transaction.from.is_empty() {
            let current_balance = self.balances.get(&transaction.from)
                .map(|entry| *entry.value())
                .unwrap_or(0);
            
            if current_balance < transaction.amount {
                return Err(LedgerError::InsufficientBalance);
            }
        }
        
        // Add to transaction pool
        self.transaction_pool.insert(transaction.id, transaction.clone());
        
        // Send to processing queue
        if let Err(_) = self.tx_sender.try_send(transaction) {
            return Err(LedgerError::PerformanceLimitExceeded(
                "Transaction queue is full".to_string(),
            ));
        }
        
        Ok(())
    }
    
    pub async fn process_transactions(&self, batch_size: usize) -> Result<()> {
        let mut transactions = Vec::new();
        
        // Collect transactions from the queue
        for _ in 0..batch_size {
            if let Ok(tx) = self.tx_receiver.try_recv() {
                transactions.push(tx);
            } else {
                break;
            }
        }
        
        if transactions.is_empty() {
            return Ok(());
        }
        
        // Process transactions in parallel
        let start_time = std::time::Instant::now();
        
        // Update balances
        for tx in &transactions {
            if !tx.from.is_empty() {
                self.balances.entry(tx.from.clone()).and_modify(|balance| {
                    *balance -= tx.amount;
                }).or_insert(0);
            }
            
            self.balances.entry(tx.to.clone()).and_modify(|balance| {
                *balance += tx.amount;
            }).or_insert(tx.amount);
        }
        
        // Create new block
        let previous_hash = {
            let blocks = self.blocks.read().await;
            blocks.last().map(|b| b.hash.clone()).unwrap_or_default()
        };
        
        let mut new_block = Block::new(previous_hash, transactions);
        new_block.mine(2); // Simple proof-of-work
        
        // Validate and add block
        new_block.validate(Some(&self.get_latest_block().await))?;
        
        {
            let mut blocks = self.blocks.write().await;
            blocks.push(new_block);
        }
        
        let processing_time = start_time.elapsed();
        self.performance_monitor.record_batch(transactions.len(), processing_time);
        
        info!("Processed {} transactions in {:?}", transactions.len(), processing_time);
        
        Ok(())
    }
    
    pub async fn get_latest_block(&self) -> Block {
        let blocks = self.blocks.read().await;
        blocks.last().unwrap().clone()
    }
    
    pub async fn get_balance(&self, address: &str) -> u64 {
        self.balances.get(address)
            .map(|entry| *entry.value())
            .unwrap_or(0)
    }
    
    pub async fn get_transaction_count(&self) -> usize {
        let blocks = self.blocks.read().await;
        blocks.iter().map(|b| b.transactions.len()).sum()
    }
    
    pub fn get_performance_stats(&self) -> crate::performance::PerformanceStats {
        self.performance_monitor.get_stats()
    }
    
    pub async fn start_background_processor(&self) {
        let ledger = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(10));
            
            loop {
                interval.tick().await;
                
                if let Err(e) = ledger.process_transactions(1000).await {
                    error!("Error processing transactions: {}", e);
                }
            }
        });
    }
}

impl Clone for DistributedLedger {
    fn clone(&self) -> Self {
        Self {
            blocks: Arc::clone(&self.blocks),
            balances: Arc::clone(&self.balances),
            transaction_pool: Arc::clone(&self.transaction_pool),
            performance_monitor: Arc::clone(&self.performance_monitor),
            tx_sender: self.tx_sender.clone(),
            tx_receiver: self.tx_receiver.clone(),
        }
    }
}