/*
Multi-threaded Downloader — Take a list of URLs and download
them in parallel using tokio or reqwest + futures. Add progress bars (indicatif).

*/

mod fetch;

use fetch::fetch_parallel;
use tokio;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let urls = vec![
        "https://rust-lang.org".to_string(),
        "https://docs.rs".to_string(),
        "https://crates.io".to_string(),
    ];
    match fetch_parallel(urls).await {
        Ok(_) => println!("Process completed Successfully!"),
        Err(_) => println!("Process failed!"),
    }
}
