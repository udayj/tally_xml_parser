use crate::types::Parser;
use roxmltree::Document;

pub struct StockQuery;

mod types {
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
}

use types::*;

impl Parser for StockQuery {
    type Content = types::StockItem;
    fn request_xml() -> String {
        "<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>Export</TALLYREQUEST>
        <TYPE>Data</TYPE>
        <ID>Stock Query</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
                <STOCKITEMNAME>{}</STOCKITEMNAME>
            </STATICVARIABLES>
        </DESC>
    </BODY>
</ENVELOPE>"
            .to_string()
    }

    fn parse(xml: &str) -> crate::types::Result<Self::Content> {
        let doc = Document::parse(xml)?;
        let root = doc.root_element();

        let mut purchase_dates = Vec::new();
        let mut purchase_parties = Vec::new();
        let mut purchase_qtys = Vec::new();
        let mut purchase_rates = Vec::new();
        let mut purchase_amounts = Vec::new();

        let mut sales_dates = Vec::new();
        let mut sales_parties = Vec::new();
        let mut sales_qtys = Vec::new();
        let mut sales_rates = Vec::new();
        let mut sales_amounts = Vec::new();

        let mut godown_names = Vec::new();
        let mut batch_names = Vec::new();
        let mut godown_qtys = Vec::new();

        for node in root.descendants() {
            match node.tag_name().name() {
                "STQPURCDATE" => purchase_dates.push(node.text().unwrap_or("").to_string()),
                "STQPURCPARTY" => purchase_parties.push(node.text().unwrap_or("").to_string()),
                "STQPURCVCHQTY" => purchase_qtys.push(node.text().unwrap_or("").to_string()),
                "STQPURCVCHRATE" => purchase_rates.push(node.text().unwrap_or("").to_string()),
                "STQPURCVCHAMOUNT" => purchase_amounts.push(node.text().unwrap_or("").to_string()),

                "STQSALESDATE" => sales_dates.push(node.text().unwrap_or("").to_string()),
                "STQSALESVCHNO" => sales_parties.push(node.text().unwrap_or("").to_string()),
                "STQSALESVCHQTY" => sales_qtys.push(node.text().unwrap_or("").to_string()),
                "STQSALESVCHRATE" => sales_rates.push(node.text().unwrap_or("").to_string()),
                "STQSALESVCHAMOUNT" => sales_amounts.push(node.text().unwrap_or("").to_string()),

                "STQGODOWNNAME" => godown_names.push(node.text().unwrap_or("").to_string()),
                "STQBATCHNAME" => batch_names.push(node.text().unwrap_or("").to_string()),
                "STQGODOWNQTY" => godown_qtys.push(node.text().unwrap_or("").to_string()),
                _ => {}
            }
        }

        let purchases = (0..purchase_dates.len())
            .map(|i| Purchase {
                date: purchase_dates.get(i).cloned().unwrap_or_default(),
                party: purchase_parties.get(i).cloned().unwrap_or_default(),
                qty: purchase_qtys.get(i).cloned().unwrap_or_default(),
                rate: purchase_rates.get(i).cloned().unwrap_or_default(),
                amount: purchase_amounts.get(i).cloned().unwrap_or_default(),
            })
            .collect();

        let sales = (0..sales_dates.len())
            .map(|i| Sale {
                date: sales_dates.get(i).cloned().unwrap_or_default(),
                party: sales_parties.get(i).cloned().unwrap_or_default(),
                qty: sales_qtys.get(i).cloned().unwrap_or_default(),
                rate: sales_rates.get(i).cloned().unwrap_or_default(),
                amount: sales_amounts.get(i).cloned().unwrap_or_default(),
            })
            .collect();

        let batches = (0..godown_names.len())
            .map(|i| Batch {
                godown: godown_names.get(i).cloned().unwrap_or_default(),
                batch_name: batch_names.get(i).cloned().unwrap_or_default(),
                qty: godown_qtys.get(i).cloned().unwrap_or_default(),
            })
            .collect();

        Ok(StockItem {
            purchases,
            sales,
            batches
        })
    }
}
