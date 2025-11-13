/*
CSV Analyzer — Like a tiny pandas. Compute mean, median, mode, and
correlations.
*/

mod analyzer;

use analyzer::Analyzer;

fn main() {
    println!("CSV Analyzer!");
    let mut analyzer = Analyzer::new();
    analyzer.read_file("file.csv");
    analyzer.show();
    analyzer.mean();
    analyzer.median();
}
