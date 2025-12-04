use crate::types::{Parser, Result};
use roxmltree::Document;
mod types;
pub use types::*;

pub struct StockVouchersQuery {
    pub stock_item_name: String,
    pub from_date: String,
    pub to_date: String,
}

impl Parser for StockVouchersQuery {
    type Output = StockVouchers;

    fn request_xml(&self) -> String {
        include_str!("stock_vouchers_request.xml")
            .replace("{{STOCK_ITEM_NAME}}", &self.stock_item_name)
            .replace("{{FROM_DATE}}", &self.from_date)
            .replace("{{TO_DATE}}", &self.to_date)
    }

    fn parse(self, xml: &str) -> Result<Self::Output> {
        let doc = Document::parse(xml)?;
        let root = doc.root_element();
        let mut vouchers = Vec::new();

        macro_rules! text {
            ($node:expr) => {
                $node.text().unwrap_or("").to_string()
            };
        }

        let mut current_voucher: Option<StockVoucher> = None;

        for node in root.descendants() {
            match node.tag_name().name() {
                "DSPVCHDATE" => {
                    // Save previous voucher if exists
                    if let Some(v) = current_voucher.take() {
                        vouchers.push(v);
                    }
                    // Start new voucher
                    current_voucher = Some(StockVoucher {
                        date: text!(node),
                        voucher_type: String::new(),
                        qty_in: String::new(),
                        rate_in: String::new(),
                        amount_in: String::new(),
                        qty_out: String::new(),
                        rate_out: String::new(),
                        amount_out: String::new(),
                        closing_qty: String::new(),
                        closing_rate: String::new(),
                        closing_amount: String::new(),
                    });
                }
                "DSPVCHTYPE" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.voucher_type = text!(node);
                    }
                }
                "DSPVCHINQTY" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.qty_in = text!(node);
                    }
                }
                "DSPVCHINRATE" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.rate_in = text!(node);
                    }
                }
                "DSPVCHINAMT" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.amount_in = text!(node);
                    }
                }
                "DSPVCHOUTQTY" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.qty_out = text!(node);
                    }
                }
                "DSPVCHOUTRATE" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.rate_out = text!(node);
                    }
                }
                "DSPVCHNETTOUTAMT" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.amount_out = text!(node);
                    }
                }
                "DSPVCHCLQTY" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.closing_qty = text!(node);
                    }
                }
                "DSPVCHCLRATE" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.closing_rate = text!(node);
                    }
                }
                "DSPVCHCLAMT" => {
                    if let Some(v) = current_voucher.as_mut() {
                        v.closing_amount = text!(node);
                    }
                }
                _ => {}
            }
        }

        // Don't forget the last voucher
        if let Some(v) = current_voucher {
            vouchers.push(v);
        }

        // Filter to only purchases and sales (vouchers with "GST")
        vouchers.retain(|v| v.voucher_type == "Purc" || v.voucher_type.contains("GST"));

        Ok(StockVouchers { vouchers })
    }
}
