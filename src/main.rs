extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
    hacker_news("https://news.ycombinator.com")
}

fn hacker_news(url: &str) {
  let mut resp = reqwest::get(url).unwrap();
  
  assert!(resp.status().is_success());
  
  let mut reader = Document::from_read(resp).unwrap();

  let mut hyper_links = reader.find(Name("a")).filter_map(|n| n.attr("href")).for_each(|x| println!("{}", x));

  return hyper_links;  
}
