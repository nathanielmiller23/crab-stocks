#![allow(unused_imports)]


use std::io;
use ureq::Response;
use serde_json::Value;
use serde_json::from_str;

//#[allow(unused_fields)]
#[derive(Debug)]
struct Stock {
   symbol: String,
   price: f32,
}

fn get_stock_info(symbol: &str) -> Option<Stock> {
    let url = format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY_ADJUSTED&symbol={}&apikey=demo", symbol);
    let response = ureq::get(&url).call();

    if let Ok(response) = response {
        let body: String = response.into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        if json.get("Error Message").is_some() {
            println!("Symbol not found");
            return None;
        }
       
       
        let meta_data = json["Meta Data"].as_object().unwrap();
        let last_refreshed = meta_data.get("3. Last Refreshed").unwrap().as_str().unwrap();
        let price = json["Time Series (Daily)"][last_refreshed]["4. close"].as_str().unwrap().parse().unwrap();

        Some(Stock { symbol: symbol.to_string(), price })
    } else {
        println!("Symbol not found");
        None
    }
}



fn main() {
    loop {
        println!("Enter a stock symbol or 'exit' to quit:");
        let mut symbol = String::new();
        io::stdin().read_line(&mut symbol).expect("Failed to read line");
        let symbol = symbol.trim();

        if symbol == "exit" {
            break;
        }

        if let Some(stock) = get_stock_info(symbol) {
            println!("{:?}", stock);
            let marketwatch_url = format!("https://www.marketwatch.com/investing/stock/{}", symbol);
            println!("MarketWatch Link: {}", marketwatch_url);
        }
    }
}
