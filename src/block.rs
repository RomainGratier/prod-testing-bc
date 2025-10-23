use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let nonce = 0;
        
        let mut block = Self {
            id,
            previous_hash,
            transactions,
            timestamp,
            nonce,
            hash: String::new(),
        };
        
        block.hash = block.calculate_hash();
        block
    }
    
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.timestamp.timestamp().to_le_bytes());
        hasher.update(self.nonce.to_le_bytes());
        
        // Include transaction hashes
        for tx in &self.transactions {
            hasher.update(tx.hash().as_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }
    
    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
    
    pub fn validate(&self, previous_block: Option<&Block>) -> crate::Result<()> {
        // Validate hash
        if self.hash != self.calculate_hash() {
            return Err(crate::LedgerError::BlockValidationFailed(
                "Invalid block hash".to_string(),
            ));
        }
        
        // Validate previous hash
        if let Some(prev) = previous_block {
            if self.previous_hash != prev.hash {
                return Err(crate::LedgerError::BlockValidationFailed(
                    "Invalid previous hash".to_string(),
                ));
            }
        } else if !self.previous_hash.is_empty() {
            return Err(crate::LedgerError::BlockValidationFailed(
                "Genesis block should have empty previous hash".to_string(),
            ));
        }
        
        // Validate transactions
        for tx in &self.transactions {
            tx.validate()?;
        }
        
        Ok(())
    }
}