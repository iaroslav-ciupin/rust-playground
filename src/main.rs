mod word_crawler;
use word_crawler::*;
extern crate url;
use self::url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crawler = Crawler::new();
    let url = "https://edition.cnn.com/".to_string();
    let domain = &"cnn.com".to_string();
    let word = &"coronavirus".to_string();
    let mut result = crawler.crawl(url, domain, word, 1).await;
    println!("=======================================");
    println!("=======================================");
    println!("Word '{}' found {} times in following URLs:", word, result.word_count);
    let mut urls: Vec<String> = result.urls.into_iter().collect();
    urls.sort();
    for url in urls {
        println!("{}", url);
    }
    Ok(())
}
