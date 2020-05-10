#[macro_use]
extern crate clap;
mod utils;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = matches.value_of("config").unwrap();
    let default_receipt_number = utils::date::get_default_date();
    let mut receipt_number = matches
        .value_of("number")
        .unwrap_or("default");
    println!("conf: {}, receipt_number: {}", path, receipt_number);
    if receipt_number == "default" {
        log::info!("No receipt number received, rrm will use {} as value", default_receipt_number);
    }
    let conf = std::fs::read_to_string(path)
        .expect(format!("File: {} not found", path).as_str());
    println!("Conf {}", conf);
}
