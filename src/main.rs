mod datastore;
mod download_hmdb;
mod metabolites;
use clap::Parser;
use datastore::populate_datastore;
use download_hmdb::download_hmdb_files;

use std::path::Path;

use surrealdb::Datastore;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to save the hmdb data
    #[arg(short, long, default_value_t = String::from("hmdb.xml"))]
    metabolite_path: String,

    /// Path of the database file to save the hmdb data
    #[arg(short, long, default_value_t = String::from("hmdb.db"))]
    database_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let arg_database_path = Path::new(&args.metabolite_path);
    let arg_metabolite_path = Path::new(&args.metabolite_path);

    if arg_database_path.is_dir() {
        panic!("{} is a directory not a file ", arg_database_path.display());
    }

    if !arg_database_path.exists() {
        if arg_metabolite_path.is_dir() {
            panic!(
                "{} is a directory not a file ",
                arg_metabolite_path.display()
            );
        }

        if !arg_metabolite_path.exists() {
            download_hmdb_files(arg_metabolite_path).await;
        }
    }

    let datastore = Datastore::new("memory")
        .await
        .expect("Failed to create datastore");

    populate_datastore(&datastore, arg_metabolite_path);
}
