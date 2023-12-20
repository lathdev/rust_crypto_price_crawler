
fn main() {
    let currencies = ["FTM","BTC","SOL"];
    for currency in &currencies {
        get_price (currency);
    };
}

fn get_price (currency: &str) {
    let url = format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD", currency);
    let response = reqwest::blocking::get(url);
    let html_content = response.unwrap().text().unwrap();
    let json: serde_json::Value =
    serde_json::from_str(&html_content).expect("JSON was not well-formatted");
    let price = json.get("USD").unwrap();
    println!("{} - {} USD", currency, price);
}