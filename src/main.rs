extern crate spider;
extern crate html2md;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use spider::website::Website;
use spider::tokio;

use reqwest;

// นับจำนวน URL ที่ประมวลผลทั้งหมด
static GLOBAL_URL_COUNT: AtomicUsize = AtomicUsize::new(0);

// เพิ่ม Copy และ Clone ให้กับ Enum Mode เพื่อให้สามารถคัดลอกค่าได้
#[derive(Copy, Clone)]
enum Mode {
    HttpRequest, // ใช้ HTTP Request ปกติ (สำหรับ SSR)
    Chrome,      // ใช้ Chrome Fetcher (สำหรับ SPA)
}

// ฟังก์ชันตรวจสอบว่าเป็น SPA หรือไม่
fn is_spa(html_content: &str) -> bool {
    html_content.contains("fetch(") || html_content.contains("XMLHttpRequest") || html_content.contains("window.__INITIAL_STATE__")
}

// ฟังก์ชันแปลง HTML เป็น Markdown
fn html_to_markdown(html: &str) -> String {
    html2md::parse_html(html)
}

// ดึงข้อมูลด้วย HTTP Request (สำหรับ SSR)
async fn fetch_with_http(url: &str) -> String {
    match reqwest::get(url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    }
}

// ดึงข้อมูลด้วย Chrome (จำลองการใช้งานผ่าน HTTP) (สำหรับ SPA)
async fn fetch_with_chrome(url: &str) -> String {
    tokio::time::sleep(Duration::from_secs(2)).await;
    fetch_with_http(url).await
}

#[tokio::main]
async fn main() {
    // กำหนดโหมดเริ่มต้น (สามารถปรับเปลี่ยนได้ตามความต้องการ)
    let default_mode = Mode::HttpRequest; // เปลี่ยนเป็น Mode::Chrome หากต้องการใช้ Chrome mode เป็นค่าเริ่มต้น

    // สร้าง Website instance โดยระบุ URL เป้าหมาย (ระบบจะดึง Sitemap จากเว็บไซต์นี้)
    let mut website: Website = Website::new("https://heygoody.com/")
        .with_caching(true)
        .build()
        .unwrap();

    // Subscribe เพื่อรับ event จากการ crawl (เช่น ลิงก์ที่พบจาก Sitemap)
    let mut rx = website.subscribe(500).unwrap();

    // สร้าง task สำหรับประมวลผลแต่ละ URL ที่ได้รับจาก event
    tokio::spawn(async move {
        while let Ok(res) = rx.recv().await {
            let url = res.get_url().to_string();
            println!("Crawled URL: {:?}", url);

            // ดึง HTML โดยใช้ HTTP Request ก่อน
            let html_content = fetch_with_http(&url).await;

            // ตรวจสอบว่าเป็น SPA หรือ SSR โดยดูจากเนื้อหา HTML ที่ได้มา
            let final_mode = if is_spa(&html_content) {
                Mode::Chrome // ถ้าเป็น SPA ให้ใช้โหมด Chrome
            } else {
                Mode::HttpRequest // ถ้าเป็น SSR ให้ใช้โหมด HTTP Request
            };

            // ดึง HTML ตามโหมดที่เลือก
            let html = match final_mode {
                Mode::HttpRequest => {
                    println!("Fetching with HTTP Request for {:?}", url);
                    html_content
                }
                Mode::Chrome => {
                    println!("Fetching with Chrome mode for {:?}", url);
                    fetch_with_chrome(&url).await
                }
            };

            // แปลง HTML ที่ดึงมาเป็น Markdown
            let markdown = html_to_markdown(&html);

            // แสดงผล Markdown ใน console
            println!("Markdown for {:?}:\n{}\n", url, markdown);

            // นับจำนวน URL ที่ประมวลผล
            GLOBAL_URL_COUNT.fetch_add(1, Ordering::Relaxed);
        }
    });

    // เริ่มกระบวนการ crawl ซึ่งจะดึงข้อมูล Sitemap และรวบรวมลิงก์ทั้งหมด
    website.crawl().await;
    website.unsubscribe();

    println!(
        "Total pages processed: {:?}",
        GLOBAL_URL_COUNT.load(Ordering::Relaxed)
    );
}
