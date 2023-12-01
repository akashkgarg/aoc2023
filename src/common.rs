
pub fn get_lines(filename :&str) -> Vec<String> {
    let contents = std::fs::read_to_string(filename)
        .expect("Could not read file");
    return contents.lines().map(str::to_string).collect();
}
