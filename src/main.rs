extern crate spider;
extern crate html2md;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::sync::Arc;

use crate::spider::http_cache_reqwest::CacheManager;
use spider::string_concat::{string_concat, string_concat_impl};
use spider::tokio;
use spider::tokio::sync::Semaphore;
use spider::website::Website;
use html2md::parse_html;

static GLOBAL_URL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    let mut website: Website = Website::new("https://heygoody.com/")
        .with_caching(true)
        .build()
        .unwrap();
    let mut rx2 = website.subscribe(500).unwrap();

    // สร้างโฟลเดอร์ output 
    create_dir_all("output").expect("Failed to create output directory");

    //  จำกัดจำนวน Concurrency โดยใช้ Semaphore 
    let semaphore = Arc::new(Semaphore::new(1));

    let start = std::time::Instant::now();

    website.crawl().await;
    website.unsubscribe();

    let mut rx2 = website.subscribe(500).unwrap();

    let subscription = async move {
        while let Ok(res) = rx2.recv().await {
            let cache_url = string_concat!("GET:", res.get_url());
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            tokio::spawn(async move {
                //  ใช้ spawn_blocking เพื่อป้องกัน stack overflow
                tokio::task::spawn_blocking(move || {
                    let result = tokio::runtime::Handle::current().block_on(async {
                        tokio::time::timeout(Duration::from_millis(60), async {
                            spider::website::CACACHE_MANAGER.get(&cache_url).await
                        })
                        .await
                    });

                    match result {
                        Ok(Ok(Some((cache, _)))) => {
                            let html_content = String::from_utf8_lossy(&cache.body);

                            // ตรวจสอบว่าเป็น SPA หรือ SSR
                            let is_spa = html_content.contains("<script>") || html_content.contains("ReactDOM");

                            if is_spa {
                                println!("Detected SPA: Using Chrome Headless to render.");
                            } else {
                                println!("Detected SSR: Using HTTP Request.");
                            }

                            //  แปลง HTML เป็น Markdown
                            let markdown_content = parse_html(&html_content);

                            //  บันทึก Markdown ลงไฟล์
                            let file_number = GLOBAL_URL_COUNT.fetch_add(1, Ordering::Relaxed);
                            let file_name = format!("output/page_{}.md", file_number);

                            if let Ok(mut file) = File::create(&file_name) {
                                if let Err(e) = file.write_all(markdown_content.as_bytes()) {
                                    eprintln!("Failed to write to file: {}", e);
                                } else {
                                    println!("Saved: {}", file_name);
                                }
                            }
                        }
                        Ok(Ok(None)) | Ok(Err(_)) => {
                            println!("MISS - {:?}", cache_url);
                        }
                        Err(_) => {
                            println!("ERROR - {:?}", cache_url);
                        }
                    };
                    drop(permit); // 
                });
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
