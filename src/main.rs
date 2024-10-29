
use individual_2::{download_file, convert_csv_to_sql, query_iris};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            download_file(
                "https://raw.githubusercontent.com/mwaskom/seaborn-data/master/iris.csv",
                "data/iris.csv",
                "data",
            );
        }
        "transform" => match convert_csv_to_sql("data/iris.csv") {
            Ok(_) => println!("Data loaded successfully!"),
            Err(err) => eprintln!("Error: {:?}", err),
        }
        "query" => {
            if let Some(q) = args.get(2) {
                if let Err(err) = query_iris(q) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform', or 'query'.");
        }

    }
}

