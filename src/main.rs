use std::fs;

mod core;

fn main() {
    core::print_version();
    let _data: String = fs::read_to_string("./in/sales_10000.csv").expect("Unable to read file");


    core::generate_rows(&_data);

}
