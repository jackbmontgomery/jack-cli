use jack::storage::Storage;
use std::env;
use std::process;

fn main() {
    let mut storage = Storage::build().unwrap_or_else(|error| {
        println!("An error has occurred: {error:?}");
        process::exit(1);
    });

    let args: Vec<String> = env::args().collect();

    let data = &args[1];

    storage.write(&data).unwrap_or_else(|error| {
        println!("An error has occurred: {error:?}");
        process::exit(1);
    });
}
