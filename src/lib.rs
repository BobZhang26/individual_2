use std::fs;
use reqwest::blocking::Client;
use csv::Reader;
use std::fs::OpenOptions;
use std::io::Write;
use rusqlite::{Connection, Result,params};

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

// download_file function that takes a URL, file path, and directory as arguments
pub fn download_file(url: &str, file_path: &str, directory: &str) {
    if !fs::metadata(directory).is_ok() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }
    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");

    std::io::copy(&mut response, &mut file).expect("Failed to copy content");

    println!("Extraction successful!");
}

pub fn convert_csv_to_sql(dataset: &str) -> Result<String> {
    let conn = Connection::open("IrisDataDB.db")?;

    conn.execute("DROP TABLE IF EXISTS iris", [])?;
    conn.execute(
        "CREATE TABLE iris (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sepal_length REAL,
            sepal_width REAL,
            petal_length REAL,
            petal_width REAL,
            species TEXT
        )",
        [],
    )?;

    let mut rdr = Reader::from_path(dataset).expect("Failed to read dataset");
    let mut stmt = conn.prepare(
        "INSERT INTO iris (
            sepal_length, 
            sepal_width, 
            petal_length, 
            petal_width, 
            species
        ) 
        VALUES (?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[&record[0], &record[1], &record[2], &record[3], &record[4]])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("IrisDataDB".to_string())
}

pub fn query_iris(query: &str) -> Result<()> {
    let conn = Connection::open("IrisDataDB.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;

        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i64>(0)?, // Assuming an "id" column of type INTEGER
                row.get::<usize, f64>(1)?, // sepal_length (REAL)
                row.get::<usize, f64>(2)?, // sepal_width (REAL)
                row.get::<usize, f64>(3)?, // petal_length (REAL)
                row.get::<usize, f64>(4)?, // petal_width (REAL)
                row.get::<usize, String>(5)?, // species (TEXT)
            ))
        })?;

        for result in results {
            match result {
                Ok((
                    id,
                    sepal_length,
                    sepal_width,
                    petal_length,
                    petal_width,
                    species,
                )) => {
                    println!(
                        "Result: id={}, sepal_length={}, sepal_width={}, petal_length={}, petal_width={}, species={}",
                        id,
                        sepal_length,
                        sepal_width,
                        petal_length,
                        petal_width,
                        species
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}
