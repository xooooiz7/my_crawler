extern crate spider;
extern crate html2md;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use spider::http_cache_reqwest::CacheManager;
use spider::tokio::io::AsyncWriteExt;
use spider::string_concat::{string_concat, string_concat_impl};
use spider::tokio;
use spider::website::Website;
use spider::fetcher::Fetcher;
use spider::http_cache_reqwest::HttpResponse;
use html2md::parse_html;

static GLOBAL_URL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    let mode = "chrome"; 

    let mut website: Website = Website::new("https://heygoody.com/")
        .with_caching(true)
        .discover_sitemap(true) 
        .build()
        .unwrap();
        
    if mode == "chrome" {
        website.chrome_fetcher(true); 
    } else {
        website.chrome_fetcher(false); 
    }

    let mut rx2 = website.subscribe(500).unwrap();

    let start = std::time::Instant::now();

    tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();

        while let Ok(res) = rx2.recv().await {
            let message = format!("CACHING - {:?}\n", res.get_url());
            let _ = stdout.write_all(message.as_bytes()).await;
        }
    });

    website.crawl().await;
    website.unsubscribe();

    let mut rx2 = website.subscribe(500).unwrap();

    let subscription = async move {
        while let Ok(res) = rx2.recv().await {
            let mut stdout = tokio::io::stdout();
            let cache_url = string_concat!("GET:", res.get_url());

            tokio::task::spawn(async move {
                let result = tokio::time::timeout(Duration::from_millis(6000), async {
                    spider::website::CACACHE_MANAGER.get(&cache_url).await
                })
                .await;

                match result {
                    Ok(Ok(Some((http_response, _cache_policy)))) => {
                        let body = http_response.body.clone();

                        if let Ok(html_content) = String::from_utf8(body) { 
                            // ตรวจสอบว่าเป็น SPA หรือ SSR
                            if is_spa(&html_content) {
                                let markdown = fetch_with_chrome_and_convert(&res.get_url().to_string()).await;
                                let message = format!("HIT - {:?} (SPA)\nMarkdown Content:\n{}\n", cache_url, markdown);
                                let _ = stdout.write_all(message.as_bytes()).await;
                            } else {
                                let markdown = parse_html(&html_content);
                                let message = format!("HIT - {:?} (SSR)\nMarkdown Content:\n{}\n", cache_url, markdown);
                                let _ = stdout.write_all(message.as_bytes()).await;
                            }
                        } else {
                            let message = format!("HIT - {:?} (Failed to parse HTML)\n", cache_url);
                            let _ = stdout.write_all(message.as_bytes()).await;
                        }
                    }
                    Ok(Ok(None)) | Ok(Err(_)) => {
                        let message = format!("MISS - {:?}\n", cache_url);
                        let _ = stdout.write_all(message.as_bytes()).await;
                    }
                    Err(_) => {
                        let message = format!("ERROR - {:?}\n", cache_url);
                        let _ = stdout.write_all(message.as_bytes()).await;
                    }
                };

                GLOBAL_URL_COUNT.fetch_add(1, Ordering::Relaxed);
            });
        }
    };

    let crawl = async move {
        website.crawl_raw().await;
        website.unsubscribe();
    };

    tokio::pin!(subscription);

    tokio::select! {
        _ = crawl => (),
        _ = subscription => (),
    };

    let duration = start.elapsed();

    println!(
        "Time elapsed in website.crawl() is: {:?} for total pages: {:?}",
        duration, GLOBAL_URL_COUNT
    )
}

fn is_spa(html_content: &str) -> bool {
    html_content.contains("fetch(") || html_content.contains("XMLHttpRequest") || html_content.contains("window.__INITIAL_STATE__")
}

async fn fetch_with_chrome_and_convert(url: &str) -> String {
    let fetcher = Fetcher::new_chrome();
    match fetcher.fetch(url).await {
        Ok(response) => {
            let html_content = String::from_utf8_lossy(&response.body).to_string();
            parse_html(&html_content) 
        }
        Err(_) => String::from("Failed to fetch with Chrome"),
    }
}
