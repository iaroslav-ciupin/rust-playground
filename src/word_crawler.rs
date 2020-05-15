use linkify::LinkFinder;
use std::collections::HashSet;
use std::ops::{Add, AddAssign};
use std::hash::Hash;
use url::Url;

pub struct Crawler {
    finder: LinkFinder,
}

#[derive(Debug, Default)]
pub struct CrawlResult {
    pub word_count: usize,
    pub urls: HashSet<String>,
}

impl AddAssign for CrawlResult {
    fn add_assign(&mut self, rhs: Self) {
        self.word_count += rhs.word_count;
        for url in rhs.urls {
            self.urls.insert(url);
        }
    }
}

impl Crawler {
    pub fn new() -> Crawler {
        Crawler { finder: LinkFinder::new() }
    }

    fn find_urls(&self, s: &String, domain: &String) -> HashSet<String> {
        self.finder
            .links(s.as_str())
            .filter(|l| {
                Url::parse(l.as_str())
                    .ok()
                    .and_then(|url| url.domain().map(|d|d.to_string()))
                    .map_or(false, |d| d.ends_with(domain) )
            })
            .map(|i|String::from(i.as_str()))
            .collect()
    }

    pub async fn crawl(&self, url: String, domain: &String, word: &String, _max_pages: u32) -> CrawlResult {
        let mut visited_urls = HashSet::new();
        let mut to_visit: HashSet<String> = [url].iter().cloned().collect();
        let mut total: usize = 0;
        for _ in 1..=2 {
            let result = self.crawl_urls(&to_visit, domain, word).await;
            for url in to_visit {
                visited_urls.insert(url);
            }
            total += result.word_count;
            to_visit = result.urls.into_iter()
                .filter(|u|!visited_urls.contains(u))
                .collect();
        }
        CrawlResult {
            word_count: total,
            urls: visited_urls,
        }
    }

    async fn crawl_urls(&self, urls: &HashSet<String>, domain: &String, word: &String) -> CrawlResult {
        let mut final_result = CrawlResult::default();
        let mut result_futures = vec![];
        for url in urls {
            result_futures.push(self.crawl_url(url, domain, word));
        }
        let results: Vec<Result<CrawlResult, String>> = futures::future::join_all(result_futures).await;
        for result in results {
            match result {
                Ok(crawl_result) => {
                    println!("found {} words and {} urls", crawl_result.word_count, crawl_result.urls.len());
                    final_result += crawl_result;
                },
                Err(msg) => {
                    //eprintln!("error: {}", msg)
                }
            }
        }
        final_result
    }

    async fn crawl_url(&self, url: &String, domain: &String, word: &String) -> Result<CrawlResult, String> {
        let response = reqwest::get(url.as_str()).await.map_err(|e|e.to_string())?;
        if response.status() != 200 {
            return Err(format!("status: {}", response.status()))
        }

        let html: String = response.text().await.map_err(|e|e.to_string())?;
        let word_count = num_occurrences(&html, word);
        let urls = self.find_urls(&html, domain);

        Ok(CrawlResult{ word_count, urls })
    }
}

fn num_occurrences(s: &String, word: &String) -> usize {
    let to_search = word.trim().to_lowercase();
    let searchable = s.to_lowercase();
    searchable.matches(to_search.as_str()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_occurrences() {
        let s = r#"
        <html>
            <body>
                <h1>Hello</h1>Covid-19... bla-bla how are you Covid-19
            </body>
            Covid
        </html>"#;

        assert_eq!(3, num_occurrences(&s.to_string(), &"Covid".to_string()));
        assert_eq!(2, num_occurrences(&s.to_string(), &"Covid-19".to_string()));
    }

    #[test]
    fn test_find_urls() {
        let s = r#"
        <html>
            <head>Welcome to foo.com</head>
            <body>
                <h1>www.bar.com is here</h1>Foo foo foo, com com .com
                <p>
                    http://example.com/v3
                </p>
                <p>
                    https://subdomain.example.com
                </p>
            </body>
            http://example.com
            https://aws.amazon.com/jora/valerich
            https://foo.bar/
            http://example.com/v2/jora
            Covid
        </html>"#;
        let expected: HashSet<&str>  = [
            "http://example.com/v3",
            "https://subdomain.example.com",
            "http://example.com",
            "http://example.com/v2/jora"
        ].iter().cloned().collect();
        let expected: HashSet<String> = expected.into_iter().map(String::from).collect();
        let crawler = Crawler{ finder: LinkFinder::new() };

        assert_eq!(expected, crawler.find_urls(&s.to_string(), &"example.com".to_string()));
    }
}
