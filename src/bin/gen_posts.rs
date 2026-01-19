use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

// 出力するJSONのデータ構造（main.rsのArticleと一致させる）
#[derive(Serialize, Deserialize, Debug)]
struct Article {
    id: usize,
    title: String,
    date: String,
    content: String,
}

fn main() {
    let content_dir = "content";
    let output_file = "posts.json";

    println!(">> OCCULT CODE: Article Generator Initiated...");

    // contentディレクトリの存在確認
    if !Path::new(content_dir).exists() {
        eprintln!("Error: '{}' directory not found.", content_dir);
        return;
    }

    let mut articles = Vec::new();
    // Front Matterと本文を分離する正規表現
    // --- (改行) メタデータ (改行) --- (改行) 本文
    let re = Regex::new(r"(?s)^---\r?\n(.*?)\r?\n---\r?\n(.*)$").unwrap();

    let paths = fs::read_dir(content_dir).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let raw_content = fs::read_to_string(&path).expect("Failed to read file");

            if let Some(caps) = re.captures(&raw_content) {
                let metadata_str = caps.get(1).unwrap().as_str();
                let body = caps.get(2).unwrap().as_str().trim().to_string();

                // メタデータの解析
                let mut id = 0;
                let mut title = String::new();
                let mut date = String::new();

                for line in metadata_str.lines() {
                    if let Some((key, value)) = line.split_once(':') {
                        let key = key.trim();
                        let value = value.trim();
                        match key {
                            "id" => id = value.parse().unwrap_or(0),
                            "title" => title = value.to_string(),
                            "date" => date = value.to_string(),
                            _ => {}
                        }
                    }
                }

                if id != 0 {
                    articles.push(Article {
                        id,
                        title,
                        date,
                        content: body,
                    });
                    println!("  [OK] Parsed: {}", file_name);
                } else {
                    println!("  [WARN] Skipped (Missing ID): {}", file_name);
                }
            } else {
                println!("  [WARN] Skipped (Invalid Format): {}", file_name);
            }
        }
    }

    // ID順にソート
    articles.sort_by_key(|a| a.id);

    // JSON書き出し
    let json = serde_json::to_string_pretty(&articles).unwrap();
    let mut file = fs::File::create(output_file).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!(">> GENERATION COMPLETE. {} articles written to {}.", articles.len(), output_file);
}