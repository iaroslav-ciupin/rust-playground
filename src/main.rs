mod word_crawler;
use word_crawler::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crawler = Crawler::new();
    let url = "https://edition.cnn.com/".to_string();
    let search = &"covid".to_string();
    let mut result = crawler.crawl(url, search, 1).await;
    println!("Word: {} found {} times in following URLs:", search, result.word_count);
    let mut urls: Vec<String> = result.urls.into_iter().collect();
    urls.sort();
    for url in urls {
        println!("{}", url);
    }
    Ok(())
}
