use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use humansize::{format_size, BINARY};
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug)]
struct Entry {
    name: String,
    size: String,
    entry_type: String, // "DIR" または "FILE"
}

fn calculate_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

fn parse_size(size_str: &str) -> f64 {
    // サイズ文字列から数値と単位を分離
    let size_str = size_str.trim();
    let size_val: f64 = size_str.split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    
    // 単位に応じて乗数を決定
    let multiplier = match size_str.split_whitespace().nth(1) {
        Some("B") => 1.0,
        Some("KiB") => 1024.0,
        Some("MiB") => 1024.0 * 1024.0,
        Some("GiB") => 1024.0 * 1024.0 * 1024.0,
        _ => 1.0,
    };
    
    size_val * multiplier
}

fn list_current_level(dir: &str) -> Vec<Entry> {
    let mut entries = vec![];
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            if path.is_dir() {
                // ディレクトリの場合
                let size = calculate_dir_size(&path);
                entries.push(Entry {
                    name: file_name,
                    size: format_size(size, BINARY),
                    entry_type: "[DIR]".to_string(),
                });
            } else {
                // ファイルの場合
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                entries.push(Entry {
                    name: file_name,
                    size: format_size(size, BINARY),
                    entry_type: "[FILE]".to_string(),
                });
            }
        }
    }
    // サイズ順（大きい順）でソート
    entries.sort_by(|a, b| {
        let size_a = parse_size(&a.size);
        let size_b = parse_size(&b.size);
        size_b.partial_cmp(&size_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    entries
}

fn main() {
    let mut current_dir = ".".to_string();

    loop {
        println!("\nCurrent directory: {}\n", current_dir);

        // 現在の階層のファイルとディレクトリをリスト
        let entries = list_current_level(&current_dir);

        if entries.is_empty() {
            println!("No entries found in this directory.");
            break;
        }

        // 結果を表示
        for entry in &entries {
            println!("{} {} - {}", entry.entry_type, entry.name, entry.size);
        }

        // 移動先を選択または終了
        let options: Vec<String> = entries
            .iter()
            .map(|e| format!("{} {}", e.entry_type, e.name))
            .collect();
        let options_with_exit = options.clone();

        println!("\nSelect a directory to explore or enter `:q` to quit:");
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&options_with_exit)
            .default(0)
            .interact_on_opt(&dialoguer::console::Term::stderr());

        match selection {
            Ok(Some(index)) => {
                if options_with_exit[index] == ":q" {
                    println!("Exiting...");
                    break;
                }
                let selected = &entries[index];
                if selected.entry_type == "[DIR]" {
                    current_dir = Path::new(&current_dir)
                        .join(&selected.name)
                        .to_string_lossy()
                        .to_string();
                } else {
                    println!("'{}' is not a directory.", selected.name);
                }
            },
            Ok(None) | Err(_) => {
                println!("Exiting...");
                break;
            }
        }
    }
}

