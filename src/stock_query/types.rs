use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockItem {
    pub purchases: Vec<Purchase>,
    pub sales: Vec<Sale>,
    pub batches: Vec<Batch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub date: String,
    pub party: String,
    pub qty: String,
    pub rate: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub date: String,
    pub party: String,
    pub qty: String,
    pub rate: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub godown: String,
    pub batch_name: String,
    pub qty: String,
}
