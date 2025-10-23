use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let signature = Self::calculate_signature(&id, &from, &to, amount, &timestamp);
        
        Self {
            id,
            from,
            to,
            amount,
            timestamp,
            signature,
        }
    }
    
    fn calculate_signature(
        id: &Uuid,
        from: &str,
        to: &str,
        amount: u64,
        timestamp: &DateTime<Utc>,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(from.as_bytes());
        hasher.update(to.as_bytes());
        hasher.update(amount.to_le_bytes());
        hasher.update(timestamp.timestamp().to_le_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn validate(&self) -> crate::Result<()> {
        if self.amount == 0 {
            return Err(crate::LedgerError::InvalidTransaction(
                "Amount must be greater than zero".to_string(),
            ));
        }
        
        if self.from == self.to {
            return Err(crate::LedgerError::InvalidTransaction(
                "Sender and receiver cannot be the same".to_string(),
            ));
        }
        
        if self.from.is_empty() || self.to.is_empty() {
            return Err(crate::LedgerError::InvalidTransaction(
                "From and to addresses cannot be empty".to_string(),
            ));
        }
        
        // Verify signature
        let expected_signature = Self::calculate_signature(
            &self.id,
            &self.from,
            &self.to,
            self.amount,
            &self.timestamp,
        );
        
        if self.signature != expected_signature {
            return Err(crate::LedgerError::InvalidTransaction(
                "Invalid transaction signature".to_string(),
            ));
        }
        
        Ok(())
    }
    
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(self).unwrap().as_bytes());
        format!("{:x}", hasher.finalize())
    }
}