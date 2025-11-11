use std::fs;
use walkdir::WalkDir;

pub fn traverse_dir_sort() {
    for entry in WalkDir::new("demo").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        println!("{}", path.display());
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                println!("📄 File: {} | Extension: {}", path.display(), ext);
                let dest_folder = path.parent().unwrap().join(ext);
                if !dest_folder.exists() {
                    println!("📁 Creating folder: {}", dest_folder.display());
                    fs::create_dir_all(&dest_folder).unwrap();
                }
                let source = path.to_str().unwrap();
                let destination = dest_folder.join(path.file_name().unwrap());
                match fs::rename(source, destination) {
                    Ok(_) => println!("✅ Moved file successfully!"),
                    Err(e) => eprintln!("❌ Failed to move file: {}", e),
                }
            }
        }
    }
}
