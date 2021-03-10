use terminal_log_symbols::{ERROR_SYMBOL, INFO_SYMBOL, SUCCESS_SYMBOL, WARNING_SYMBOL};

fn main() {
    println!("{} Failed to download file", ERROR_SYMBOL);
    println!("{} Finished downloading file", SUCCESS_SYMBOL);
    println!("{} HDD storage space reaching limit", WARNING_SYMBOL);
    println!("{} Created new repository", INFO_SYMBOL);
}
