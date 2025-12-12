# Tally XML Parser

A Rust library for parsing XML responses from Tally ERP 9, with support for stock item queries and stock voucher transactions.

## Features

- **Stock Item Queries**: Parse stock item data including purchases, sales, and batch information
- **Stock Voucher Queries**: Parse stock voucher transactions with in/out quantities, rates, and amounts
- **Type-Safe Parsing**: Strongly-typed data structures with serde support for easy serialization
- **Trait-Based Architecture**: Extensible `Parser` trait for adding custom query types
- **Error Handling**: Comprehensive error handling using `thiserror`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tally_xml_parser = "0.1.0"
```

## Usage

### Stock Item Query

```rust
use tally_xml_parser::stock_query::StockQuery;
use tally_xml_parser::types::Parser;

// Get the request XML
let query = StockQuery;
let request_xml = query.request_xml();

// Send request_xml to Tally ERP and get response
// let response_xml = send_to_tally(request_xml);

// Parse the response
let stock_item = query.parse(&response_xml)?;

// Access parsed data
for purchase in stock_item.purchases {
    println!("Purchase: {} from {} - Qty: {}, Rate: {}",
        purchase.date, purchase.party, purchase.qty, purchase.rate);
}

for sale in stock_item.sales {
    println!("Sale: {} to {} - Qty: {}, Rate: {}",
        sale.date, sale.party, sale.qty, sale.rate);
}

for batch in stock_item.batches {
    println!("Batch: {} in {} - Qty: {}",
        batch.batch_name, batch.godown, batch.qty);
}
```

### Stock Vouchers Query

```rust
use tally_xml_parser::stock_vouchers::StockVouchersQuery;
use tally_xml_parser::types::Parser;

// Create a query with parameters
let query = StockVouchersQuery {
    stock_item_name: "PRODUCT_NAME".to_string(),
    from_date: "20240101".to_string(),
    to_date: "20241231".to_string(),
};

// Get the request XML
let request_xml = query.request_xml();

// Send request_xml to Tally ERP and get response
// let response_xml = send_to_tally(request_xml);

// Parse the response
let vouchers = query.parse(&response_xml)?;

// Access parsed voucher data
for voucher in vouchers.vouchers {
    println!("Date: {}, Type: {}", voucher.date, voucher.voucher_type);
    println!("In: Qty={}, Rate={}, Amount={}",
        voucher.qty_in, voucher.rate_in, voucher.amount_in);
    println!("Out: Qty={}, Rate={}, Amount={}",
        voucher.qty_out, voucher.rate_out, voucher.amount_out);
    println!("Closing: Qty={}, Rate={}, Amount={}",
        voucher.closing_qty, voucher.closing_rate, voucher.closing_amount);
}
```

## Data Structures

### StockItem

Contains parsed stock item information:
- `purchases: Vec<Purchase>` - List of purchase transactions
- `sales: Vec<Sale>` - List of sale transactions
- `batches: Vec<Batch>` - List of batch/godown information

### StockVouchers

Contains parsed voucher transactions:
- `vouchers: Vec<StockVoucher>` - List of stock vouchers (filtered for purchases and GST-related transactions)

All structures implement `Serialize` and `Deserialize` for easy JSON conversion:

```rust
let json = serde_json::to_string_pretty(&stock_item)?;
```

## Architecture

The library uses a trait-based design centered around the `Parser` trait:

```rust
pub trait Parser {
    type Output;
    fn request_xml(&self) -> String;
    fn parse(self, xml: &str) -> Result<Self::Output>;
}
```

This allows for easy extension with custom query types while maintaining consistent error handling and API design.

## Dependencies

- `roxmltree` - Fast XML parsing
- `serde` - Serialization framework
- `thiserror` - Error handling
- `serde_json` - JSON support

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
