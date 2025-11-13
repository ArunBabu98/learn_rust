# Stage 1 - Rust Foundations

• Word Counter — Read a text file and print word frequencies. Learn: std::fs, iterators,
string slices, HashMap.

• Todo CLI — Add, remove, list, and mark tasks as done. Store data in a JSON file using
serde and serde_json.

• File Organizer — Sort files in a directory by extension. Learn: walkdir, std::fs::rename,
pattern matching.

■ Milestone: You’re comfortable with ownership, Result, and Rust modules

# Stage 2 - Systems & Concurrency

• 4. Multi-threaded Downloader — Take a list of URLs and download them in parallel using tokio or reqwest + futures. Add progress bars (indicatif). Skills: Async Rust,
concurrency safety, channels.

• 5. CSV Analyzer — Like a tiny pandas. Compute mean, median, mode, and
correlations. Skills: Iterator pipelines, generics, file I/O.

• 6. HTTP REST API — CRUD API for notes or tasks using axum or warp. Skills: async, DB, modular structure, error handling
