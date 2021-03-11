use terminal_log_symbols::{colored, ERROR_SYMBOL, INFO_SYMBOL, SUCCESS_SYMBOL, WARNING_SYMBOL};

fn main() {
    println!("{} Failed to download file", ERROR_SYMBOL);
    println!("{} Finished downloading file", SUCCESS_SYMBOL);
    println!("{} HDD storage space reaching limit", WARNING_SYMBOL);
    println!("{} Created new repository", INFO_SYMBOL);
    println!();
    println!("{} Failed to download file", colored::ERROR_SYMBOL);
    println!("{} Finished downloading file", colored::SUCCESS_SYMBOL);
    println!(
        "{} HDD storage space reaching limit",
        colored::WARNING_SYMBOL
    );
    println!("{} Created new repository", colored::INFO_SYMBOL);
}
