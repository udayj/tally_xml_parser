use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockVouchers {
    pub vouchers: Vec<StockVoucher>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockVoucher {
    pub date: String,
    pub voucher_type: String,
    pub qty_in: String,
    pub rate_in: String,
    pub amount_in: String,
    pub qty_out: String,
    pub rate_out: String,
    pub amount_out: String,
    pub closing_qty: String,
    pub closing_rate: String,
    pub closing_amount: String,
}
