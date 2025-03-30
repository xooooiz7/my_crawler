extern crate spider;
extern crate roxmltree; // สำหรับ parse XML sitemap

use std::time::Instant;
use std::fs::{write, OpenOptions};
use std::io::Write;
use std::path::Path;

use spider::website::Website;
use spider::features::chrome_common::RequestInterceptConfiguration;
use reqwest; // ใช้สำหรับ HTTP requests
use html2md::parse_html; // ใช้สำหรับแปลง HTML เป็น Markdown

// ฟังก์ชันสำหรับดึง Sitemap จากไฟล์ sitemap.xml
async fn fetch_sitemap(url: &str) -> Result<Vec<String>, reqwest::Error> {
    let response = reqwest::get(format!("{}/sitemap.xml", url)).await?;
    let body = response.text().await?;

    let mut urls = Vec::new();
    if body.contains("<url>") {
        let doc = roxmltree::Document::parse(&body).unwrap();
        for node in doc.descendants().filter(|n| n.has_tag_name("loc")) {
            if let Some(link) = node.text() {
                urls.push(link.to_string());
            }
        }
    }
    Ok(urls)
}

// ฟังก์ชันสำหรับตรวจสอบว่าเว็บไซต์เป็น SPA หรือ SSR
async fn check_if_spa(url: &str) -> bool {
    if let Ok(resp) = reqwest::get(url).await {
        if let Ok(body) = resp.text().await {
            if body.contains("window.__INITIAL_STATE__")
                || body.contains("ReactDOM.render")
                || body.contains("vuejs")
                || body.contains("angular")
                || body.contains("next-data")
                || body.contains("<script src=")
            {
                return true;
            }
        }
    }
    false
}

// ฟังก์ชันสำหรับแปลง HTML เป็น Markdown
fn convert_html_to_markdown(html_content: &str) -> String {
    parse_html(html_content)
}

// ฟังก์ชันสำหรับบันทึก Markdown ลงไฟล์แยก
fn save_markdown_to_file(url: &str, markdown_content: &str) {
    let file_name = url.replace("https://", "").replace("/", "_") + ".md";
    let path = Path::new("output").join(file_name);

    if let Err(e) = write(&path, markdown_content) {
        eprintln!(" Error writing file {}: {}", path.display(), e);
    } else {
        println!(" Saved: {}", path.display());
    }
}

// ฟังก์ชันสำหรับบันทึก Markdown รวมทุก URL ลงไฟล์เดียว
fn append_markdown_to_file(url: &str, markdown_content: &str) {
    let path = Path::new("output/all_pages.md");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("Unable to open file");

    if let Err(e) = writeln!(file, "\n## {}\n\n{}", url, markdown_content) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!(" Appended content to: {}", path.display());
    }
}

#[tokio::main]
async fn main() {
    let base_url = "https://heygoody.com";

    // ตรวจสอบว่าเว็บไซต์เป็น SPA หรือ SSR
    let is_spa = check_if_spa(base_url).await;
    println!("เว็บไซต์นี้เป็น: {}", if is_spa { "SPA" } else { "SSR" });

    // กำหนดการใช้งาน Chrome Intercept ถ้าเป็น SPA
    let mut website = if is_spa {
        let intercept_config = RequestInterceptConfiguration::new(true);
        Website::new(base_url)
            .with_caching(true)
            .with_chrome_intercept(intercept_config)
            .build()
            .unwrap()
    } else {
        Website::new(base_url)
            .with_caching(true)
            .build()
            .unwrap()
    };

    // ดึง URLs จาก Sitemap
    let sitemap_urls = fetch_sitemap(base_url).await.unwrap_or_else(|_| vec![]);
    println!("Found URLs from Sitemap: {:?}", sitemap_urls);

    //ทำการแปลง HTML ของแต่ละ URL เป็น Markdown และบันทึก
    for url in &sitemap_urls {
        if let Ok(resp) = reqwest::get(url).await {
            if let Ok(body) = resp.text().await {
                let markdown_content = convert_html_to_markdown(&body);

                // บันทึก Markdown เป็นไฟล์แยก
                save_markdown_to_file(url, &markdown_content);

                // บันทึก Markdown รวมทุกหน้าในไฟล์เดียว
                append_markdown_to_file(url, &markdown_content);
            }
        }
    }

    // ทำการ Crawl เว็บไซต์ 
    let start = Instant::now();
    website.crawl().await;
    let duration = start.elapsed();
    println!("Time elapsed in crawl: {:?}", duration);
}
