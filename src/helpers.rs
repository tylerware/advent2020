use std::str::FromStr;

// copied from https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30
// modified
pub fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}
