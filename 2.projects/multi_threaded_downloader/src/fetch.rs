use std::{cmp::min, fmt::Write, time::Duration};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use reqwest;
use tokio;

async fn fetch(url: String) -> Result<reqwest::Response, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    Ok(resp)
}

pub async fn fetch_parallel(url_list: Vec<String>) -> Result<(), reqwest::Error> {
    let mut handles = vec![];
    for url in url_list {
        let handle = tokio::spawn(fetch(url));
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(result) => {
                match result {
                    Ok(response) => {
                        if let Some(size) = response.content_length() {
                            let mut download = 0;
                            let pb = ProgressBar::new(size);

                            pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap() .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                            .progress_chars("#>-"));

                            while download < size {
                                let new = min(download + 223211, size);
                                download = new;
                                pb.set_position(new);
                                tokio::time::sleep(Duration::from_millis(12)).await;
                            }
                            pb.finish_with_message("downloaded");
                        }
                    }
                    Err(err) => println!("Error: {}", err),
                }
                println!("✅ Task finished");
            }
            Err(err) => println!("Error: {}", err),
        };
    }
    Ok(())
}
