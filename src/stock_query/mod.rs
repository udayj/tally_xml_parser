use crate::types::{Parser, Result};
use roxmltree::Document;
mod types;
pub use types::*;

pub struct StockQuery;

impl Parser for StockQuery {
    type Output = types::StockItem;

    fn request_xml(&self) -> String {
        include_str!("stock_query_request.xml").to_string()
    }

    fn parse(self, xml: &str) -> Result<Self::Output> {
        let doc = Document::parse(xml)?;
        let root = doc.root_element();
        let mut purchases = Vec::new();
        let mut sales = Vec::new();
        let mut batches = Vec::new();

        macro_rules! text {
            ($node:expr) => {
                $node.text().unwrap_or("").to_string()
            };
        }

        for node in root.descendants() {
            match node.tag_name().name() {
                // --- Purchases ---
                "STQPURCDATE" => {
                    // Create a new Purchase item and push it. This assumes the XML is well-formed
                    // and fields for a single purchase appear in order.
                    purchases.push(Purchase {
                        date: text!(node),
                        party: String::new(), // Will be filled by the next tags
                        qty: String::new(),
                        rate: String::new(),
                        amount: String::new(),
                    });
                }
                "STQPURCPARTY" => {
                    if let Some(p) = purchases.last_mut() {
                        p.party = text!(node);
                    }
                }
                "STQPURCVCHQTY" => {
                    if let Some(p) = purchases.last_mut() {
                        p.qty = text!(node);
                    }
                }
                "STQPURCVCHRATE" => {
                    if let Some(p) = purchases.last_mut() {
                        p.rate = text!(node);
                    }
                }
                "STQPURCVCHAMOUNT" => {
                    if let Some(p) = purchases.last_mut() {
                        p.amount = text!(node);
                    }
                }

                // --- Sales ---
                "STQSALESDATE" => {
                    sales.push(Sale {
                        date: text!(node),
                        party: String::new(),
                        qty: String::new(),
                        rate: String::new(),
                        amount: String::new(),
                    });
                }
                "STQSALESVCHNO" => {
                    if let Some(s) = sales.last_mut() {
                        s.party = text!(node);
                    }
                }
                "STQSALESVCHQTY" => {
                    if let Some(s) = sales.last_mut() {
                        s.qty = text!(node);
                    }
                }
                "STQSALESVCHRATE" => {
                    if let Some(s) = sales.last_mut() {
                        s.rate = text!(node);
                    }
                }
                "STQSALESVCHAMOUNT" => {
                    if let Some(s) = sales.last_mut() {
                        s.amount = text!(node);
                    }
                }

                // --- Batches ---
                "STQGODOWNNAME" => {
                    batches.push(Batch {
                        godown: text!(node),
                        batch_name: String::new(),
                        qty: String::new(),
                    });
                }
                "STQBATCHNAME" => {
                    if let Some(b) = batches.last_mut() {
                        b.batch_name = text!(node);
                    }
                }
                "STQGODOWNQTY" => {
                    if let Some(b) = batches.last_mut() {
                        b.qty = text!(node);
                    }
                }
                _ => {}
            }
        }

        // Return the final StockItem structure
        Ok(StockItem {
            purchases,
            sales,
            batches,
        })
    }
}
