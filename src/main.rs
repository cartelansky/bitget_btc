use reqwest;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.bitget.com/api/spot/v1/public/products";
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;

    let mut markets: Vec<String> = Vec::new();

    if let Some(data) = json["data"].as_array() {
        for item in data {
            if let (Some(base_coin), Some(quote_coin)) =
                (item["baseCoin"].as_str(), item["quoteCoin"].as_str())
            {
                if quote_coin == "BTC" {
                    markets.push(format!("BITGET:{}BTC", base_coin));
                }
            }
        }
    }

    markets.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split(":").collect();
        let b_parts: Vec<&str> = b.split(":").collect();
        let a_symbol = a_parts[1].trim_end_matches("BTC");
        let b_symbol = b_parts[1].trim_end_matches("BTC");

        if a_symbol.chars().next().unwrap().is_numeric()
            && b_symbol.chars().next().unwrap().is_numeric()
        {
            // Her iki sembol de sayı ile başlıyorsa
            let a_num: f64 = a_symbol.parse().unwrap_or(0.0);
            let b_num: f64 = b_symbol.parse().unwrap_or(0.0);
            b_num.partial_cmp(&a_num).unwrap_or(Ordering::Equal)
        } else if a_symbol.chars().next().unwrap().is_numeric() {
            // Sadece a sayı ile başlıyorsa
            Ordering::Less
        } else if b_symbol.chars().next().unwrap().is_numeric() {
            // Sadece b sayı ile başlıyorsa
            Ordering::Greater
        } else {
            // Her ikisi de alfabetik
            a_symbol.cmp(b_symbol)
        }
    });

    let mut file = File::create("bitget_btc_markets.txt")?;
    for market in markets {
        writeln!(file, "{}", market)?;
    }

    println!("Veriler başarıyla 'bitget_btc_markets.txt' dosyasına yazıldı.");

    Ok(())
}
