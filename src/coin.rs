extern crate reqwest;
extern crate select;
extern crate scraper;
#[macro_use]
extern crate prettytable;


use select::document::Document;
use select::predicate::Name;
use scraper::{Html, Selector};
use prettytable::Table;

fn main() {
  
  get_tr("https://coinmarketcap.com/historical/20130505/"); 

}

// Get ALL <tr> tags on the page
fn get_tr(url: &str) {
  
  let mut resp = reqwest::get(url).unwrap();
  assert!(resp.status().is_success());

  let mut table = Table::new();
  
  table.add_row(row!["Name", "Symbol"]);

  // let reader = Document::from_read(resp).unwrap();
  let body = resp.text().unwrap();

  let fragment = Html::parse_document(&body);

  let currency_name = Selector::parse(".currency-name-container").unwrap();
  for currency_x in fragment.select(&currency_name) {
    let currency = currency_x.text().collect::<Vec<_>>();
    table.add_row(row![currency]);
  }

  table.printstd();

}

// #Name
// Get the currency-name-container

// #Symbol
// Get col-symbol

// #Market Cap
// Get market-cap

// #Price
// Get price. a->text

// #Circulating Supply
// Get circulating-supply

// #% 1h
// Get percent-1h

// #% 24h
// Get percent-24h
