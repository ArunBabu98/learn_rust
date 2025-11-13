use std::collections::HashMap;

use csv::Reader;
use prettytable::{Cell, Row, Table, format};

#[derive(PartialEq, Eq)]
enum ColumnType {
    Numeric,
    String,
    Mixed,
}

struct ColumnData {
    values: Vec<String>,
    col_type: ColumnType,
}

pub struct Analyzer {
    headers: Vec<String>,
    columns: HashMap<String, ColumnData>,
}

impl ColumnData {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            col_type: ColumnType::String,
        }
    }
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            columns: HashMap::new(),
        }
    }

    /// Reads a CSV file into memory.
    pub fn read_file(&mut self, path: &str) {
        match Reader::from_path(path) {
            Ok(mut rdr) => {
                // Read headers
                match rdr.headers() {
                    Ok(headers) => {
                        // self.headers = headers.iter().map(|h| h.to_string()).collect::<Vec<_>>();
                        headers.iter().for_each(|h| {
                            self.columns.insert(h.to_string(), ColumnData::new());
                            self.headers.push(h.to_string());
                        });
                    }
                    Err(err) => {
                        eprintln!("Error reading headers: {}", err);
                        return;
                    }
                }

                for result in rdr.records() {
                    match result {
                        Ok(record) => {
                            let row = record.iter().map(|f| f.to_string()).collect::<Vec<_>>();
                            for (index, row_cell) in row.iter().enumerate() {
                                if let Some(key) = self.headers.get(index) {
                                    if let Some(column_data) = self.columns.get_mut(key) {
                                        if row_cell.parse::<f64>().is_ok() {
                                            match column_data.col_type {
                                                ColumnType::String => {
                                                    column_data.col_type = ColumnType::Mixed
                                                }
                                                _ => column_data.col_type = ColumnType::Numeric,
                                            }
                                        } else if column_data.col_type == ColumnType::Numeric {
                                            column_data.col_type = ColumnType::Mixed;
                                        }

                                        column_data.values.push(row_cell.to_string());
                                    }
                                }
                            }
                        }
                        Err(err) => eprintln!("Error reading record: {}", err),
                    }
                }
            }
            Err(err) => eprintln!("Error opening CSV file '{}': {}", path, err),
        }
    }

    // Mean of numeric columns
    pub fn mean(&mut self) {
        for column in &self.columns {
            if column.1.col_type == ColumnType::Numeric {
                let col_values = column
                    .1
                    .values
                    .iter()
                    .map(|num| num.parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
                let mean = col_values.iter().sum::<f64>() / col_values.len() as f64;
                println!("Mean of Column {} is:  {}", column.0, mean)
            }
        }
    }

    // median of numeric columns
    pub fn median(&mut self) {
        for column in &self.columns {
            if column.1.col_type == ColumnType::Numeric {
                let mut col_values = column
                    .1
                    .values
                    .iter()
                    .map(|num| num.parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
                col_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let mid = col_values.len() / 2;
                if col_values.len() % 2 == 1 {
                    // odd length
                    println!("The median of Column {} is: {}", column.0, col_values[mid],);
                } else {
                    // even length → average of the two middle values
                    let median = (col_values[mid - 1] + col_values[mid]) / 2.0;
                    println!("The median of Column {} is: {}", column.0, median);
                }
            }
        }
    }

    // Show summary of the columns
    pub fn show(&self) {
        if self.columns.is_empty() {
            println!("No data loaded. Please read a CSV file first.");
            return;
        }

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        table.set_titles(Row::new(vec![
            Cell::new("Column Name"),
            Cell::new("Type"),
            Cell::new("# Values"),
            Cell::new("Examples"),
        ]));

        for header in &self.headers {
            if let Some(column_data) = self.columns.get(header) {
                let type_str = match column_data.col_type {
                    ColumnType::Numeric => "Numeric",
                    ColumnType::String => "String",
                    ColumnType::Mixed => "Mixed",
                };

                // Take first 3 values as preview examples
                let examples = column_data
                    .values
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ");

                table.add_row(Row::new(vec![
                    Cell::new(header),
                    Cell::new(type_str),
                    Cell::new(&column_data.values.len().to_string()),
                    Cell::new(&examples),
                ]));
            }
        }

        table.printstd();
    }
}
